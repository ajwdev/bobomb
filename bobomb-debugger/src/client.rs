use anyhow::*;
use bytes::Bytes;

use bobomb_grpc::api::*;
use bobomb_grpc::api_grpc::BobombDebuggerClient;
use bobomb_grpc::grpc;
use bobomb_grpc::protobuf;
use bobomb_grpc::grpc::prelude::*;
use bobomb_grpc::VIEWSTAMP_KEY;

// TODO I dont think the client needs to know about these AST nodes. Refactor
// out their usage
use crate::ast::{Display, Format};

pub(crate) struct ApiClient {
    viewstamp: String,
    client: BobombDebuggerClient,
    debug_response: bool,
}

impl ApiClient {
    pub fn new(host: &str, port: u16) -> Result<Self> {
        let client_conf = Default::default();
        let client = BobombDebuggerClient::new_plain(host, port, client_conf)?;

        Ok(Self {
            client,
            viewstamp: String::new(),
            debug_response: false,
        })
    }

    pub fn debug_responses(&mut self, b: bool) {
        self.debug_response = b;
    }

    fn print_debug_response<M,T>(&self, resp: &Result<(M, T, M), grpc::Error>)
        where T: protobuf::Message + std::fmt::Debug
    {
        if self.debug_response {
            match resp {
                // TODO A real logging library maybe?
                Ok((_,msg,_)) => println!("[Debug API] {:?}", msg),
                Err(why) => println!("[Debug API] error: {:?}", why),
            }
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

    //
    // API calls
    //

    pub async fn do_status(&mut self) -> Result<StatusReply, grpc::Error> {
        let resp = self
            .client
            .status(self.req_options(), StatusRequest::new())
            .join_metadata_result()
            .await;

        self.print_debug_response(&resp);
        self.map_viewstamp(resp)
    }

    pub async fn do_read_memory(
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

        self.print_debug_response(&resp);
        self.map_viewstamp(resp)
    }

    pub async fn do_read_cpu(&mut self) -> Result<ReadCPUReply, grpc::Error> {
        let req = ReadCPURequest::new();

        let resp = self
            .client
            .read_cpu(self.req_options(), req)
            .join_metadata_result()
            .await;

        self.print_debug_response(&resp);
        self.map_viewstamp(resp)
    }

    pub async fn do_continue(&mut self) -> Result<ResumeReply, grpc::Error> {
        let resp = self
            .client
            .resume(self.req_options(), ResumeRequest::new())
            .single()
            .join_metadata_result()
            .await;

        self.print_debug_response(&resp);
        self.map_viewstamp(resp)
    }

    pub async fn do_step(&mut self) -> Result<StepReply, grpc::Error> {
        let resp = self
            .client
            .step(self.req_options(), StepRequest::new())
            .single()
            .join_metadata_result()
            .await;

        self.print_debug_response(&resp);
        self.map_viewstamp(resp)
    }

    pub async fn do_attach(&mut self) -> Result<AttachReply, grpc::Error> {
        let resp = self
            .client
            .attach(self.req_options(), AttachRequest::new())
            .join_metadata_result()
            .await;

        self.print_debug_response(&resp);
        self.map_viewstamp(resp)
    }

    pub async fn do_put_breakpoint(
        &mut self,
        addr: u32,
        temporary: bool,
    ) -> Result<BreakpointReply, grpc::Error> {
        let mut req = PutBreakpointRequest::new();
        req.address = addr;
        req.temporary = temporary;

        let resp = self
            .client
            .put_breakpoint(self.req_options(), req)
            .join_metadata_result()
            .await;

        self.print_debug_response(&resp);
        self.map_viewstamp(resp)
    }

    pub async fn do_delete_breakpoint(&mut self, addr: u32) -> Result<BreakpointReply, grpc::Error> {
        let mut req = DeleteBreakpointRequest::new();
        req.address = addr;

        let resp = self
            .client
            .delete_breakpoint(self.req_options(), req)
            .join_metadata_result()
            .await;

        self.print_debug_response(&resp);
        self.map_viewstamp(resp)
    }
}
