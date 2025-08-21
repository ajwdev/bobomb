use std::collections::HashMap;
use std::pin::Pin;
use std::string::String;
use std::sync::atomic;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use futures::future::FutureExt;
use parking_lot::Mutex;

// pub mod api {
//     tonic::include_proto!("debugger");
// }
// NOTE - I prefer the following so that Rust analyzer can pick up the references, etc. See
// https://github.com/rust-lang/rust-analyzer/issues/3767
use crate::grpc;
use crate::grpc::bobomb_debugger_server::BobombDebugger;

use crate::nes::cpu::status::Flags;
use crate::nes::executor::ExecutorContext;
use crate::nes::Nes;

// TODO(ajw) - Figure out what to do with these
const LOCK_TIMEOUT: Duration = Duration::from_secs(10);
pub const VIEWSTAMP_KEY: &'static str = "bobomb-viewstamp";

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
            return true;
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
        self.stop_from_step
            .compare_and_swap(true, false, atomic::Ordering::Relaxed)
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

// TODO Figure out what to do with these
fn new_viewstamp() -> String {
    let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    d.as_micros().to_string()
}

macro_rules! emulation_running_bail {
    ($shim:expr) => {
        if $shim.is_executing() {
            return Err(tonic::Status::new(
                tonic::Code::Aborted,
                "emulation is running",
            ));
        }
    };
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

    fn build_cpu_msg_from_nes(nes: &Nes) -> grpc::CpuState {
        let status = grpc::cpu_state::CpuStatusRegister {
            negative: nes.cpu.SR.is_set(Flags::Negative),
            overflow: nes.cpu.SR.is_set(Flags::Overflow),
            interrupt: nes.cpu.SR.is_set(Flags::Interrupt),
            zero: nes.cpu.SR.is_set(Flags::Zero),
            carry: nes.cpu.SR.is_set(Flags::Carry),
            ..Default::default()
        };
        let cpu_msg = grpc::CpuState {
            x: nes.cpu.X as u32,
            y: nes.cpu.Y as u32,
            ac: nes.cpu.AC as u32,
            program_counter: nes.cpu.PC as u32,
            stack_pointer: nes.cpu.SP as u32,
            status: Some(status),
        };

        cpu_msg
    }

    fn build_cpu_msg(&self) -> grpc::CpuState {
        let nes = self.nes.lock();
        Self::build_cpu_msg_from_nes(&nes)
    }
}

#[tonic::async_trait]
impl BobombDebugger for Server {
    async fn attach(
        &self,
        _req: tonic::Request<grpc::AttachRequest>,
    ) -> std::result::Result<tonic::Response<grpc::AttachReply>, tonic::Status> {
        self.ctx.breakpoints.lock().enable_step();

        let reply = grpc::AttachReply {
            cpu: Some(self.build_cpu_msg()),
            ..Default::default()
        };

        let mut resp = tonic::Response::new(reply);
        // TODO(ajw) Do this for everything?
        resp.metadata_mut()
            .insert(VIEWSTAMP_KEY, "TODO".parse().unwrap());

        Ok(resp)
    }

    type ResumeStream =
        Pin<Box<dyn futures::Stream<Item = Result<grpc::ResumeReply, tonic::Status>> + Send>>;

    async fn resume(
        &self,
        _req: tonic::Request<grpc::ResumeRequest>,
    ) -> std::result::Result<tonic::Response<Self::ResumeStream>, tonic::Status> {
        let fut = self.ctx.subscribe_to_stop();
        let fut_nes = self.nes.clone();
        let stream = fut
            .map(move |_| {
                // At this point we *should* be stopped. Though technically this is racy
                // because someone else could start the thread again right after we
                // had stopped. It might be better if the `subscribe_to_stop` future
                // returned the state of CPU. The viewstamps should prevent us from getting
                // bad data though.
                Ok(grpc::ResumeReply {
                    cpu: Some(Self::build_cpu_msg_from_nes(&fut_nes.lock())),
                    ..Default::default()
                })
            })
            .fuse()
            .into_stream();

        Ok(tonic::Response::new(Box::pin(stream)))
    }

    async fn restart(
        &self,
        req: tonic::Request<grpc::RestartRequest>,
    ) -> std::result::Result<tonic::Response<grpc::RestartReply>, tonic::Status> {
        let msg = req.into_inner();
        let pc = if msg.set_program_counter {
            Some(msg.program_counter as u16)
        } else {
            None
        };

        self.ctx.trigger_restart(pc);
        self.ctx.start_execution();

        Ok(tonic::Response::new(grpc::RestartReply {}))
    }

    type StepStream =
        Pin<Box<dyn futures::Stream<Item = Result<grpc::StepReply, tonic::Status>> + Send>>;

    async fn step(
        &self,
        _req: tonic::Request<grpc::StepRequest>,
    ) -> std::result::Result<tonic::Response<Self::StepStream>, tonic::Status> {
        emulation_running_bail!(self.ctx);

        let fut = self.ctx.subscribe_to_stop();
        let fut_nes = self.nes.clone();

        let stream = fut
            .map(move |_| {
                // TODO DRY this up
                Ok(grpc::StepReply {
                    cpu: Some(Self::build_cpu_msg_from_nes(&fut_nes.lock())),
                    ..Default::default()
                })
            })
            .fuse()
            .into_stream();

        self.ctx.breakpoints.lock().enable_step();
        self.ctx.start_execution();

        Ok(tonic::Response::new(Box::pin(stream)))
    }

    async fn put_breakpoint(
        &self,
        req: tonic::Request<grpc::PutBreakpointRequest>,
    ) -> std::result::Result<tonic::Response<grpc::BreakpointReply>, tonic::Status> {
        emulation_running_bail!(self.ctx);

        let msg = req.into_inner();
        let addr = msg.address as u16;
        self.ctx.breakpoints.lock().set(addr, msg.temporary);

        let reply = grpc::BreakpointReply {
            address: addr as u32,
            temporary: msg.temporary,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn delete_breakpoint(
        &self,
        req: tonic::Request<grpc::DeleteBreakpointRequest>,
    ) -> std::result::Result<tonic::Response<grpc::BreakpointReply>, tonic::Status> {
        emulation_running_bail!(self.ctx);

        let msg = req.into_inner();
        let addr = msg.address as u16;
        self.ctx.breakpoints.lock().remove(addr);

        let reply = grpc::BreakpointReply {
            address: addr as u32,
            ..Default::default()
        };

        Ok(tonic::Response::new(reply))
    }

    async fn read_memory(
        &self,
        req: tonic::Request<grpc::ReadMemoryRequest>,
    ) -> std::result::Result<tonic::Response<grpc::ReadMemoryReply>, tonic::Status> {
        emulation_running_bail!(self.ctx);

        let mut reply = grpc::ReadMemoryReply {
            ..Default::default()
        };
        let msg = req.into_inner();

        let nes = self.nes.lock();
        let interconnect = nes.interconnect.lock();
        if msg.count_by_instruction {
            let (data, start, count) =
                interconnect.read_range_by_instruction(msg.start as u16, msg.count as i16);
            reply.data = data;
            reply.start = start as u32;
            reply.count = count as u32;
        } else {
            let (data, start, count) = interconnect.read_range(msg.start as u16, msg.count as i16);
            reply.data = data;
            reply.start = start as u32;
            reply.count = count as u32;
        }

        reply.program_counter = nes.cpu.PC.into();

        Ok(tonic::Response::new(reply))
    }

    async fn read_cpu(
        &self,
        _req: tonic::Request<grpc::ReadCpuRequest>,
    ) -> std::result::Result<tonic::Response<grpc::ReadCpuReply>, tonic::Status> {
        emulation_running_bail!(self.ctx);

        let reply = grpc::ReadCpuReply {
            cpu: Some(self.build_cpu_msg()),
            ..Default::default()
        };

        Ok(tonic::Response::new(reply))
    }

    async fn status(
        &self,
        _req: tonic::Request<grpc::StatusRequest>,
    ) -> std::result::Result<tonic::Response<grpc::StatusReply>, tonic::Status> {
        if self.ctx.is_executing() {
            Ok(tonic::Response::new(grpc::StatusReply {
                emulation_state: grpc::status_reply::EmulationState::Running.into(),
                ..Default::default()
            }))
        } else {
            Ok(tonic::Response::new(grpc::StatusReply {
                emulation_state: grpc::status_reply::EmulationState::Stopped.into(),
                ..Default::default()
            }))
        }
    }
}
