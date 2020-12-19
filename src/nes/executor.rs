use std::sync::Arc;

use anyhow::Result;
use minifb::{Key, Window, WindowOptions};
use parking_lot::{Condvar, Mutex};
use futures::channel::oneshot;

use crate::nes::Nes;
use crate::nes::debugger::Breakpoints;
use bobomb_grpc::grpc;
use bobomb_grpc::api_grpc;
use bobomb_grpc::grpc::ServerBuilder;

pub type ExecutorLock = (Mutex<bool>, Condvar);

pub struct ExecutionGate {
    gate: ExecutorLock,
}

impl ExecutionGate {
    pub fn new() -> Self {
        Self {
            gate: (Mutex::new(true), Condvar::new()),
        }
    }

    pub fn wait(&self) -> bool {
        let (ref lock, ref cvar) = self.gate;
        let mut running = lock.lock();

        if !*running {
            cvar.wait(&mut running);
            true
        } else {
            false
        }
    }

    pub fn is_executing(&self) -> bool {
        *self.gate.0.lock()
    }

    pub fn stop_execution(&self) {
        self.set_execution_lock(false);
    }

    pub fn start_execution(&self) {
        self.set_execution_lock(true);
    }

    fn set_execution_lock(&self, value: bool) {
        let (ref lock, ref cvar) = self.gate;
        let mut running = lock.lock();

        *running = value;
        cvar.notify_all();
    }
}

const WIDTH: usize = 256;
const HEIGHT: usize = 240;


pub struct ExecutorContext {
    execution_gate: Arc<ExecutionGate>,
    events: Mutex<Vec<oneshot::Sender<u16>>>,
    pub breakpoints: Mutex<Breakpoints>,
}

impl ExecutorContext {
    pub fn new(execution_gate: Arc<ExecutionGate>) -> Self {
        Self {
            execution_gate,
            breakpoints: Mutex::new(Breakpoints::new()),
            events: Mutex::new(Vec::new()),
        }
    }

    pub fn subscribe_to_stop(&self) -> oneshot::Receiver<u16> {
        let (snd, recv) = oneshot::channel::<u16>();
        self.events.lock().push(snd);
        recv
    }

    pub fn publish_stop(&self, pc: u16) {
        for s in self.events.lock().drain(0..) {
            if let Err(why) = s.send(pc) {
                eprintln!("subscription error: {:?}", why);
            }
        }
    }

    pub fn is_executing(&self) -> bool {
        self.execution_gate.is_executing()
    }

    pub fn stop_execution(&self) {
        self.execution_gate.stop_execution()
    }

    pub fn start_execution(&self) {
        self.execution_gate.start_execution()
    }
}

pub struct Executor {
    nes: Arc<Mutex<Nes>>,
    execution_gate: Arc<ExecutionGate>,
    ctx: Arc<ExecutorContext>,
    server_address: String,
    window: Window,
}

impl Executor {
    pub fn new(nes: Nes) -> Result<Self> {
        let execution_gate = Arc::new(ExecutionGate::new());
        let ctx_gate = execution_gate.clone();

        Ok(Self {
            nes: Arc::new(Mutex::new(nes)),
            execution_gate,
            ctx: Arc::new(ExecutorContext::new(ctx_gate)),
            server_address: String::from("127.0.0.1:6502"),
            window: Window::new("Bobomb", WIDTH, HEIGHT, WindowOptions{
                title: true,
                resize: false,
                scale: minifb::Scale::X2,
               ..WindowOptions::default()
            })?,
        })
    }

    fn build_debugger_server(&self) -> Result<grpc::ServerBuilder> {
        let mut bldr = ServerBuilder::new_plain();
        bldr.http.set_addr(&self.server_address)?;

        let service_def = api_grpc::BobombDebuggerServer::new_service_def(
            crate::nes::debugger::Server::new(self.nes.clone(), self.ctx.clone()),
        );
        bldr.add_service(service_def);

        Ok(bldr)
    }

    fn block_execution(&mut self, pc: u16) -> bool {
        if self.ctx.breakpoints.lock().check(pc) {
            println!("Breakpoint reached at {:#06x}", pc);
            self.ctx.stop_execution();
            // TODO Consider publishing the state of the CPU here
            self.ctx.publish_stop(pc);

            self.execution_gate.wait()
        } else {
            false
        }
    }

    pub fn run(mut self) -> Result<()> {
        let bldr = self.build_debugger_server()?;
        let _server = bldr.build()?;

        // Limit to max ~60 fps update rate
        // self.window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        let mut last_pc: u16 = 0;
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            // Has the debugger stoped us?
            if self.block_execution(last_pc) {
                println!("Waking up from stop");
            }

            // Step the nes
            let mut nes = self.nes.lock();
            let step_info = nes.step();
            if step_info.should_paint {
                self.window.update_with_buffer(
                    &nes.interconnect.lock().ppu.front,
                    WIDTH,
                    HEIGHT,
                )?;
            }

            last_pc = step_info.program_counter;
        }

        Ok(())
    }
}
