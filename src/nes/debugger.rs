use parking_lot::Mutex;
use std::sync::Arc;
use std::collections::HashSet;
use std::string::String;
use bytes::Bytes;

use futures::stream;
use std::time::{Duration,SystemTime,UNIX_EPOCH};

use bobomb_grpc::protobuf;
use bobomb_grpc::grpc;
use bobomb_grpc::grpc::prelude::*;
use bobomb_grpc::api::*;
use bobomb_grpc::api_grpc::BobombDebugger;
use bobomb_grpc::VIEWSTAMP_KEY;

use crate::nes::ExecutorLock;
use crate::nes::cpu::Cpu;
use crate::nes::cpu::status::Flags;
use crate::nes::interconnect::Interconnect;

const LOCK_TIMEOUT: Duration = Duration::from_secs(10);

// NOTE You'll see a lot of code where we cast NES addresses which are normally u16 to u32. This is
// because u32 is the smallest datatype Protobufs natively support (as far as I can tell)

#[derive(Clone)]
pub struct DebuggerShim {
    lock_pair: ExecutorLock,
    cpu: Arc<Mutex<Cpu>>,
    interconnect: Arc<Mutex<Interconnect>>,
    breakpoints: Arc<Mutex<HashSet<u16>>>,
}

impl DebuggerShim {
    pub fn new(c: Arc<Mutex<Cpu>>, i: Arc<Mutex<Interconnect>>, lock_pair: ExecutorLock) -> Self {
        DebuggerShim {
            cpu: c,
            interconnect: i,
            lock_pair,

            breakpoints: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    #[inline]
    fn set_execution_lock(&self, value: bool) {
        let &(ref lock, ref cvar) = &*self.lock_pair;
        let mut running = lock.lock();
        *running = value;

        cvar.notify_all();
    }

    pub fn stop_execution(&self) {
        self.set_execution_lock(false);
    }

    pub fn start_execution(&self) {
        self.set_execution_lock(true);
    }

    pub fn is_executing(&self) -> bool {
        let &(ref lock, _) = &*self.lock_pair;
        let running = lock.lock();
        *running
    }

    pub fn block_until_running(&self) {
        let &(ref lock, ref cvar) = &*self.lock_pair;
        let mut running = lock.lock();

        if !*running { cvar.wait(&mut running); }
    }

    pub fn is_breakpoint(&mut self, addr: u16) -> bool {
        self.breakpoints.lock().contains(&addr)
    }
}

pub struct Server {
    shim: Arc<DebuggerShim>,
    viewstamp: Arc<Mutex<String>>,
}

impl Server {
    pub fn new(shim: Arc<DebuggerShim>) -> Self {
        Self {
            shim,
            viewstamp: Arc::new(Mutex::new(new_viewstamp())),
        }
    }

    fn build_cpu_msg(&self) -> CPUState {
        let c = self.shim.cpu.lock();

        let mut cpu_msg = CPUState::new();
        cpu_msg.x = c.X as u32;
        cpu_msg.y = c.Y as u32;
        cpu_msg.ac = c.AC as u32;
        cpu_msg.program_counter = c.PC as u32;
        cpu_msg.stack_pointer = c.SP as u32;

        let status = CPUState_CpuStatusRegister{
            negative: c.SR.is_set(Flags::Negative),
            overflow: c.SR.is_set(Flags::Overflow),
            interrupt: c.SR.is_set(Flags::Interrupt),
            zero: c.SR.is_set(Flags::Zero),
            carry: c.SR.is_set(Flags::Carry),
            ..Default::default()
        };
        cpu_msg.status = protobuf::SingularPtrField::some(status);
        cpu_msg
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
    }
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
        mut resp: grpc::ServerResponseUnarySink<StatusReply>) -> grpc::Result<()>
    {
        match self.viewstamp.try_lock_for(LOCK_TIMEOUT) {
            Some(viewstamp) => {
                let mut r = StatusReply::new();

                if self.shim.is_executing() {
                    r.emulation_state = StatusReply_EmulationState::RUNNING;
                } else {
                    r.emulation_state = StatusReply_EmulationState::STOPPED;
                }

                resp.send_metadata(dbg_metadata(&*viewstamp))?;
                resp.finish(r)
            },
            None => lock_error!(resp),
        }
    }

    fn stop(
        &self,
        ctx: grpc::ServerHandlerContext,
        _req: grpc::ServerRequestSingle<StopRequest>,
        mut resp: grpc::ServerResponseUnarySink<StopReply>) -> grpc::Result<()>
    {
        match self.viewstamp.try_lock_for(LOCK_TIMEOUT) {
            Some(mut viewstamp) => {
                viewstamp_bail!(ctx, &*viewstamp.as_bytes(), resp);

                self.shim.stop_execution();
                *viewstamp = new_viewstamp();

                let reply = StopReply{
                    cpu: protobuf::SingularPtrField::some(self.build_cpu_msg()),
                    ..Default::default()
                };

                resp.send_metadata(dbg_metadata(&*viewstamp))?;
                resp.finish(reply)
            },
            None => lock_error!(resp),
        }
    }

    fn resume(
        &self,
        ctx: grpc::ServerHandlerContext,
        _req: grpc::ServerRequestSingle<ResumeRequest>,
        mut resp: grpc::ServerResponseSink<ResumeReply>) -> grpc::Result<()>
    {
        match self.viewstamp.try_lock_for(LOCK_TIMEOUT) {
            Some(mut viewstamp) => {
                viewstamp_bail!(ctx, &*viewstamp.as_bytes(), resp);

                self.shim.start_execution();
                *viewstamp = new_viewstamp();

                resp.send_metadata(dbg_metadata(&*viewstamp))?;
                self.shim.block_until_running();

                let stream = stream::iter(std::iter::once(Ok(ResumeReply::new())));
                ctx.pump(stream, resp);
                Ok(())
            },
            None => lock_error!(resp),
        }
    }

    fn put_breakpoint(
        &self,
        ctx: grpc::ServerHandlerContext,
        req: grpc::ServerRequestSingle<PutBreakpointRequest>,
        mut resp: grpc::ServerResponseUnarySink<BreakpointReply>) -> grpc::Result<()>
    {
        emulation_running_bail!(self.shim, resp);

        match self.viewstamp.try_lock_for(LOCK_TIMEOUT) {
            Some(viewstamp) => {
                viewstamp_bail!(ctx, &*viewstamp.as_bytes(), resp);

                let addr = req.message.get_address() as u16;
                self.shim.breakpoints.lock().insert(addr);

                let mut reply = BreakpointReply::new();
                reply.address = addr as u32;
                // reply.temporary = req.temporary;

                resp.send_metadata(dbg_metadata(&*viewstamp))?;
                resp.finish(reply)
            },
            None => lock_error!(resp),
        }
    }

    fn delete_breakpoint(
        &self,
        ctx: grpc::ServerHandlerContext,
        req: grpc::ServerRequestSingle<DeleteBreakpointRequest>,
        mut resp: grpc::ServerResponseUnarySink<BreakpointReply>) -> grpc::Result<()>
    {
        emulation_running_bail!(self.shim, resp);

        match self.viewstamp.try_lock_for(LOCK_TIMEOUT) {
            Some(mut viewstamp) => {
                viewstamp_bail!(ctx, &*viewstamp.as_bytes(), resp);

                let addr = req.message.get_address() as u16;
                self.shim.breakpoints.lock().remove(&addr);
                *viewstamp = new_viewstamp();

                let mut reply = BreakpointReply::new();
                reply.address = addr as u32;

                resp.send_metadata(dbg_metadata(&*viewstamp))?;
                resp.finish(reply)
            },
            None => lock_error!(resp),
        }
    }

    fn read_cpu(
        &self,
        ctx: grpc::ServerHandlerContext,
        _req: grpc::ServerRequestSingle<ReadCPURequest>,
        mut resp: grpc::ServerResponseUnarySink<ReadCPUReply>) -> grpc::Result<()>
    {
        emulation_running_bail!(self.shim, resp);

        match self.viewstamp.try_lock_for(LOCK_TIMEOUT) {
            Some(viewstamp) => {
                viewstamp_bail!(ctx, &*viewstamp.as_bytes(), resp);

                let reply = ReadCPUReply{
                    cpu: protobuf::SingularPtrField::some(self.build_cpu_msg()),
                    ..Default::default()
                };

                resp.send_metadata(dbg_metadata(&*viewstamp))?;
                resp.finish(reply)
            },
            None => lock_error!(resp),
        }
    }

    fn read_memory(
        &self,
        ctx: grpc::ServerHandlerContext,
        req: grpc::ServerRequestSingle<ReadMemoryRequest>,
        mut resp: grpc::ServerResponseUnarySink<ReadMemoryReply>) -> grpc::Result<()>
    {
        emulation_running_bail!(self.shim, resp);

        match self.viewstamp.try_lock_for(LOCK_TIMEOUT) {
            Some(viewstamp) => {
                viewstamp_bail!(ctx, &*viewstamp.as_bytes(), resp);

                let mut reply = ReadMemoryReply::new();

                let interconnect = self.shim.interconnect.lock();
                if req.message.count_by_instruction {
                    let (data, start, count) = interconnect.read_range_by_instruction(
                        req.message.start as u16,
                        req.message.count as i16,
                    );
                    reply.data = data;
                    reply.start = start as u32;
                    reply.count = count as u32;
                } else {
                    let (data, start, count) = interconnect.read_range(
                        req.message.start as u16,
                        req.message.count as i16,
                    );
                    reply.data = data;
                    reply.start = start as u32;
                    reply.count = count as u32;
                }

                let c = self.shim.cpu.lock();
                reply.program_counter = c.PC.into();

                resp.send_metadata(dbg_metadata(&*viewstamp))?;
                resp.finish(reply)
            },
            None => lock_error!(resp),
        }
    }
}
