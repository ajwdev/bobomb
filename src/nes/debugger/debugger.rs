use parking_lot::Mutex;
use std::sync::Arc;
use std::collections::HashSet;

use crate::nes::debugger::debugger_server::*;
use crate::nes::debugger::debugger_server_grpc::*;

use crate::nes::ExecutorLock;
use crate::nes::cpu::Cpu;
use crate::nes::interconnect::Interconnect;
use crate::nes::cpu::disassemble::Disassembler;
use crate::nes::address::Addressable;

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
            lock_pair: lock_pair,

            breakpoints: Arc::new(Mutex::new(HashSet::new())),
            // internal_lock_pair: ExecutorLock,
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

    // pub fn is_breakpoint<T: Addressable>(&self, addr: T) -> bool {
    pub fn is_breakpoint(&self, addr: u16) -> bool {
        self.breakpoints.lock().contains(&addr)
    }

}

pub struct DebuggerImpl {
    shim: Arc<DebuggerShim>,
}

impl DebuggerImpl {
    pub fn new(shim: Arc<DebuggerShim>) -> Self {
        DebuggerImpl {
            shim: shim,
        }
    }

    // pub fn update_cursor<T: Addressable>(&mut self, addr: T) {
    //     self.cursor = self.shim.cpu.lock().get_pc().into();
    // }

    fn create_disassemble_msg<T: Addressable>(&self, address: T) -> Result<DisassembleMsg, String> {
        let mem = self.shim.interconnect.lock();
        let _cpu = self.shim.cpu.lock();

        let mut msg = DisassembleMsg::new();

        let address = address.nes_address();
        let opc = mem.read_word(address);
        let byte_stream = &[
            mem.read_word(address + 1),
            mem.read_word(address + 2),
        ];

        match Disassembler::disassemble(opc, byte_stream, address) {
            Ok((result,width)) => {
                msg.set_line(result);
                msg.set_address(address as u32);
                msg.set_instruction_width(width);
                Ok(msg)
            }
            Err(e) => Err(e)
        }
    }
}

impl Debugger for DebuggerImpl {
    fn ping(
        &self,
        _ctx: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<PingRequest>,
        resp: ::grpc::ServerResponseUnarySink<OkReply>) -> ::grpc::Result<()>
    {
        let mut r = OkReply::new();
        r.set_message(format!("Pong: {}", req.message.message));
        resp.finish(r)
    }

    fn stop(
        &self,
        _ctx: ::grpc::ServerHandlerContext,
        _req: ::grpc::ServerRequestSingle<StopRequest>,
        resp: ::grpc::ServerResponseUnarySink<OkReply>) -> ::grpc::Result<()>
    {
        self.shim.stop_execution();
        let mut r = OkReply::new();
        r.set_message(format!("0x{:04X}", self.shim.cpu.lock().get_pc()));

        resp.finish(r)
    }

    fn cont(
        &self,
        _ctx: ::grpc::ServerHandlerContext,
        _req: ::grpc::ServerRequestSingle<ContinueRequest>,
        resp: ::grpc::ServerResponseUnarySink<OkReply>) -> ::grpc::Result<()>
    {
        self.shim.start_execution();
        resp.finish(OkReply::new())
    }

    fn breakpoint(
        &self,
        _ctx: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<BreakpointRequest>,
        resp: ::grpc::ServerResponseUnarySink<OkReply>) -> ::grpc::Result<()>
    {
        for a in req.message.get_addresses() {
            let tmp = *a as u16;
            match req.message.action {
                BreakpointRequest_Action::SET => {
                    self.shim.breakpoints.lock().insert(tmp);
                }
                BreakpointRequest_Action::CLEAR => {
                    self.shim.breakpoints.lock().remove(&tmp);
                }
            }
        }

        resp.finish(OkReply::new())
    }

    fn disassemble(
        &self,
        _ctx: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<DisassembleRequest>,
        resp: ::grpc::ServerResponseUnarySink<DisassembleReply>) -> ::grpc::Result<()>
    {
        let mut r = DisassembleReply::new();
        let count = if req.message.count == 0 {
            1
        } else {
            req.message.count
        };
        let mut address = req.message.address as u16;

        for _ in 0..count {
            match self.create_disassemble_msg(address) {
                Ok(msg) => {
                    address += msg.instruction_width as u16;
                    r.mut_disassembly().push(msg);
                }
                Err(why) => {
                    r.last_error = why;
                    break;
                }
            }
        }

        let len = r.mut_disassembly().len();
        r.length = len as u64;

        resp.finish(r)
    }
}
