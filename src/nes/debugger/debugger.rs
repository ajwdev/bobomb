use parking_lot::{Mutex,Condvar};
use std::sync::Arc;
use std::collections::HashSet;

extern crate grpc;
use grpc::result::GrpcResult;
use grpc::error::GrpcError;

use nes::debugger::debugger_server::*;
use nes::debugger::debugger_server_grpc::*;

use nes::ExecutorLock;
use nes::cpu::Cpu;
use nes::interconnect::Interconnect;
use nes::cpu::disassemble::Disassembler;
use nes::address::Addressable;

// NOTE You'll see a lot of code where we cast NES addresses which are normally u16 to u32. This is
// because u32 is the smallest datatype Protobufs natively support (as far as I can tell)

pub struct DebuggerImpl {
    attached: bool,
    cursor: u16,
    lock_pair: ExecutorLock,
    cpu: Arc<Mutex<Cpu>>,
    interconnect: Arc<Mutex<Interconnect>>,
}

impl DebuggerImpl {
    pub fn new(c: Arc<Mutex<Cpu>>, i: Arc<Mutex<Interconnect>>, lock_pair: ExecutorLock) -> Self {
        let attached;
        {
            let &(ref lock, ref cvar) = &*lock_pair;
            attached = *lock.lock().unwrap();
        }

        DebuggerImpl {
            attached: attached,
            cursor: 0,
            lock_pair: lock_pair,
            cpu: c,
            interconnect: i,
        }
    }

    pub fn update_cursor<T: Addressable>(&mut self, addr: T) {
        self.cursor = self.cpu.lock().unwrap()
            .get_pc().into();
    }

    pub fn is_attached(&self) -> bool {
        self.attached
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

    fn create_disassemble_msg<T: Addressable>(&self, address: T) -> Result<DisassembleMsg, String> {
        let mem = self.shim.interconnect.lock();
        let cpu = self.shim.cpu.lock();

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
    fn Ping(&self, req: PingRequest) -> GrpcResult<OkReply> {
        let mut r = OkReply::new();
        r.set_message(format!("Pong: {}", req.message));
        Ok(r)
    }

    fn Stop(&self, req: StopRequest) -> GrpcResult<OkReply> {
        self.stop_execution();
        let mut r = OkReply::new();
        r.set_message(format!("0x{:04X}", self.cpu.lock().unwrap().get_pc()));

        Ok(r)
    }

    fn Continue(&self, req: ContinueRequest) -> GrpcResult<OkReply> {
        self.start_execution();
        Ok(OkReply::new())
    }

    fn Disassemble(&self, req: DisassembleRequest) -> GrpcResult<DisassembleReply> {
        let mut r = DisassembleReply::new();
        let count = if req.count == 0 {
            1
        } else {
            req.count
        };
        let mut address = req.address as u16;

        for _ in 0..count {
            match self.create_disassemble_msg(address) {
                Ok(msg) => {
                    address += (msg.instruction_width as u16);
                    r.mut_disassembly().push(msg);
                }
                Err(msg) => {
                    r.last_error = msg;
                    break;
                }
            }
        }

        let len = r.mut_disassembly().len();
        r.length = len as u64;

        Ok(r)
    }
}
