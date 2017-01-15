// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait Debugger {
    fn Ping(&self, p: super::debugger_server::PingRequest) -> ::grpc::result::GrpcResult<super::debugger_server::OkReply>;

    fn Stop(&self, p: super::debugger_server::StopRequest) -> ::grpc::result::GrpcResult<super::debugger_server::OkReply>;

    fn Continue(&self, p: super::debugger_server::ContinueRequest) -> ::grpc::result::GrpcResult<super::debugger_server::OkReply>;

    fn Disassemble(&self, p: super::debugger_server::DisassembleRequest) -> ::grpc::result::GrpcResult<super::debugger_server::DisassembleReply>;
}

pub trait DebuggerAsync {
    fn Ping(&self, p: super::debugger_server::PingRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::debugger_server::OkReply>;

    fn Stop(&self, p: super::debugger_server::StopRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::debugger_server::OkReply>;

    fn Continue(&self, p: super::debugger_server::ContinueRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::debugger_server::OkReply>;

    fn Disassemble(&self, p: super::debugger_server::DisassembleRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::debugger_server::DisassembleReply>;
}

// sync client

pub struct DebuggerClient {
    async_client: DebuggerAsyncClient,
}

impl DebuggerClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        DebuggerAsyncClient::new(host, port, tls).map(|c| {
            DebuggerClient {
                async_client: c,
            }
        })
    }
}

impl Debugger for DebuggerClient {
    fn Ping(&self, p: super::debugger_server::PingRequest) -> ::grpc::result::GrpcResult<super::debugger_server::OkReply> {
        ::futures::Future::wait(self.async_client.Ping(p))
    }

    fn Stop(&self, p: super::debugger_server::StopRequest) -> ::grpc::result::GrpcResult<super::debugger_server::OkReply> {
        ::futures::Future::wait(self.async_client.Stop(p))
    }

    fn Continue(&self, p: super::debugger_server::ContinueRequest) -> ::grpc::result::GrpcResult<super::debugger_server::OkReply> {
        ::futures::Future::wait(self.async_client.Continue(p))
    }

    fn Disassemble(&self, p: super::debugger_server::DisassembleRequest) -> ::grpc::result::GrpcResult<super::debugger_server::DisassembleReply> {
        ::futures::Future::wait(self.async_client.Disassemble(p))
    }
}

// async client

pub struct DebuggerAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_Ping: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::debugger_server::PingRequest, super::debugger_server::OkReply>>,
    method_Stop: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::debugger_server::StopRequest, super::debugger_server::OkReply>>,
    method_Continue: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::debugger_server::ContinueRequest, super::debugger_server::OkReply>>,
    method_Disassemble: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::debugger_server::DisassembleRequest, super::debugger_server::DisassembleReply>>,
}

impl DebuggerAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            DebuggerAsyncClient {
                grpc_client: c,
                method_Ping: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Debugger/Ping".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Stop: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Debugger/Stop".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Continue: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Debugger/Continue".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Disassemble: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Debugger/Disassemble".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl DebuggerAsync for DebuggerAsyncClient {
    fn Ping(&self, p: super::debugger_server::PingRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::debugger_server::OkReply> {
        self.grpc_client.call_unary(p, self.method_Ping.clone())
    }

    fn Stop(&self, p: super::debugger_server::StopRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::debugger_server::OkReply> {
        self.grpc_client.call_unary(p, self.method_Stop.clone())
    }

    fn Continue(&self, p: super::debugger_server::ContinueRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::debugger_server::OkReply> {
        self.grpc_client.call_unary(p, self.method_Continue.clone())
    }

    fn Disassemble(&self, p: super::debugger_server::DisassembleRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::debugger_server::DisassembleReply> {
        self.grpc_client.call_unary(p, self.method_Disassemble.clone())
    }
}

// sync server

pub struct DebuggerServer {
    async_server: DebuggerAsyncServer,
}

struct DebuggerServerHandlerToAsync {
    handler: ::std::sync::Arc<Debugger + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl DebuggerAsync for DebuggerServerHandlerToAsync {
    fn Ping(&self, p: super::debugger_server::PingRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::debugger_server::OkReply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Ping(p)
        })
    }

    fn Stop(&self, p: super::debugger_server::StopRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::debugger_server::OkReply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Stop(p)
        })
    }

    fn Continue(&self, p: super::debugger_server::ContinueRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::debugger_server::OkReply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Continue(p)
        })
    }

    fn Disassemble(&self, p: super::debugger_server::DisassembleRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::debugger_server::DisassembleReply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Disassemble(p)
        })
    }
}

impl DebuggerServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : Debugger + Send + Sync + 'static>(addr: A, h: H) -> Self {
        let h = DebuggerServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        DebuggerServer {
            async_server: DebuggerAsyncServer::new(addr, h),
        }
    }
}

// async server

pub struct DebuggerAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl DebuggerAsyncServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : DebuggerAsync + 'static + Sync + Send + 'static>(addr: A, h: H) -> Self {
        let service_definition = DebuggerAsyncServer::new_service_def(h);
        DebuggerAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(addr, service_definition),
        }
    }

    pub fn new_service_def<H : DebuggerAsync + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Debugger/Ping".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Ping(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Debugger/Stop".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Stop(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Debugger/Continue".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Continue(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Debugger/Disassemble".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Disassemble(p))
                    },
                ),
            ],
        )
    }
}
