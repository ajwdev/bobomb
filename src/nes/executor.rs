use std::sync::Arc;
use std::sync::atomic::{AtomicBool,Ordering};

use anyhow::*;
use minifb::{Key, Window, WindowOptions};
use parking_lot::{Condvar, Mutex};
use futures::channel::oneshot;

use crate::nes::Nes;
use crate::nes::debugger::Breakpoints;
use bobomb_grpc::grpc;
use bobomb_grpc::api_grpc;
use bobomb_grpc::grpc::ServerBuilder;

#[derive(Debug)]
pub enum ExitStatus {
    Restart(Option<u16>),
    Success,
}

impl std::fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExitState ( {:?} )", self)
    }
}

// TODO Add error state grpc stuff to debugger
pub enum EmulationStatus {
    Running,
    Stopped,
    Crashed(String),
}

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
    // Restart fields
    pub restart: AtomicBool,
    pub restart_pc: Mutex<Option<u16>>,
}

impl ExecutorContext {
    pub fn new(execution_gate: Arc<ExecutionGate>) -> Self {
        Self {
            execution_gate,
            breakpoints: Mutex::new(Breakpoints::new()),
            events: Mutex::new(Vec::new()),
            restart: AtomicBool::new(false),
            restart_pc: Mutex::new(None),
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

    pub fn trigger_restart(&self, pc: Option<u16>) {
        self.restart.store(true, Ordering::Relaxed);
        *self.restart_pc.lock() = pc;
    }

    pub fn should_restart(&self) -> bool {
        self.restart.load(Ordering::Relaxed)
    }
}

pub struct Executor {
    nes: Arc<Mutex<Nes>>,
    execution_gate: Arc<ExecutionGate>,
    ctx: Arc<ExecutorContext>,
    server_address: String,
    wait_on_error: bool,
    window: Window,
}

impl Executor {
    pub fn new(nes: Nes, opts: &crate::Opts) -> Result<Self> {
        let execution_gate = Arc::new(ExecutionGate::new());
        let ctx_gate = execution_gate.clone();
        let ctx = Arc::new(ExecutorContext::new(ctx_gate));

        if opts.wait_for_attach {
            ctx.breakpoints.lock().enable_step();
        }

        Ok(Self {
            nes: Arc::new(Mutex::new(nes)),
            execution_gate,
            ctx,
            server_address: String::from("127.0.0.1:6502"),
            wait_on_error: true,
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
            println!("Stopped at {:#06x}", pc);
            self.ctx.stop_execution();
            // TODO Consider publishing the state of the CPU here
            self.ctx.publish_stop(pc);

            self.execution_gate.wait()
        } else {
            false
        }
    }

    // TODO Keep an eye on the Try trait in nightly. Then we
    // could easily turn the ? operators into an ExitStatus
    pub fn run(mut self) -> Result<ExitStatus> {
        let bldr = self.build_debugger_server()?;
        let _server = bldr.build()?;

        // Limit to max ~60 fps update rate
        // self.window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        let mut last_pc: u16 = self.nes.lock().cpu.PC;

        // TODO The UI blocks whenever we're stopped by the debugger. Can we start
        // the actual emulation on another thread?

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            // Has the debugger stoped us?
            if self.block_execution(last_pc) {
                println!("Waking up from stop");
            }

            // Check for restart. We do this after our potential blocking call
            if self.ctx.should_restart() {
                // Check for PC
                let pc = self.ctx.restart_pc.lock().take();
                return Ok(ExitStatus::Restart(pc))
            }

            // Step the nes
            let mut nes = self.nes.lock();
            match nes.step() {
                Ok(step_info) => {
                    if step_info.should_paint {
                        self.window.update_with_buffer(
                            &nes.interconnect.lock().ppu.front,
                            WIDTH,
                            HEIGHT,
                        )?;
                    }

                    last_pc = step_info.program_counter;
                }
                Err(why) => {
                    // Should we stop so that the debugger can do some post mortem?
                    if self.wait_on_error {
                        drop(nes); // Drop to release the lock for the debugger
                        self.ctx.stop_execution();
                        self.ctx.publish_stop(last_pc);
                        eprintln!("Error encountered, waiting for debugger... {:?}", why);
                        self.execution_gate.wait();
                    }
                    // Check for restart again. DRY this up with code above
                    if self.ctx.should_restart() {
                        // Check for PC
                        let pc = self.ctx.restart_pc.lock().take();
                        return Ok(ExitStatus::Restart(pc))
                    }

                    return Err(why);
                }
            }
        }

        Ok(ExitStatus::Success)
    }
}
