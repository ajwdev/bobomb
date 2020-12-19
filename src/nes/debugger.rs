use std::collections::HashMap;
use std::string::String;
use std::sync::Arc;
use std::sync::atomic;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bytes::Bytes;
use parking_lot::Mutex;
use futures::future::FutureExt;

use bobomb_grpc::api::*;
use bobomb_grpc::api_grpc::BobombDebugger;
use bobomb_grpc::grpc;
use bobomb_grpc::protobuf;
use bobomb_grpc::VIEWSTAMP_KEY;

use crate::nes::Nes;
use crate::nes::cpu::status::Flags;
use crate::nes::executor::ExecutorContext;

const LOCK_TIMEOUT: Duration = Duration::from_secs(10);

// NOTE You'll see a lot of code where we cast NES addresses which are normally u16 to u32. This is
// because u32 is the smallest datatype Protobufs natively support (as far as I can tell)

pub struct Breakpoints {
    map: HashMap<u16, bool>,
    stop_from_step: atomic::AtomicBool,
}

impl Breakpoints {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            stop_from_step: atomic::AtomicBool::new(false),
        }
    }

    pub fn check(&mut self, key: u16) -> bool {
        // Check to see if we should stop for stepping
        if self.check_step() {
            return true
        }

        match self.map.get(&key) {
            Some(temp) => {
                // Found a breakpoint, is it temporary?
                if *temp {
                    self.remove(key);
                }
                true
            }
            None => false,
        }
    }

    pub fn enable_step(&self) {
        self.stop_from_step.store(true, atomic::Ordering::Relaxed);
    }

    fn check_step(&self) -> bool {
        self.stop_from_step.compare_and_swap(
            true,
            false,
            atomic::Ordering::Relaxed,
        )
    }

    pub fn get(&mut self, key: u16) -> Option<bool> {
        self.map.get(&key).map(|b| *b)
    }

    pub fn set(&mut self, key: u16, temp: bool) {
        match self.map.get_mut(&key) {
            Some(t) => {
                if *t && !temp {
                    // Change temporary breakpoint to a persistent breakpoint
                    *t = temp;
                }
            }
            None => {
                self.map.insert(key, temp);
            }
        }
    }

    pub fn remove(&mut self, key: u16) {
        self.map.remove(&key);
    }
}

pub struct Server {
    nes: Arc<Mutex<Nes>>,
    ctx: Arc<ExecutorContext>,
    viewstamp: Arc<Mutex<String>>,
}

impl Server {
    pub fn new(nes: Arc<Mutex<Nes>>, ctx: Arc<ExecutorContext>) -> Self {
        Self {
            nes,
            ctx,
            viewstamp: Arc::new(Mutex::new(new_viewstamp())),
        }
    }

    fn build_cpu_msg_from_nes(nes: &Nes) -> CPUState {
        let mut cpu_msg = CPUState::new();
        cpu_msg.x = nes.cpu.X as u32;
        cpu_msg.y = nes.cpu.Y as u32;
        cpu_msg.ac = nes.cpu.AC as u32;
        cpu_msg.program_counter = nes.cpu.PC as u32;
        cpu_msg.stack_pointer = nes.cpu.SP as u32;

        let status = CPUState_CpuStatusRegister {
            negative: nes.cpu.SR.is_set(Flags::Negative),
            overflow: nes.cpu.SR.is_set(Flags::Overflow),
            interrupt: nes.cpu.SR.is_set(Flags::Interrupt),
            zero: nes.cpu.SR.is_set(Flags::Zero),
            carry: nes.cpu.SR.is_set(Flags::Carry),
            ..Default::default()
        };
        cpu_msg.status = protobuf::SingularPtrField::some(status);
        cpu_msg
    }

    fn build_cpu_msg(&self) -> CPUState {
        let nes = self.nes.lock();
        Self::build_cpu_msg_from_nes(&nes)
    }
}

fn new_viewstamp() -> String {
    let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    d.as_micros().to_string()
}

fn dbg_metadata(viewstamp: &str) -> grpc::Metadata {
    let mut meta = grpc::Metadata::new();
    meta.add(
        grpc::MetadataKey::from(VIEWSTAMP_KEY),
        Bytes::from(viewstamp.to_string()),
    );

    meta
}

macro_rules! emulation_running_bail {
    ($shim:expr, $resp:expr) => {
        if $shim.is_executing() {
            return $resp.send_grpc_error(
                grpc::GrpcStatus::Aborted,
                String::from("emulation is running"),
            );
        }
    };
}

macro_rules! viewstamp_bail {
    ($ctx:ident, $vs:expr, $resp:expr) => {
        // Extract the viewstamp
        match $ctx.metadata.get(VIEWSTAMP_KEY) {
            Some(in_vs) => {
                if $vs != in_vs {
                    eprintln!(
                        "viewstamp mismatch: expected {}, got {}",
                        String::from_utf8_lossy($vs),
                        String::from_utf8_lossy(in_vs),
                    );
                    return $resp.send_grpc_error(
                        grpc::GrpcStatus::FailedPrecondition,
                        String::from("unable to acquire debugger lock"),
                    );
                }
            }
            None => {
                return $resp.send_grpc_error(
                    grpc::GrpcStatus::NotFound,
                    String::from("viewstamp not present in request"),
                );
            }
        }
    };
}

macro_rules! lock_error {
    ($r:expr) => {
        $r.send_grpc_error(
            grpc::GrpcStatus::Aborted,
            String::from("unable to acquire debugger lock"),
        )
    };
}

impl BobombDebugger for Server {
    fn status(
        &self,
        _ctx: grpc::ServerHandlerContext,
        _req: grpc::ServerRequestSingle<StatusRequest>,
        mut resp: grpc::ServerResponseUnarySink<StatusReply>,
    ) -> grpc::Result<()> {
        match self.viewstamp.try_lock_for(LOCK_TIMEOUT) {
            Some(viewstamp) => {
                let mut r = StatusReply::new();

                if self.ctx.is_executing() {
                    r.emulation_state = StatusReply_EmulationState::RUNNING;
                } else {
                    r.emulation_state = StatusReply_EmulationState::STOPPED;
                }

                resp.send_metadata(dbg_metadata(&*viewstamp))?;
                resp.finish(r)
            }
            None => lock_error!(resp),
        }
    }

    fn attach(
        &self,
        ctx: grpc::ServerHandlerContext,
        _req: grpc::ServerRequestSingle<AttachRequest>,
        mut resp: grpc::ServerResponseUnarySink<AttachReply>,
    ) -> grpc::Result<()> {
        match self.viewstamp.try_lock_for(LOCK_TIMEOUT) {
            Some(mut viewstamp) => {
                viewstamp_bail!(ctx, &*viewstamp.as_bytes(), resp);

                self.ctx.breakpoints.lock().enable_step();
                *viewstamp = new_viewstamp();

                let reply = AttachReply {
                    cpu: protobuf::SingularPtrField::some(self.build_cpu_msg()),
                    ..Default::default()
                };

                resp.send_metadata(dbg_metadata(&*viewstamp))?;
                resp.finish(reply)
            }
            None => lock_error!(resp),
        }
    }

    fn resume(
        &self,
        ctx: grpc::ServerHandlerContext,
        _req: grpc::ServerRequestSingle<ResumeRequest>,
        mut resp: grpc::ServerResponseSink<ResumeReply>,
    ) -> grpc::Result<()> {
        match self.viewstamp.try_lock_for(LOCK_TIMEOUT) {
            Some(mut viewstamp) => {
                viewstamp_bail!(ctx, &*viewstamp.as_bytes(), resp);

                let fut = self.ctx.subscribe_to_stop();
                let fut_nes = self.nes.clone();
                let stream = fut
                    .map(move |_| {
                        // At this point we *should* be stopped. Though technically this is racy
                        // because someone else could start the thread again right after we
                        // had stopped. It might be better if the `subscribe_to_stop` future
                        // returned the state of CPU. The viewstamps should prevent us from getting
                        // bad data though.
                        Ok(ResumeReply {
                            cpu: protobuf::SingularPtrField::some(Self::build_cpu_msg_from_nes(
                                &fut_nes.lock(),
                            )),
                            ..Default::default()
                        })
                    })
                    .fuse()
                    .into_stream();

                self.ctx.start_execution();
                *viewstamp = new_viewstamp();
                resp.send_metadata(dbg_metadata(&*viewstamp))?;

                ctx.pump(stream, resp);
                Ok(())
            }
            None => lock_error!(resp),
        }
    }

    fn step(
        &self,
        ctx: grpc::ServerHandlerContext,
        _req: grpc::ServerRequestSingle<StepRequest>,
        mut resp: grpc::ServerResponseSink<StepReply>,
    ) -> grpc::Result<()> {
        emulation_running_bail!(self.ctx, resp);

        match self.viewstamp.try_lock_for(LOCK_TIMEOUT) {
            Some(mut viewstamp) => {
                let fut = self.ctx.subscribe_to_stop();
                let fut_nes = self.nes.clone();

                let stream = fut
                    .map(move |_| {
                        // TODO DRY this up
                        Ok(StepReply {
                            cpu: protobuf::SingularPtrField::some(Self::build_cpu_msg_from_nes(
                                &fut_nes.lock(),
                            )),
                            ..Default::default()
                        })
                    })
                    .fuse()
                    .into_stream();

                self.ctx.breakpoints.lock().enable_step();
                self.ctx.start_execution();
                *viewstamp = new_viewstamp();
                resp.send_metadata(dbg_metadata(&*viewstamp))?;

                ctx.pump(stream, resp);
                Ok(())
            }
            None => lock_error!(resp),
        }
    }

    fn put_breakpoint(
        &self,
        ctx: grpc::ServerHandlerContext,
        req: grpc::ServerRequestSingle<PutBreakpointRequest>,
        mut resp: grpc::ServerResponseUnarySink<BreakpointReply>,
    ) -> grpc::Result<()> {
        emulation_running_bail!(self.ctx, resp);

        match self.viewstamp.try_lock_for(LOCK_TIMEOUT) {
            Some(viewstamp) => {
                viewstamp_bail!(ctx, &*viewstamp.as_bytes(), resp);

                let addr = req.message.address as u16;
                self.ctx.breakpoints.lock().set(addr, req.message.temporary);

                let mut reply = BreakpointReply::new();
                reply.address = addr as u32;
                reply.temporary = req.message.temporary;

                resp.send_metadata(dbg_metadata(&*viewstamp))?;
                resp.finish(reply)
            }
            None => lock_error!(resp),
        }
    }

    fn delete_breakpoint(
        &self,
        ctx: grpc::ServerHandlerContext,
        req: grpc::ServerRequestSingle<DeleteBreakpointRequest>,
        mut resp: grpc::ServerResponseUnarySink<BreakpointReply>,
    ) -> grpc::Result<()> {
        emulation_running_bail!(self.ctx, resp);

        match self.viewstamp.try_lock_for(LOCK_TIMEOUT) {
            Some(mut viewstamp) => {
                viewstamp_bail!(ctx, &*viewstamp.as_bytes(), resp);

                let addr = req.message.get_address() as u16;
                self.ctx.breakpoints.lock().remove(addr);
                *viewstamp = new_viewstamp();

                let mut reply = BreakpointReply::new();
                reply.address = addr as u32;

                resp.send_metadata(dbg_metadata(&*viewstamp))?;
                resp.finish(reply)
            }
            None => lock_error!(resp),
        }
    }

    fn read_cpu(
        &self,
        ctx: grpc::ServerHandlerContext,
        _req: grpc::ServerRequestSingle<ReadCPURequest>,
        mut resp: grpc::ServerResponseUnarySink<ReadCPUReply>,
    ) -> grpc::Result<()> {
        emulation_running_bail!(self.ctx, resp);

        match self.viewstamp.try_lock_for(LOCK_TIMEOUT) {
            Some(viewstamp) => {
                viewstamp_bail!(ctx, &*viewstamp.as_bytes(), resp);

                let reply = ReadCPUReply {
                    cpu: protobuf::SingularPtrField::some(self.build_cpu_msg()),
                    ..Default::default()
                };

                resp.send_metadata(dbg_metadata(&*viewstamp))?;
                resp.finish(reply)
            }
            None => lock_error!(resp),
        }
    }

    fn read_memory(
        &self,
        ctx: grpc::ServerHandlerContext,
        req: grpc::ServerRequestSingle<ReadMemoryRequest>,
        mut resp: grpc::ServerResponseUnarySink<ReadMemoryReply>,
    ) -> grpc::Result<()> {
        emulation_running_bail!(self.ctx, resp);

        match self.viewstamp.try_lock_for(LOCK_TIMEOUT) {
            Some(viewstamp) => {
                viewstamp_bail!(ctx, &*viewstamp.as_bytes(), resp);

                let mut reply = ReadMemoryReply::new();

                let nes = self.nes.lock();
                let interconnect = nes.interconnect.lock();
                if req.message.count_by_instruction {
                    let (data, start, count) = interconnect.read_range_by_instruction(
                        req.message.start as u16,
                        req.message.count as i16,
                    );
                    reply.data = data;
                    reply.start = start as u32;
                    reply.count = count as u32;
                } else {
                    let (data, start, count) =
                        interconnect.read_range(req.message.start as u16, req.message.count as i16);
                    reply.data = data;
                    reply.start = start as u32;
                    reply.count = count as u32;
                }

                reply.program_counter = nes.cpu.PC.into();

                resp.send_metadata(dbg_metadata(&*viewstamp))?;
                resp.finish(reply)
            }
            None => lock_error!(resp),
        }
    }
}
