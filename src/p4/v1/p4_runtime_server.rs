use tonic::codegen::*;
#[doc = " Generated trait containing gRPC methods that should be implemented for use with P4RuntimeServer."]
#[async_trait]
pub trait P4Runtime: Send + Sync + 'static {
    #[doc = " Update one or more P4 entities on the target."]
    async fn write(
        &self,
        request: tonic::Request<super::WriteRequest>,
    ) -> std::result::Result<tonic::Response<super::WriteResponse>, tonic::Status>;
    #[doc = " Server streaming response type for the Read method."]
    type ReadStream: tonic::codegen::tokio_stream::Stream<
            Item = std::result::Result<super::ReadResponse, tonic::Status>,
        > + Send
        + 'static;
    #[doc = " Read one or more P4 entities from the target."]
    async fn read(
        &self,
        request: tonic::Request<super::ReadRequest>,
    ) -> std::result::Result<tonic::Response<Self::ReadStream>, tonic::Status>;
    #[doc = " Sets the P4 forwarding-pipeline config."]
    async fn set_forwarding_pipeline_config(
        &self,
        request: tonic::Request<super::SetForwardingPipelineConfigRequest>,
    ) -> std::result::Result<
        tonic::Response<super::SetForwardingPipelineConfigResponse>,
        tonic::Status,
    >;
    #[doc = " Gets the current P4 forwarding-pipeline config."]
    async fn get_forwarding_pipeline_config(
        &self,
        request: tonic::Request<super::GetForwardingPipelineConfigRequest>,
    ) -> std::result::Result<
        tonic::Response<super::GetForwardingPipelineConfigResponse>,
        tonic::Status,
    >;
    #[doc = " Server streaming response type for the StreamChannel method."]
    type StreamChannelStream: tonic::codegen::tokio_stream::Stream<
            Item = std::result::Result<super::StreamMessageResponse, tonic::Status>,
        > + Send
        + 'static;
    #[doc = " Represents the bidirectional stream between the controller and the"]
    #[doc = " switch (initiated by the controller), and is managed for the following"]
    #[doc = " purposes:"]
    #[doc = " - connection initiation through client arbitration"]
    #[doc = " - indicating switch session liveness: the session is live when switch"]
    #[doc = "   sends a positive client arbitration update to the controller, and is"]
    #[doc = "   considered dead when either the stream breaks or the switch sends a"]
    #[doc = "   negative update for client arbitration"]
    #[doc = " - the controller sending/receiving packets to/from the switch"]
    #[doc = " - streaming of notifications from the switch"]
    async fn stream_channel(
        &self,
        request: tonic::Request<tonic::Streaming<super::StreamMessageRequest>>,
    ) -> std::result::Result<tonic::Response<Self::StreamChannelStream>, tonic::Status>;
    async fn capabilities(
        &self,
        request: tonic::Request<super::CapabilitiesRequest>,
    ) -> std::result::Result<tonic::Response<super::CapabilitiesResponse>, tonic::Status>;
}
#[derive(Debug)]
pub struct P4RuntimeServer<T: P4Runtime> {
    inner: _Inner<T>,
    accept_compression_encodings: EnabledCompressionEncodings,
    send_compression_encodings: EnabledCompressionEncodings,
    max_decoding_message_size: Option<usize>,
    max_encoding_message_size: Option<usize>,
}
struct _Inner<T>(Arc<T>);
impl<T: P4Runtime> P4RuntimeServer<T> {
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
    pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
    where
        F: tonic::service::Interceptor,
    {
        InterceptedService::new(Self::new(inner), interceptor)
    }
    #[doc = " Enable decompressing requests with the given encoding."]
    #[must_use]
    pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
        self.accept_compression_encodings.enable(encoding);
        self
    }
    #[doc = " Compress responses with the given encoding, if the client supports it."]
    #[must_use]
    pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
        self.send_compression_encodings.enable(encoding);
        self
    }
    #[doc = " Limits the maximum size of a decoded message."]
    #[doc = ""]
    #[doc = " Default: `4MB`"]
    #[must_use]
    pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
        self.max_decoding_message_size = Some(limit);
        self
    }
    #[doc = " Limits the maximum size of an encoded message."]
    #[doc = ""]
    #[doc = " Default: `usize::MAX`"]
    #[must_use]
    pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
        self.max_encoding_message_size = Some(limit);
        self
    }
}
impl<T, B> tonic::codegen::Service<http::Request<B>> for P4RuntimeServer<T>
where
    T: P4Runtime,
    B: Body + Send + 'static,
    B::Error: Into<StdError> + Send + 'static,
{
    type Response = http::Response<tonic::body::BoxBody>;
    type Error = std::convert::Infallible;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<std::result::Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    fn call(&mut self, req: http::Request<B>) -> Self::Future {
        let inner = self.inner.clone();
        match req.uri().path() {
            "/p4.v1.P4Runtime/Write" => {
                #[allow(non_camel_case_types)]
                struct WriteSvc<T: P4Runtime>(pub Arc<T>);
                impl<T: P4Runtime> tonic::server::UnaryService<super::WriteRequest> for WriteSvc<T> {
                    type Response = super::WriteResponse;
                    type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                    fn call(
                        &mut self,
                        request: tonic::Request<super::WriteRequest>,
                    ) -> Self::Future {
                        let inner = Arc::clone(&self.0);
                        let fut = async move { <T as P4Runtime>::write(&inner, request).await };
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
                    let method = WriteSvc(inner);
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
            "/p4.v1.P4Runtime/Read" => {
                #[allow(non_camel_case_types)]
                struct ReadSvc<T: P4Runtime>(pub Arc<T>);
                impl<T: P4Runtime> tonic::server::ServerStreamingService<super::ReadRequest> for ReadSvc<T> {
                    type Response = super::ReadResponse;
                    type ResponseStream = T::ReadStream;
                    type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                    fn call(
                        &mut self,
                        request: tonic::Request<super::ReadRequest>,
                    ) -> Self::Future {
                        let inner = Arc::clone(&self.0);
                        let fut = async move { <T as P4Runtime>::read(&inner, request).await };
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
                    let method = ReadSvc(inner);
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
            "/p4.v1.P4Runtime/SetForwardingPipelineConfig" => {
                #[allow(non_camel_case_types)]
                struct SetForwardingPipelineConfigSvc<T: P4Runtime>(pub Arc<T>);
                impl<T: P4Runtime>
                    tonic::server::UnaryService<super::SetForwardingPipelineConfigRequest>
                    for SetForwardingPipelineConfigSvc<T>
                {
                    type Response = super::SetForwardingPipelineConfigResponse;
                    type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                    fn call(
                        &mut self,
                        request: tonic::Request<super::SetForwardingPipelineConfigRequest>,
                    ) -> Self::Future {
                        let inner = Arc::clone(&self.0);
                        let fut = async move {
                            <T as P4Runtime>::set_forwarding_pipeline_config(&inner, request).await
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
                    let method = SetForwardingPipelineConfigSvc(inner);
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
            "/p4.v1.P4Runtime/GetForwardingPipelineConfig" => {
                #[allow(non_camel_case_types)]
                struct GetForwardingPipelineConfigSvc<T: P4Runtime>(pub Arc<T>);
                impl<T: P4Runtime>
                    tonic::server::UnaryService<super::GetForwardingPipelineConfigRequest>
                    for GetForwardingPipelineConfigSvc<T>
                {
                    type Response = super::GetForwardingPipelineConfigResponse;
                    type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                    fn call(
                        &mut self,
                        request: tonic::Request<super::GetForwardingPipelineConfigRequest>,
                    ) -> Self::Future {
                        let inner = Arc::clone(&self.0);
                        let fut = async move {
                            <T as P4Runtime>::get_forwarding_pipeline_config(&inner, request).await
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
                    let method = GetForwardingPipelineConfigSvc(inner);
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
            "/p4.v1.P4Runtime/StreamChannel" => {
                #[allow(non_camel_case_types)]
                struct StreamChannelSvc<T: P4Runtime>(pub Arc<T>);
                impl<T: P4Runtime> tonic::server::StreamingService<super::StreamMessageRequest>
                    for StreamChannelSvc<T>
                {
                    type Response = super::StreamMessageResponse;
                    type ResponseStream = T::StreamChannelStream;
                    type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                    fn call(
                        &mut self,
                        request: tonic::Request<tonic::Streaming<super::StreamMessageRequest>>,
                    ) -> Self::Future {
                        let inner = Arc::clone(&self.0);
                        let fut =
                            async move { <T as P4Runtime>::stream_channel(&inner, request).await };
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
                    let method = StreamChannelSvc(inner);
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
                    let res = grpc.streaming(method, req).await;
                    Ok(res)
                };
                Box::pin(fut)
            }
            "/p4.v1.P4Runtime/Capabilities" => {
                #[allow(non_camel_case_types)]
                struct CapabilitiesSvc<T: P4Runtime>(pub Arc<T>);
                impl<T: P4Runtime> tonic::server::UnaryService<super::CapabilitiesRequest> for CapabilitiesSvc<T> {
                    type Response = super::CapabilitiesResponse;
                    type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                    fn call(
                        &mut self,
                        request: tonic::Request<super::CapabilitiesRequest>,
                    ) -> Self::Future {
                        let inner = Arc::clone(&self.0);
                        let fut =
                            async move { <T as P4Runtime>::capabilities(&inner, request).await };
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
                    let method = CapabilitiesSvc(inner);
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
            _ => Box::pin(async move {
                Ok(http::Response::builder()
                    .status(200)
                    .header("grpc-status", "12")
                    .header("content-type", "application/grpc")
                    .body(empty_body())
                    .unwrap())
            }),
        }
    }
}
impl<T: P4Runtime> Clone for P4RuntimeServer<T> {
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
impl<T: P4Runtime> Clone for _Inner<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl<T: P4Runtime> tonic::server::NamedService for P4RuntimeServer<T> {
    const NAME: &'static str = "p4.v1.P4Runtime";
}
