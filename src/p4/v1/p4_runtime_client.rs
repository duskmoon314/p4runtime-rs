use tonic::codegen::http::Uri;
use tonic::codegen::*;
#[derive(Debug, Clone)]
pub struct P4RuntimeClient<T> {
    inner: tonic::client::Grpc<T>,
}
impl P4RuntimeClient<tonic::transport::Channel> {
    #[doc = " Attempt to create a new client by connecting to a given endpoint."]
    pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
        D: TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
        Ok(Self::new(conn))
    }
}
impl<T> P4RuntimeClient<T>
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
    ) -> P4RuntimeClient<InterceptedService<T, F>>
    where
        F: tonic::service::Interceptor,
        T::ResponseBody: Default,
        T: tonic::codegen::Service<
            http::Request<tonic::body::BoxBody>,
            Response = http::Response<
                <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
            >,
        >,
        <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
            Into<StdError> + Send + Sync,
    {
        P4RuntimeClient::new(InterceptedService::new(inner, interceptor))
    }
    #[doc = " Compress requests with the given encoding."]
    #[doc = ""]
    #[doc = " This requires the server to support it otherwise it might respond with an"]
    #[doc = " error."]
    #[must_use]
    pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
        self.inner = self.inner.send_compressed(encoding);
        self
    }
    #[doc = " Enable decompressing responses."]
    #[must_use]
    pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
        self.inner = self.inner.accept_compressed(encoding);
        self
    }
    #[doc = " Limits the maximum size of a decoded message."]
    #[doc = ""]
    #[doc = " Default: `4MB`"]
    #[must_use]
    pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
        self.inner = self.inner.max_decoding_message_size(limit);
        self
    }
    #[doc = " Limits the maximum size of an encoded message."]
    #[doc = ""]
    #[doc = " Default: `usize::MAX`"]
    #[must_use]
    pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
        self.inner = self.inner.max_encoding_message_size(limit);
        self
    }
    #[doc = " Update one or more P4 entities on the target."]
    pub async fn write(
        &mut self,
        request: impl tonic::IntoRequest<super::WriteRequest>,
    ) -> std::result::Result<tonic::Response<super::WriteResponse>, tonic::Status> {
        self.inner.ready().await.map_err(|e| {
            tonic::Status::new(
                tonic::Code::Unknown,
                format!("Service was not ready: {}", e.into()),
            )
        })?;
        let codec = tonic::codec::ProstCodec::default();
        let path = http::uri::PathAndQuery::from_static("/p4.v1.P4Runtime/Write");
        let mut req = request.into_request();
        req.extensions_mut()
            .insert(GrpcMethod::new("p4.v1.P4Runtime", "Write"));
        self.inner.unary(req, path, codec).await
    }
    #[doc = " Read one or more P4 entities from the target."]
    pub async fn read(
        &mut self,
        request: impl tonic::IntoRequest<super::ReadRequest>,
    ) -> std::result::Result<
        tonic::Response<tonic::codec::Streaming<super::ReadResponse>>,
        tonic::Status,
    > {
        self.inner.ready().await.map_err(|e| {
            tonic::Status::new(
                tonic::Code::Unknown,
                format!("Service was not ready: {}", e.into()),
            )
        })?;
        let codec = tonic::codec::ProstCodec::default();
        let path = http::uri::PathAndQuery::from_static("/p4.v1.P4Runtime/Read");
        let mut req = request.into_request();
        req.extensions_mut()
            .insert(GrpcMethod::new("p4.v1.P4Runtime", "Read"));
        self.inner.server_streaming(req, path, codec).await
    }
    #[doc = " Sets the P4 forwarding-pipeline config."]
    pub async fn set_forwarding_pipeline_config(
        &mut self,
        request: impl tonic::IntoRequest<super::SetForwardingPipelineConfigRequest>,
    ) -> std::result::Result<
        tonic::Response<super::SetForwardingPipelineConfigResponse>,
        tonic::Status,
    > {
        self.inner.ready().await.map_err(|e| {
            tonic::Status::new(
                tonic::Code::Unknown,
                format!("Service was not ready: {}", e.into()),
            )
        })?;
        let codec = tonic::codec::ProstCodec::default();
        let path =
            http::uri::PathAndQuery::from_static("/p4.v1.P4Runtime/SetForwardingPipelineConfig");
        let mut req = request.into_request();
        req.extensions_mut().insert(GrpcMethod::new(
            "p4.v1.P4Runtime",
            "SetForwardingPipelineConfig",
        ));
        self.inner.unary(req, path, codec).await
    }
    #[doc = " Gets the current P4 forwarding-pipeline config."]
    pub async fn get_forwarding_pipeline_config(
        &mut self,
        request: impl tonic::IntoRequest<super::GetForwardingPipelineConfigRequest>,
    ) -> std::result::Result<
        tonic::Response<super::GetForwardingPipelineConfigResponse>,
        tonic::Status,
    > {
        self.inner.ready().await.map_err(|e| {
            tonic::Status::new(
                tonic::Code::Unknown,
                format!("Service was not ready: {}", e.into()),
            )
        })?;
        let codec = tonic::codec::ProstCodec::default();
        let path =
            http::uri::PathAndQuery::from_static("/p4.v1.P4Runtime/GetForwardingPipelineConfig");
        let mut req = request.into_request();
        req.extensions_mut().insert(GrpcMethod::new(
            "p4.v1.P4Runtime",
            "GetForwardingPipelineConfig",
        ));
        self.inner.unary(req, path, codec).await
    }
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
    pub async fn stream_channel(
        &mut self,
        request: impl tonic::IntoStreamingRequest<Message = super::StreamMessageRequest>,
    ) -> std::result::Result<
        tonic::Response<tonic::codec::Streaming<super::StreamMessageResponse>>,
        tonic::Status,
    > {
        self.inner.ready().await.map_err(|e| {
            tonic::Status::new(
                tonic::Code::Unknown,
                format!("Service was not ready: {}", e.into()),
            )
        })?;
        let codec = tonic::codec::ProstCodec::default();
        let path = http::uri::PathAndQuery::from_static("/p4.v1.P4Runtime/StreamChannel");
        let mut req = request.into_streaming_request();
        req.extensions_mut()
            .insert(GrpcMethod::new("p4.v1.P4Runtime", "StreamChannel"));
        self.inner.streaming(req, path, codec).await
    }
    pub async fn capabilities(
        &mut self,
        request: impl tonic::IntoRequest<super::CapabilitiesRequest>,
    ) -> std::result::Result<tonic::Response<super::CapabilitiesResponse>, tonic::Status> {
        self.inner.ready().await.map_err(|e| {
            tonic::Status::new(
                tonic::Code::Unknown,
                format!("Service was not ready: {}", e.into()),
            )
        })?;
        let codec = tonic::codec::ProstCodec::default();
        let path = http::uri::PathAndQuery::from_static("/p4.v1.P4Runtime/Capabilities");
        let mut req = request.into_request();
        req.extensions_mut()
            .insert(GrpcMethod::new("p4.v1.P4Runtime", "Capabilities"));
        self.inner.unary(req, path, codec).await
    }
}
