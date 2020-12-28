// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// server interface

pub trait BobombDebugger {
    fn attach(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::api::AttachRequest>, resp: ::grpc::ServerResponseUnarySink<super::api::AttachReply>) -> ::grpc::Result<()>;

    fn resume(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::api::ResumeRequest>, resp: ::grpc::ServerResponseSink<super::api::ResumeReply>) -> ::grpc::Result<()>;

    fn restart(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::api::RestartRequest>, resp: ::grpc::ServerResponseUnarySink<super::api::RestartReply>) -> ::grpc::Result<()>;

    fn step(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::api::StepRequest>, resp: ::grpc::ServerResponseSink<super::api::StepReply>) -> ::grpc::Result<()>;

    fn put_breakpoint(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::api::PutBreakpointRequest>, resp: ::grpc::ServerResponseUnarySink<super::api::BreakpointReply>) -> ::grpc::Result<()>;

    fn delete_breakpoint(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::api::DeleteBreakpointRequest>, resp: ::grpc::ServerResponseUnarySink<super::api::BreakpointReply>) -> ::grpc::Result<()>;

    fn read_memory(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::api::ReadMemoryRequest>, resp: ::grpc::ServerResponseUnarySink<super::api::ReadMemoryReply>) -> ::grpc::Result<()>;

    fn read_cpu(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::api::ReadCPURequest>, resp: ::grpc::ServerResponseUnarySink<super::api::ReadCPUReply>) -> ::grpc::Result<()>;

    fn status(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::api::StatusRequest>, resp: ::grpc::ServerResponseUnarySink<super::api::StatusReply>) -> ::grpc::Result<()>;
}

// client

pub struct BobombDebuggerClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for BobombDebuggerClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        BobombDebuggerClient {
            grpc_client: grpc_client,
        }
    }
}

impl BobombDebuggerClient {
    pub fn attach(&self, o: ::grpc::RequestOptions, req: super::api::AttachRequest) -> ::grpc::SingleResponse<super::api::AttachReply> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/Attach"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn resume(&self, o: ::grpc::RequestOptions, req: super::api::ResumeRequest) -> ::grpc::StreamingResponse<super::api::ResumeReply> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/Resume"),
            streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_server_streaming(o, req, descriptor)
    }

    pub fn restart(&self, o: ::grpc::RequestOptions, req: super::api::RestartRequest) -> ::grpc::SingleResponse<super::api::RestartReply> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/Restart"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn step(&self, o: ::grpc::RequestOptions, req: super::api::StepRequest) -> ::grpc::StreamingResponse<super::api::StepReply> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/Step"),
            streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_server_streaming(o, req, descriptor)
    }

    pub fn put_breakpoint(&self, o: ::grpc::RequestOptions, req: super::api::PutBreakpointRequest) -> ::grpc::SingleResponse<super::api::BreakpointReply> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/PutBreakpoint"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn delete_breakpoint(&self, o: ::grpc::RequestOptions, req: super::api::DeleteBreakpointRequest) -> ::grpc::SingleResponse<super::api::BreakpointReply> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/DeleteBreakpoint"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn read_memory(&self, o: ::grpc::RequestOptions, req: super::api::ReadMemoryRequest) -> ::grpc::SingleResponse<super::api::ReadMemoryReply> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/ReadMemory"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn read_cpu(&self, o: ::grpc::RequestOptions, req: super::api::ReadCPURequest) -> ::grpc::SingleResponse<super::api::ReadCPUReply> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/ReadCPU"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn status(&self, o: ::grpc::RequestOptions, req: super::api::StatusRequest) -> ::grpc::SingleResponse<super::api::StatusReply> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/Status"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct BobombDebuggerServer;


impl BobombDebuggerServer {
    pub fn new_service_def<H : BobombDebugger + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/BobombDebugger",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/Attach"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).attach(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/Resume"),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |ctx, req, resp| (*handler_copy).resume(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/Restart"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).restart(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/Step"),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |ctx, req, resp| (*handler_copy).step(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/PutBreakpoint"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).put_breakpoint(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/DeleteBreakpoint"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).delete_breakpoint(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/ReadMemory"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).read_memory(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/ReadCPU"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).read_cpu(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/BobombDebugger/Status"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).status(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}
