use std::u32;
use std::collections::{HashMap,HashSet};

use anyhow::*;
use bytes::Bytes;
use futures::executor::block_on;

use rustyline::error::ReadlineError;
use rustyline::{Editor,EditMode,Config,CompletionType};

use bobomb_grpc::api::*;
use bobomb_grpc::api_grpc::BobombDebuggerClient;
use bobomb_grpc::VIEWSTAMP_KEY;
use bobomb_grpc::grpc;
use bobomb_grpc::grpc::prelude::*;

use std::fmt::Write;
use ansi_term::Color::Red;
use lalrpop_util::ParseError;

use crate::ast::*;
use crate::disassemble::Disassembly;
use crate::grammar::CommandParser;

use lazy_static::lazy_static;
lazy_static! {
    static ref PARSER: CommandParser = CommandParser::new();
    static ref RESERVED_VARIABLES: HashSet<&'static str> =
        ["$PC", "$X", "$Y", "$AC", "SP"].iter().cloned().collect();
}

const PROMPT: &'static str = "(bobomb) ";

pub struct Repl {
    client: BobombDebuggerClient,
    ctrlc_handler: CtrlCHandler,
    viewstamp: String,
    env: HashMap<String,i32>,
    print_chunk_size: usize,

    variable_counter: usize,
    last_examine: Option<usize>,
    last_print: Option<u32>,
    last_format: Format,
    display_commands: Vec<Option<(Cmd,String)>>,
}

fn print_error<E: std::fmt::Display>(why: E) {
    eprintln!("{}: {}", Red.bold().paint("Error"), why)
}

impl Repl {
    pub fn new(host: &str, port: u16) -> Result<Self> {
        let client_conf = Default::default();
        let client = BobombDebuggerClient::new_plain(host, port, client_conf)?;

        Ok(Self {
            client,
            viewstamp: String::new(),
            variable_counter: 1,
            last_examine: None,
            last_print: None,
            last_format: Format {
                display: Some(Display::Decimal),
                count: Some(1),
            },
            print_chunk_size: 8,
            env: HashMap::new(),
            display_commands: Vec::new(),
        })
    }

    pub fn run(&mut self) {
        let config = Config::builder()
            .history_ignore_space(true)
            .edit_mode(EditMode::Emacs)
            .completion_type(CompletionType::List)
            .build();

        let mut rl = Editor::<()>::with_config(config);
        if let Err(why) = rl.load_history(".bobomb_history") {
            eprintln!("Unable to load history: {}", why);
        }

        if let Err(why) = block_on(self.attach()) {
            print_error(format!("unable to attach debugger: {}", why));
        }

        loop {
            match rl.readline(PROMPT) {
                Ok(line) => {
                    if !line.trim_end().is_empty() {
                        rl.add_history_entry(line.as_str());

                        match self.parse_line(&line) {
                            Ok(ast) => {
                                if let Err(why) = block_on(self.process(ast, Some(&line))) {
                                    print_error(why)
                                }
                            }
                            Err(why) => print_error(why),
                        }
                    }

                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                }
                Err(ReadlineError::Eof) => {
                    break;
                }
                Err(why) => {
                    print_error(why);
                }
            }
        }

        rl.save_history(".bobomb_history").unwrap();
    }

    fn merge_format(&self, next: Option<Format>) -> Format {
        if next.is_none() {
            return self.last_format;
        }

        let unwrapped = next.unwrap();
        let mut fmt = self.last_format.clone();

        if unwrapped.display.is_some() {
            fmt.display = unwrapped.display;
            // We specified a display format so we must reset the count as well
            fmt.count = unwrapped.count.or(Some(1));
        } else {
            // We're not setting the format but might be displaying more items
            fmt.count = unwrapped.count.or(fmt.count);
        }

        fmt
    }

    fn print_hex(&self, start: usize, data: &Vec<u8>) {
        let mut addr = start;

        for chunk in data.chunks(self.print_chunk_size) {
            println!(
                "{:#06x}:  {}",
                addr,
                chunk
                    .iter()
                    .map(|x| format!("{:#04x}", x))
                    .collect::<Vec<String>>()
                    .join("  "),
            );
            addr += self.print_chunk_size;
        }
    }

    fn print_decimal(&self, start: usize, data: &Vec<u8>) {
        let mut addr = start;

        for chunk in data.chunks(self.print_chunk_size) {
            println!(
                "{:#06x}:  {}",
                addr,
                chunk
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("  "),
            );
            addr += self.print_chunk_size;
        }
    }

    fn print_disassembly(&self, start: usize, pc: usize, data: &Vec<u8>) -> Result<()> {
        let dis = Disassembly::disassemble(start, &mut data.iter())?;
        dis.print(pc);
        Ok(())
    }

    async fn attach(&mut self) -> Result<()> {
        let status = self.do_status().await?;

        if status.emulation_state == StatusReply_EmulationState::RUNNING {
            self.do_attach().await?;
        }

        let cpu_resp = self.do_read_cpu().await?;
        self.update_env_with_cpu(&cpu_resp.cpu.unwrap());

        Ok(())
    }

    pub fn parse_line<'input>(&self, line: &'input str) -> Result<Cmd> {
        match PARSER.parse(line) {
            Ok(ast) => Ok(ast),
            Err(why) => {
                let highlight = |start: usize, end: usize| {
                    let mut s = String::new();
                    s.push_str(&" |\n | ");
                    writeln!(s, "{}", line).unwrap();
                    s.push_str(" | ");
                    s.push_str(&" ".repeat(start));
                    s.push_str(&"^".repeat(end - start));
                    s
                };

                match why {
                    ParseError::InvalidToken { location } =>
                        Err(anyhow!("{}\n{}", why, highlight(location, location + 1))),

                    ParseError::UnrecognizedEOF { location, .. } =>
                        Err(anyhow!("{}\n{}", why, highlight(location, location + 1))),

                    ParseError::UnrecognizedToken { token: (start, _, end), .. } =>
                        Err(anyhow!("{}\n{}", why, highlight(start, end))),

                    _ => Err(anyhow!("{}", why)),
                }
            }
        }
    }

    pub async fn process(&mut self, ast: Cmd, line: Option<&str>) -> Result<()> {
        match ast {
            Cmd::Status => {
                let status = self.do_status().await?;
                println!("Status {:#?}", status);
            }

            Cmd::Display(opt_cmd) => {
                match opt_cmd {
                    Some(cmd) => {
                        match *cmd {
                            Cmd::Print(_,_) | Cmd::Examine(_,_) => {
                                self.display_commands.push(
                                    Some((*cmd, line.expect("line cannot be None").to_string()))
                                );
                                println!("[{}] {}", self.display_commands.len()-1, line.unwrap());
                            }
                            _ => bail!("Command {} cannot be used with 'display'", cmd.name()),
                        }
                    }
                    None => {
                        for (i,c) in self.display_commands.iter().enumerate() {
                            if let Some((_, cmd_str)) = c {
                                println!("[{}] {}", i, cmd_str);
                            }
                        }
                    }
                }
            }

            Cmd::Undisplay(num) => {
                if let Some(x) = self.display_commands.get_mut(num as usize) {
                    println!("Cleared display {}", num);
                    *x = None;
                }
            }

            Cmd::Examine(expr, f) => {
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
                let fmt = self.merge_format(f);

                let reply = self.do_read_memory(addr, fmt).await?;

                match fmt.display.unwrap_or(Display::Decimal) {
                    Display::Decimal => {
                        self.print_decimal(
                            reply.start as usize,
                            &reply.data,
                        )
                    }
                    Display::Hex => {
                        self.print_hex(
                            reply.start as usize,
                            &reply.data,
                        )
                    }
                    Display::Instruction => {
                        self.print_disassembly(
                            reply.start as usize,
                            reply.program_counter as usize,
                            &reply.data,
                        )?
                    }
                }

                self.last_examine = Some(reply.start as usize + reply.data.len());
                self.last_format = fmt;
            }

            Cmd::Print(expr, f) => {
                let num = match expr {
                    Some(e) => e.reduce(&self.env)? as u32,
                    None => {
                        match self.last_print {
                            Some(n) => n as u32,
                            None => bail!("No print history available"),
                        }
                    }
                };
                let fmt = self.merge_format(f);

                let var_name = format!("${}", self.variable_counter);
                self.env.set(&var_name, num as i32);

                match fmt.display.unwrap_or(Display::Decimal) {
                    Display::Decimal => println!("{} = {}", var_name, num),
                    Display::Hex => println!("{} = {:#06x}", var_name, num),
                    Display::Instruction => {
                        eprintln!("Format instruction unsupported in print command. Use Examine instead.")
                    }
                }

                self.variable_counter += 1;
                self.last_print = Some(num);
                self.last_format = fmt;
            }

            Cmd::Attach => {
                let resp = self.do_attach().await?;
                self.update_env_with_cpu(&resp.cpu.unwrap());
                self.display_on_stop().await?;
            }

            Cmd::Continue => {
                self.do_continue().await?;
            }

            Cmd::Step => {
                let resp = self.do_step().await?;
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

            Cmd::PrintVar(var) => {
                match var {
                    Some(name) => {
                        if let Some(v) = self.env.get(&name) {
                            println!("{}\t{}\t{:#06x}", name, v, v);
                        } else {
                            println!("variable {} not found", name);
                        }
                    }
                    None => {
                        for (k,v) in &self.env {
                            println!("{}\t{}\t{:#06x}", k, v, v);
                        }
                    }
                }
            }

            Cmd::Break(expr) => {
                let addr = match expr {
                    Some(e) => e.reduce(&self.env)? as u32,
                    None => {
                        match self.env.get("$PC") {
                            Some(v) => *v as u32,
                            None => bail!("program counter is unknown"),
                        }
                    }
                };
                let resp = self.do_put_breakpoint(addr, false).await?;
                println!("Breakpoint set at {:#06x}", resp.address);
            }

            Cmd::Clear(addr) => {
                let resp = self.do_delete_breakpoint(addr as u32).await?;
                println!("Breakpoint {:#06x} deleted", resp.address);
            }

            _ => panic!("unknown command"),
        }

        Ok(())
    }

    fn display_on_stop(&mut self) -> BoxFuture<Result<()>> {
        // NOTE See the Rustlang docs on recursive futures to understand why
        // we have to do this boxed future magic
        // https://rust-lang.github.io/async-book/07_workarounds/04_recursion.html
        async move {
            let cmds = &self.display_commands.iter()
                .filter_map(|x| x.clone())
                .collect::<Vec<(Cmd,String)>>();

            for (cmd, cstr) in cmds {
                println!("{}", cstr);
                self.process(cmd.clone(), None).await?;
            }

            Ok(())
        }.boxed()
    }

    fn update_env_with_cpu(&mut self, msg: &CPUState) {
        self.env.set("$PC", msg.program_counter as i32);
        self.env.set("$SP", msg.stack_pointer as i32);
        self.env.set("$X", msg.x as i32);
        self.env.set("$Y", msg.y as i32);
        self.env.set("$AC", msg.ac as i32);
        // TODO status registers
    }

    fn req_options(&self) -> grpc::RequestOptions {
        let mut meta = grpc::Metadata::new();
        meta.add(
            grpc::MetadataKey::from(VIEWSTAMP_KEY),
            Bytes::from(self.viewstamp.to_string()),
        );

        grpc::RequestOptions {
            metadata: meta,
            ..Default::default()
        }
    }

    fn map_viewstamp<T, E>(
        &mut self,
        resp: Result<(grpc::Metadata, T, grpc::Metadata), E>,
    ) -> Result<T, E> {
        resp.and_then(|reply| {
            if let Some(vs) = reply.0.get(VIEWSTAMP_KEY) {
                self.viewstamp = String::from_utf8_lossy(vs).into();
            }
            Ok(reply)
        })
        .map(|x| x.1)
    }

    async fn do_status(&mut self) -> Result<StatusReply, grpc::Error> {
        let resp = self
            .client
            .status(self.req_options(), StatusRequest::new())
            .join_metadata_result()
            .await;

        self.map_viewstamp(resp)
    }

    async fn do_read_memory(
        &mut self,
        addr: u32,
        fmt: Format,
    ) -> Result<ReadMemoryReply, grpc::Error> {
        let count = fmt.count.unwrap_or(1);
        let start = if count < 0 {
            ((addr as i32) + count) as u32
        } else {
            addr
        };

        let req = ReadMemoryRequest {
            start,
            count: count.abs(),
            count_by_instruction: fmt.display == Some(Display::Instruction),
            ..Default::default()
        };

        let resp = self
            .client
            .read_memory(self.req_options(), req)
            .join_metadata_result()
            .await;

        self.map_viewstamp(resp)
    }

    async fn do_read_cpu(&mut self) -> Result<ReadCPUReply, grpc::Error> {
        let req = ReadCPURequest::new();

        let resp = self
            .client
            .read_cpu(self.req_options(), req)
            .join_metadata_result()
            .await;

        self.map_viewstamp(resp)
    }

    async fn do_continue(&mut self) -> Result<ResumeReply, grpc::Error> {
        let resp = self
            .client
            .resume(self.req_options(), ResumeRequest::new())
            .single()
            .join_metadata_result()
            .await;

        self.map_viewstamp(resp)
    }

    async fn do_step(&mut self) -> Result<StepReply, grpc::Error> {
        let resp = self
            .client
            .step(self.req_options(), StepRequest::new())
            .single()
            .join_metadata_result()
            .await;

        self.map_viewstamp(resp)
    }

    async fn do_attach(&mut self) -> Result<AttachReply, grpc::Error> {
        let resp = self
            .client
            .attach(self.req_options(), AttachRequest::new())
            .join_metadata_result()
            .await;

        self.map_viewstamp(resp)
    }

    async fn do_put_breakpoint(&mut self, addr: u32, temporary: bool) -> Result<BreakpointReply, grpc::Error> {
        let mut req = PutBreakpointRequest::new();
        req.address = addr;
        req.temporary = temporary;

        let resp = self
            .client
            .put_breakpoint(self.req_options(), req)
            .join_metadata_result()
            .await;

        self.map_viewstamp(resp)
    }

    async fn do_delete_breakpoint(&mut self, addr: u32) -> Result<BreakpointReply, grpc::Error> {
        let mut req = DeleteBreakpointRequest::new();
        req.address = addr;

        let resp = self
            .client
            .delete_breakpoint(self.req_options(), req)
            .join_metadata_result()
            .await;

        self.map_viewstamp(resp)
    }
}
