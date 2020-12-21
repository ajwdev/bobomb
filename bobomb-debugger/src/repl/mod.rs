mod printer;

use std::collections::{HashMap, HashSet};
use std::u32;

use anyhow::*;
use futures::executor::block_on;
use futures::future::{BoxFuture, FutureExt};
use futures::select;

use rustyline::error::ReadlineError;
use rustyline;
use rustyline::{CompletionType, EditMode, Editor};

use bobomb_grpc::api::*;

use crate::ast::*;
use crate::client;
use crate::parser::PARSER;

use crate::ctrl_c::CtrlCHandler;

use lazy_static::lazy_static;
lazy_static! {
    static ref RESERVED_VARIABLES: HashSet<&'static str> =
        ["$PC", "$X", "$Y", "$AC", "SP", "Z", "C", "N", "V", "I"]
            .iter()
            .cloned()
            .collect();
}

const PROMPT: &'static str = "(bobomb) ";
const CTRLC: &'static str = "^C";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub debug_requests: bool,
}

pub struct Repl {
    client: client::ApiClient,
    ctrlc_handler: CtrlCHandler,
    env: HashMap<String, i32>,
    variable_counter: usize,
    display_commands: Vec<Option<(Cmd, String)>>,

    printer: printer::Printer<i32>,
    last_examine: Option<usize>,
    last_print: Option<i32>,
}

impl Repl {
    pub fn new(cfg: Config) -> Result<Self> {
        let ctrlc_handler = CtrlCHandler::new();
        CtrlCHandler::register(&ctrlc_handler)?;

        let mut client = client::ApiClient::new(&cfg.host, cfg.port)?;
        client.debug_responses(cfg.debug_requests);

        Ok(Self {
            client,
            ctrlc_handler,
            variable_counter: 1,
            env: HashMap::new(),
            display_commands: Vec::new(),
            printer: printer::Printer::new(),
            last_examine: None,
            last_print: None,
        })
    }

    pub fn run(&mut self) {
        let config = rustyline::Config::builder()
            .history_ignore_space(true)
            .edit_mode(EditMode::Emacs)
            .completion_type(CompletionType::List)
            .build();

        let mut rl = Editor::<()>::with_config(config);
        if let Err(why) = rl.load_history(".bobomb_history") {
            eprintln!("Unable to load history: {}", why);
        }

        if let Err(why) = block_on(self.attach()) {
            printer::error(anyhow!("unable to attach debugger: {}", why));
        }

        loop {
            match rl.readline(PROMPT) {
                Ok(line) => {
                    if !line.trim_end().is_empty() {
                        rl.add_history_entry(line.as_str());

                        match PARSER.parse(&line) {
                            Ok(ast) => {
                                if let Err(why) = block_on(self.process(ast, Some(&line))) {
                                    printer::error(why)
                                }
                            }
                            Err(why) => printer::parse_error(&line, why),
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("{}", CTRLC);
                }
                Err(ReadlineError::Eof) => {
                    break;
                }
                Err(why) => {
                    printer::error(Error::new(why));
                }
            }
        }

        rl.save_history(".bobomb_history").unwrap();
    }

    pub async fn process(&mut self, ast: Cmd, line: Option<&str>) -> Result<()> {
        match ast {
            Cmd::Status => {
                let status = self.client.do_status().await?;
                println!("Status {:#?}", status);
            }

            Cmd::Display(opt_cmd) => match opt_cmd {
                Some(cmd) => match *cmd {
                    Cmd::Print(_, _) | Cmd::Examine(_, _) => {
                        self.display_commands
                            .push(Some((*cmd, line.expect("line cannot be None").to_string())));
                        println!("[{}] {}", self.display_commands.len() - 1, line.unwrap());
                    }
                    _ => bail!("Command {} cannot be used with 'display'", cmd.name()),
                },
                None => {
                    for (i, c) in self.display_commands.iter().enumerate() {
                        if let Some((_, cmd_str)) = c {
                            println!("[{}] {}", i, cmd_str);
                        }
                    }
                }
            },

            Cmd::Undisplay(num) => {
                if let Some(x) = self.display_commands.get_mut(num as usize) {
                    println!("Cleared display {}", num);
                    *x = None;
                }
            }

            Cmd::Examine(expr, fmt) => {
                let addr = match expr {
                    Some(e) => e.reduce(&self.env)? as u32,
                    None => {
                        // Check to see if we have a previous examine address we can use
                        match self.last_examine {
                            Some(n) => n as u32,
                            None => bail!("Starting examine address required"),
                        }
                    }
                };

                let next_fmt = self.printer.update_examine_format(fmt);
                let reply = self.client.do_read_memory(addr, next_fmt).await?;

                self.printer
                    .examine(reply.start, reply.program_counter, &reply.data)?;
                self.last_examine = Some(reply.start as usize + reply.data.len());
            }

            Cmd::Print(expr, fmt) => {
                let num = match expr {
                    Some(e) => e.reduce(&self.env)?,
                    None => match self.last_print {
                        Some(n) => n,
                        None => bail!("No print history available"),
                    },
                };

                self.printer.update_print_format(fmt);

                let var_name = format!("${}", self.variable_counter);
                self.variable_counter += 1;
                self.env.set(&var_name, num);

                self.printer.print(&var_name, num)?;
                self.last_print = Some(num);
            }

            Cmd::Attach => {
                let resp = self.client.do_attach().await?;
                self.update_env_with_cpu(&resp.cpu.unwrap());
                self.display_on_stop().await?;
            }

            Cmd::Continue => {
                let sigch = self.ctrlc_handler.notify();
                let resp = select! {
                    resp = self.client.do_continue().fuse() => resp?,
                    _ = sigch.fuse() => bail!("Cancelled. Must re-attach debugger"),
                };
                self.update_env_with_cpu(&resp.cpu.unwrap());
                self.display_on_stop().await?;
            }

            Cmd::Step => {
                let resp = self.client.do_step().await?;
                self.update_env_with_cpu(&resp.cpu.unwrap());
                self.display_on_stop().await?;
            }

            Cmd::SetVar(v, e) => {
                if RESERVED_VARIABLES.contains(v.as_str()) {
                    bail!("variable {} is reserved and cannot be changed", v);
                }

                let result = e.reduce(&self.env)?;
                self.env.set(&v, result);
            }

            Cmd::PrintVar(var) => match var {
                Some(name) => {
                    if let Some(v) = self.env.get(&name) {
                        println!("{}\t{}\t{:#06x}", name, v, v);
                    } else {
                        println!("variable {} not found", name);
                    }
                }
                None => {
                    for (k, v) in &self.env {
                        println!("{}\t{}\t{:#06x}", k, v, v);
                    }
                }
            },

            Cmd::Break(expr) => {
                let addr = match expr {
                    Some(e) => e.reduce(&self.env)? as u32,
                    None => match self.env.get("$PC") {
                        Some(v) => *v as u32,
                        None => bail!("program counter is unknown"),
                    },
                };
                let resp = self.client.do_put_breakpoint(addr, false).await?;
                println!("Breakpoint set at {:#06x}", resp.address);
            }

            Cmd::Clear(addr) => {
                let resp = self.client.do_delete_breakpoint(addr as u32).await?;
                println!("Breakpoint {:#06x} deleted", resp.address);
            }

            _ => panic!("unknown command"),
        }

        Ok(())
    }

    async fn attach(&mut self) -> Result<()> {
        let status = self.client.do_status().await?;

        if status.emulation_state == StatusReply_EmulationState::RUNNING {
            self.client.do_attach().await?;
        }

        let cpu_resp = self.client.do_read_cpu().await?;
        self.update_env_with_cpu(&cpu_resp.cpu.unwrap());

        Ok(())
    }

    fn display_on_stop(&mut self) -> BoxFuture<Result<()>> {
        // NOTE See the Rustlang docs on recursive futures to understand why
        // we have to do this boxed future magic
        // https://rust-lang.github.io/async-book/07_workarounds/04_recursion.html
        async move {
            let cmds = &self
                .display_commands
                .iter()
                .filter_map(|x| x.clone())
                .collect::<Vec<(Cmd, String)>>();

            for (cmd, cstr) in cmds {
                println!("{}", cstr);
                self.process(cmd.clone(), None).await?;
            }

            Ok(())
        }
        .boxed()
    }

    fn update_env_with_cpu(&mut self, msg: &CPUState) {
        self.env.set("$PC", msg.program_counter as i32);
        self.env.set("$SP", msg.stack_pointer as i32);
        self.env.set("$X", msg.x as i32);
        self.env.set("$Y", msg.y as i32);
        self.env.set("$AC", msg.ac as i32);
        let st = msg.status.clone().unwrap();
        self.env.set("$C", st.carry as i32);
        self.env.set("$Z", st.zero as i32);
        self.env.set("$I", st.interrupt as i32);
        self.env.set("$V", st.overflow as i32);
        self.env.set("$N", st.negative as i32);
    }
}
