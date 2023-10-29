#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttachRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttachReply {
    #[prost(message, optional, tag = "1")]
    pub cpu: ::core::option::Option<CpuState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeReply {
    #[prost(message, optional, tag = "1")]
    pub cpu: ::core::option::Option<CpuState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestartRequest {
    #[prost(bool, tag = "1")]
    pub set_program_counter: bool,
    #[prost(uint32, tag = "2")]
    pub program_counter: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestartReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepReply {
    #[prost(message, optional, tag = "1")]
    pub cpu: ::core::option::Option<CpuState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutBreakpointRequest {
    #[prost(uint32, tag = "1")]
    pub address: u32,
    #[prost(bool, tag = "2")]
    pub temporary: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBreakpointRequest {
    #[prost(uint32, tag = "1")]
    pub address: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BreakpointReply {
    #[prost(uint32, tag = "1")]
    pub address: u32,
    #[prost(bool, tag = "2")]
    pub temporary: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadMemoryRequest {
    #[prost(uint32, tag = "1")]
    pub start: u32,
    #[prost(int32, tag = "2")]
    pub count: i32,
    #[prost(bool, tag = "3")]
    pub count_by_instruction: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadMemoryReply {
    #[prost(uint32, tag = "1")]
    pub start: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "3")]
    pub count: u32,
    #[prost(uint32, tag = "4")]
    pub program_counter: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusReply {
    #[prost(string, tag = "1")]
    pub rom_name: ::prost::alloc::string::String,
    #[prost(enumeration = "status_reply::EmulationState", tag = "2")]
    pub emulation_state: i32,
}
/// Nested message and enum types in `StatusReply`.
pub mod status_reply {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum EmulationState {
        Unknown = 0,
        Running = 1,
        Stopped = 2,
        Error = 3,
    }
    impl EmulationState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EmulationState::Unknown => "UNKNOWN",
                EmulationState::Running => "RUNNING",
                EmulationState::Stopped => "STOPPED",
                EmulationState::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "RUNNING" => Some(Self::Running),
                "STOPPED" => Some(Self::Stopped),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadCpuRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadCpuReply {
    #[prost(message, optional, tag = "1")]
    pub cpu: ::core::option::Option<CpuState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CpuState {
    #[prost(uint32, tag = "1")]
    pub x: u32,
    #[prost(uint32, tag = "2")]
    pub y: u32,
    #[prost(uint32, tag = "3")]
    pub ac: u32,
    #[prost(message, optional, tag = "4")]
    pub status: ::core::option::Option<cpu_state::CpuStatusRegister>,
    #[prost(uint32, tag = "5")]
    pub program_counter: u32,
    #[prost(uint32, tag = "6")]
    pub stack_pointer: u32,
}
/// Nested message and enum types in `CPUState`.
pub mod cpu_state {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CpuStatusRegister {
        #[prost(bool, tag = "1")]
        pub negative: bool,
        #[prost(bool, tag = "2")]
        pub overflow: bool,
        #[prost(bool, tag = "3")]
        pub interrupt: bool,
        #[prost(bool, tag = "4")]
        pub zero: bool,
        #[prost(bool, tag = "5")]
        pub carry: bool,
    }
}
/// Generated client implementations.
pub mod bobomb_debugger_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct BobombDebuggerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BobombDebuggerClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BobombDebuggerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BobombDebuggerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            BobombDebuggerClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn attach(
            &mut self,
            request: impl tonic::IntoRequest<super::AttachRequest>,
        ) -> std::result::Result<tonic::Response<super::AttachReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/debugger.BobombDebugger/Attach",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("debugger.BobombDebugger", "Attach"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn resume(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ResumeReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/debugger.BobombDebugger/Resume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("debugger.BobombDebugger", "Resume"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn restart(
            &mut self,
            request: impl tonic::IntoRequest<super::RestartRequest>,
        ) -> std::result::Result<tonic::Response<super::RestartReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/debugger.BobombDebugger/Restart",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("debugger.BobombDebugger", "Restart"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn step(
            &mut self,
            request: impl tonic::IntoRequest<super::StepRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StepReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/debugger.BobombDebugger/Step",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("debugger.BobombDebugger", "Step"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn put_breakpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::PutBreakpointRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BreakpointReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/debugger.BobombDebugger/PutBreakpoint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("debugger.BobombDebugger", "PutBreakpoint"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_breakpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBreakpointRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BreakpointReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/debugger.BobombDebugger/DeleteBreakpoint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("debugger.BobombDebugger", "DeleteBreakpoint"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_memory(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadMemoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadMemoryReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/debugger.BobombDebugger/ReadMemory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("debugger.BobombDebugger", "ReadMemory"));
            self.inner.unary(req, path, codec).await
        }
        /// TODO WriteMemory
        pub async fn read_cpu(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadCpuRequest>,
        ) -> std::result::Result<tonic::Response<super::ReadCpuReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/debugger.BobombDebugger/ReadCPU",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("debugger.BobombDebugger", "ReadCPU"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn status(
            &mut self,
            request: impl tonic::IntoRequest<super::StatusRequest>,
        ) -> std::result::Result<tonic::Response<super::StatusReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/debugger.BobombDebugger/Status",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("debugger.BobombDebugger", "Status"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod bobomb_debugger_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BobombDebuggerServer.
    #[async_trait]
    pub trait BobombDebugger: Send + Sync + 'static {
        async fn attach(
            &self,
            request: tonic::Request<super::AttachRequest>,
        ) -> std::result::Result<tonic::Response<super::AttachReply>, tonic::Status>;
        /// Server streaming response type for the Resume method.
        type ResumeStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ResumeReply, tonic::Status>,
            >
            + Send
            + 'static;
        async fn resume(
            &self,
            request: tonic::Request<super::ResumeRequest>,
        ) -> std::result::Result<tonic::Response<Self::ResumeStream>, tonic::Status>;
        async fn restart(
            &self,
            request: tonic::Request<super::RestartRequest>,
        ) -> std::result::Result<tonic::Response<super::RestartReply>, tonic::Status>;
        /// Server streaming response type for the Step method.
        type StepStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::StepReply, tonic::Status>,
            >
            + Send
            + 'static;
        async fn step(
            &self,
            request: tonic::Request<super::StepRequest>,
        ) -> std::result::Result<tonic::Response<Self::StepStream>, tonic::Status>;
        async fn put_breakpoint(
            &self,
            request: tonic::Request<super::PutBreakpointRequest>,
        ) -> std::result::Result<tonic::Response<super::BreakpointReply>, tonic::Status>;
        async fn delete_breakpoint(
            &self,
            request: tonic::Request<super::DeleteBreakpointRequest>,
        ) -> std::result::Result<tonic::Response<super::BreakpointReply>, tonic::Status>;
        async fn read_memory(
            &self,
            request: tonic::Request<super::ReadMemoryRequest>,
        ) -> std::result::Result<tonic::Response<super::ReadMemoryReply>, tonic::Status>;
        /// TODO WriteMemory
        async fn read_cpu(
            &self,
            request: tonic::Request<super::ReadCpuRequest>,
        ) -> std::result::Result<tonic::Response<super::ReadCpuReply>, tonic::Status>;
        async fn status(
            &self,
            request: tonic::Request<super::StatusRequest>,
        ) -> std::result::Result<tonic::Response<super::StatusReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct BobombDebuggerServer<T: BobombDebugger> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BobombDebugger> BobombDebuggerServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BobombDebuggerServer<T>
    where
        T: BobombDebugger,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/debugger.BobombDebugger/Attach" => {
                    #[allow(non_camel_case_types)]
                    struct AttachSvc<T: BobombDebugger>(pub Arc<T>);
                    impl<
                        T: BobombDebugger,
                    > tonic::server::UnaryService<super::AttachRequest>
                    for AttachSvc<T> {
                        type Response = super::AttachReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AttachRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BobombDebugger>::attach(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AttachSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/debugger.BobombDebugger/Resume" => {
                    #[allow(non_camel_case_types)]
                    struct ResumeSvc<T: BobombDebugger>(pub Arc<T>);
                    impl<
                        T: BobombDebugger,
                    > tonic::server::ServerStreamingService<super::ResumeRequest>
                    for ResumeSvc<T> {
                        type Response = super::ResumeReply;
                        type ResponseStream = T::ResumeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResumeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BobombDebugger>::resume(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ResumeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/debugger.BobombDebugger/Restart" => {
                    #[allow(non_camel_case_types)]
                    struct RestartSvc<T: BobombDebugger>(pub Arc<T>);
                    impl<
                        T: BobombDebugger,
                    > tonic::server::UnaryService<super::RestartRequest>
                    for RestartSvc<T> {
                        type Response = super::RestartReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RestartRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BobombDebugger>::restart(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RestartSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/debugger.BobombDebugger/Step" => {
                    #[allow(non_camel_case_types)]
                    struct StepSvc<T: BobombDebugger>(pub Arc<T>);
                    impl<
                        T: BobombDebugger,
                    > tonic::server::ServerStreamingService<super::StepRequest>
                    for StepSvc<T> {
                        type Response = super::StepReply;
                        type ResponseStream = T::StepStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StepRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BobombDebugger>::step(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StepSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/debugger.BobombDebugger/PutBreakpoint" => {
                    #[allow(non_camel_case_types)]
                    struct PutBreakpointSvc<T: BobombDebugger>(pub Arc<T>);
                    impl<
                        T: BobombDebugger,
                    > tonic::server::UnaryService<super::PutBreakpointRequest>
                    for PutBreakpointSvc<T> {
                        type Response = super::BreakpointReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PutBreakpointRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BobombDebugger>::put_breakpoint(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PutBreakpointSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/debugger.BobombDebugger/DeleteBreakpoint" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteBreakpointSvc<T: BobombDebugger>(pub Arc<T>);
                    impl<
                        T: BobombDebugger,
                    > tonic::server::UnaryService<super::DeleteBreakpointRequest>
                    for DeleteBreakpointSvc<T> {
                        type Response = super::BreakpointReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteBreakpointRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BobombDebugger>::delete_breakpoint(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteBreakpointSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/debugger.BobombDebugger/ReadMemory" => {
                    #[allow(non_camel_case_types)]
                    struct ReadMemorySvc<T: BobombDebugger>(pub Arc<T>);
                    impl<
                        T: BobombDebugger,
                    > tonic::server::UnaryService<super::ReadMemoryRequest>
                    for ReadMemorySvc<T> {
                        type Response = super::ReadMemoryReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadMemoryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BobombDebugger>::read_memory(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadMemorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/debugger.BobombDebugger/ReadCPU" => {
                    #[allow(non_camel_case_types)]
                    struct ReadCPUSvc<T: BobombDebugger>(pub Arc<T>);
                    impl<
                        T: BobombDebugger,
                    > tonic::server::UnaryService<super::ReadCpuRequest>
                    for ReadCPUSvc<T> {
                        type Response = super::ReadCpuReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadCpuRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BobombDebugger>::read_cpu(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadCPUSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/debugger.BobombDebugger/Status" => {
                    #[allow(non_camel_case_types)]
                    struct StatusSvc<T: BobombDebugger>(pub Arc<T>);
                    impl<
                        T: BobombDebugger,
                    > tonic::server::UnaryService<super::StatusRequest>
                    for StatusSvc<T> {
                        type Response = super::StatusReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BobombDebugger>::status(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: BobombDebugger> Clone for BobombDebuggerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: BobombDebugger> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BobombDebugger> tonic::server::NamedService for BobombDebuggerServer<T> {
        const NAME: &'static str = "debugger.BobombDebugger";
    }
}
