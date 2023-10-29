use anyhow::Result;

use bobomb::grpc;
use bobomb::grpc::bobomb_debugger_client::BobombDebuggerClient;
use futures::{FutureExt, TryFutureExt};

use crate::repl::printer;

// TODO I dont think the client needs to know about these AST nodes. Refactor
// out their usage
use bobomb::debugger::ast::{Display, Format};

pub(crate) struct ApiClient {
    client: BobombDebuggerClient<tonic::transport::Channel>,
    debug_response: bool,
}

impl ApiClient {
    pub async fn new(host: &str, port: u16) -> Result<Self> {
        let client = BobombDebuggerClient::connect(format!("https://{}:{}", host, port)).await?;

        Ok(Self {
            client,
            debug_response: false,
        })
    }

    pub fn debug_responses(&mut self, b: bool) {
        self.debug_response = b;
    }

    pub async fn do_status(&mut self) -> Result<grpc::StatusReply, tonic::Status> {
        self.client
            .status(grpc::StatusRequest {})
            .inspect(|resp| {
                if self.debug_response {
                    match resp {
                        Ok(msg) => printer::debug(format!("StatusResponse({:?})", msg)),
                        Err(why) => println!("[Debug API] error: {:?}", why),
                    }
                }
            })
            .map_ok(|x| x.into_inner())
            .await
    }

    pub async fn do_read_memory(
        &mut self,
        addr: u32,
        fmt: Format,
    ) -> Result<grpc::ReadMemoryReply, tonic::Status> {
        let count = fmt.count.unwrap_or(1);
        let start = if count < 0 {
            ((addr as i32) + count) as u32
        } else {
            addr
        };

        let req = grpc::ReadMemoryRequest {
            start,
            count: count.abs(),
            count_by_instruction: fmt.display == Some(Display::Instruction),
            ..Default::default()
        };

        self.client
            .read_memory(req)
            .inspect(|resp| {
                if self.debug_response {
                    match resp {
                        Ok(msg) => printer::debug(format!("ReadMemoryResponse({:?})", msg)),
                        Err(why) => println!("[Debug API] error: {:?}", why),
                    }
                }
            })
            .map_ok(|x| x.into_inner())
            .await
    }

    pub async fn do_read_cpu(&mut self) -> Result<grpc::ReadCpuReply, tonic::Status> {
        self.client
            .read_cpu(grpc::ReadCpuRequest {})
            .inspect(|resp| {
                if self.debug_response {
                    match resp {
                        Ok(msg) => printer::debug(format!("ReadCpuResponse({:?})", msg)),
                        Err(why) => println!("[Debug API] error: {:?}", why),
                    }
                }
            })
            .map_ok(|x| x.into_inner())
            .await
    }

    pub async fn do_continue(&mut self) -> Result<Option<grpc::ResumeReply>, tonic::Status> {
        self.client
            .resume(grpc::ResumeRequest {})
            .await?
            .into_inner()
            .message()
            .await
    }

    pub async fn do_step(&mut self) -> Result<Option<grpc::StepReply>, tonic::Status> {
        self.client
            .step(grpc::StepRequest {})
            .await?
            .into_inner()
            .message()
            .await
    }

    pub async fn do_attach(&mut self) -> Result<grpc::AttachReply, tonic::Status> {
        self.client
            .attach(grpc::AttachRequest {})
            .inspect(|resp| {
                if self.debug_response {
                    match resp {
                        Ok(msg) => printer::debug(format!("AttachResponse({:?})", msg)),
                        Err(why) => println!("[Debug API] error: {:?}", why),
                    }
                }
            })
            .map_ok(|x| x.into_inner())
            .await
    }

    pub async fn do_put_breakpoint(
        &mut self,
        address: u32,
        temporary: bool,
    ) -> Result<grpc::BreakpointReply, tonic::Status> {
        let req = grpc::PutBreakpointRequest { address, temporary };

        self.client
            .put_breakpoint(req)
            .inspect(|resp| {
                if self.debug_response {
                    match resp {
                        Ok(msg) => printer::debug(format!("PutBreakpointResponse({:?})", msg)),
                        Err(why) => println!("[Debug API] error: {:?}", why),
                    }
                }
            })
            .map_ok(|x| x.into_inner())
            .await
    }

    pub async fn do_delete_breakpoint(
        &mut self,
        address: u32,
    ) -> Result<grpc::BreakpointReply, tonic::Status> {
        let req = grpc::DeleteBreakpointRequest { address };

        self.client
            .delete_breakpoint(req)
            .inspect(|resp| {
                if self.debug_response {
                    match resp {
                        Ok(msg) => printer::debug(format!("DeleteBreakpointResponse({:?})", msg)),
                        Err(why) => println!("[Debug API] error: {:?}", why),
                    }
                }
            })
            .map_ok(|x| x.into_inner())
            .await
    }

    pub async fn do_restart(
        &mut self,
        pc: Option<u32>,
    ) -> Result<grpc::RestartReply, tonic::Status> {
        let mut req = grpc::RestartRequest {
            ..Default::default()
        };
        if let Some(n) = pc {
            req.set_program_counter = true;
            req.program_counter = n;
        }

        self.client
            .restart(req)
            .inspect(|resp| {
                if self.debug_response {
                    match resp {
                        Ok(msg) => printer::debug(format!("RestartResponse({:?})", msg)),
                        Err(why) => println!("[Debug API] error: {:?}", why),
                    }
                }
            })
            .map_ok(|x| x.into_inner())
            .await
    }
}
