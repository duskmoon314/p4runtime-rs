use p4runtime::p4::v1 as p4v1;
use tonic::codegen::*;

use crate::{counter::CounterHelper, p4info::P4Info, register::RegisterHelper, table::TableHelper};

/// Client options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientOptions {
    pub canonical_bytestrings: bool,

    pub stream_channel_buffer_size: usize,
}

impl ClientOptions {
    pub fn canonical_bytestrings_mut(&mut self) -> &mut bool {
        &mut self.canonical_bytestrings
    }
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            canonical_bytestrings: true,
            stream_channel_buffer_size: 1000,
        }
    }
}

/// A client wrapper
#[derive(Debug)]
pub struct Client<T> {
    /// P4Runtime client
    pub p4rt_client: p4v1::p4_runtime_client::P4RuntimeClient<T>,

    /// Device ID
    pub device_id: u64,

    /// Election ID
    pub election_id: u128,

    /// P4Info
    pub p4info: P4Info,

    /// Role
    pub role: Option<p4v1::Role>,

    /// Options
    pub options: ClientOptions,

    /// Stream Request Sender
    pub stream_request_sender: Option<tokio::sync::mpsc::Sender<p4v1::StreamMessageRequest>>,
}

impl<T> AsRef<Client<T>> for &Client<T> {
    fn as_ref(&self) -> &Client<T> {
        self
    }
}

impl<T> AsRef<Client<T>> for &mut Client<T> {
    fn as_ref(&self) -> &Client<T> {
        self
    }
}

impl<T> AsMut<Client<T>> for &mut Client<T> {
    fn as_mut(&mut self) -> &mut Client<T> {
        self
    }
}

impl<T> Client<T> {
    pub fn new(
        client: p4v1::p4_runtime_client::P4RuntimeClient<T>,
        device_id: u64,
        election_id: u128,
        role: Option<p4v1::Role>,
        options: ClientOptions,
    ) -> Self {
        Self {
            p4rt_client: client,
            device_id,
            election_id,
            p4info: P4Info::default(),
            role,
            options,
            stream_request_sender: None,
        }
    }

    pub fn role_name(&self) -> String {
        self.role
            .as_ref()
            .map(|r| r.name.clone())
            .unwrap_or_default()
    }

    pub fn p4info(&self) -> &P4Info {
        &self.p4info
    }

    pub fn p4info_mut(&mut self) -> &mut P4Info {
        &mut self.p4info
    }

    pub fn table(&self) -> TableHelper<&Self, T> {
        TableHelper::new(self)
    }

    pub fn table_mut(&mut self) -> TableHelper<&mut Self, T> {
        TableHelper::new(self)
    }

    pub fn counter(&self) -> CounterHelper<&Self, T> {
        CounterHelper::new(self)
    }

    pub fn counter_mut(&mut self) -> CounterHelper<&mut Self, T> {
        CounterHelper::new(self)
    }

    pub fn register(&self) -> RegisterHelper<&Self, T> {
        RegisterHelper::new(self)
    }

    pub fn register_mut(&mut self) -> RegisterHelper<&mut Self, T> {
        RegisterHelper::new(self)
    }
}

impl<T> Client<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    pub async fn arbitration(
        &mut self,
    ) -> Result<
        tonic::Response<tonic::codec::Streaming<p4v1::StreamMessageResponse>>,
        crate::SendStreamMessageRequestError,
    > {
        let (stream_request_sender, stream_request_receiver) =
            tokio::sync::mpsc::channel(self.options.stream_channel_buffer_size);
        self.stream_request_sender = Some(stream_request_sender);

        let req = p4v1::StreamMessageRequest {
            update: Some(p4v1::stream_message_request::Update::Arbitration(
                p4v1::MasterArbitrationUpdate {
                    device_id: self.device_id,
                    role: self.role.clone(),
                    election_id: Some(self.election_id.into()),
                    status: None,
                },
            )),
        };

        self.stream_request_sender
            .as_ref()
            .unwrap()
            .send(req)
            .await?;

        self.p4rt_client
            .stream_channel(tokio_stream::wrappers::ReceiverStream::new(
                stream_request_receiver,
            ))
            .await
            .map_err(Into::into)
    }

    pub async fn set_forwarding_pipeline(
        &mut self,
        p4_device_config: Vec<u8>,
    ) -> Result<tonic::Response<p4v1::SetForwardingPipelineConfigResponse>, tonic::Status> {
        let req = p4v1::SetForwardingPipelineConfigRequest {
            device_id: self.device_id,
            role: self.role_name(),
            election_id: Some(self.election_id.into()),
            action: p4v1::set_forwarding_pipeline_config_request::Action::VerifyAndCommit as i32,
            config: Some(p4v1::ForwardingPipelineConfig {
                p4info: Some(self.p4info.as_ref().clone()),
                p4_device_config,
                cookie: Some(p4v1::forwarding_pipeline_config::Cookie { cookie: 0 }),
            }),
            ..Default::default()
        };

        self.p4rt_client.set_forwarding_pipeline_config(req).await
    }

    pub async fn write_update(
        &mut self,
        update: p4v1::Update,
    ) -> Result<tonic::Response<p4v1::WriteResponse>, tonic::Status> {
        self.write_update_batch(vec![update]).await
    }

    #[inline]
    pub async fn write_update_batch(
        &mut self,
        updates: Vec<p4v1::Update>,
    ) -> Result<tonic::Response<p4v1::WriteResponse>, tonic::Status> {
        let req = p4v1::WriteRequest {
            device_id: self.device_id,
            role: self.role_name(),
            election_id: Some(self.election_id.into()),
            updates,

            ..Default::default()
        };

        self.p4rt_client.write(req).await
    }

    #[inline]
    pub async fn read_entity_stream(
        &mut self,
        entity: p4v1::Entity,
    ) -> Result<tonic::Response<tonic::codec::Streaming<p4v1::ReadResponse>>, tonic::Status> {
        self.read_entity_stream_batch(vec![entity]).await
    }

    #[inline]
    pub async fn read_entity_stream_batch(
        &mut self,
        entities: Vec<p4v1::Entity>,
    ) -> Result<tonic::Response<tonic::codec::Streaming<p4v1::ReadResponse>>, tonic::Status> {
        let req = p4v1::ReadRequest {
            device_id: self.device_id,
            role: self.role_name(),
            entities,
        };

        self.p4rt_client.read(req).await
    }

    /// Read only one entity
    pub async fn read_entity_single(
        &mut self,
        entity: p4v1::Entity,
    ) -> Result<p4v1::Entity, crate::error::ReadEntitySingleError> {
        let mut stream: tonic::Response<tonic::codec::Streaming<p4v1::ReadResponse>> =
            self.read_entity_stream(entity).await?;

        // Get entity from stream
        let mut entities: Vec<p4v1::Entity> = Vec::new();

        while let Some(res) = stream.get_mut().message().await? {
            entities.extend(res.entities);
        }

        match entities.len() {
            0 => Err(crate::error::ReadEntitySingleError::NoneFound),
            1 => Ok(entities.swap_remove(0)),
            n @ 2.. => Err(crate::error::ReadEntitySingleError::MultipleFound(n)),
        }
    }

    /// Read entities
    pub async fn read_entity_wildcard(
        &mut self,
        entity: p4v1::Entity,
    ) -> Result<Vec<p4v1::Entity>, tonic::Status> {
        let mut stream: tonic::Response<tonic::codec::Streaming<p4v1::ReadResponse>> =
            self.read_entity_stream(entity).await?;

        // Get entity from stream
        let mut entities: Vec<p4v1::Entity> = Vec::new();

        while let Some(res) = stream.get_mut().message().await? {
            entities.extend(res.entities);
        }

        Ok(entities)
    }

    /// Read entities in batch
    pub async fn read_entity_batch(
        &mut self,
        entities: Vec<p4v1::Entity>,
    ) -> Result<Vec<p4v1::Entity>, tonic::Status> {
        let mut stream: tonic::Response<tonic::codec::Streaming<p4v1::ReadResponse>> =
            self.read_entity_stream_batch(entities).await?;

        // Get entity from stream
        let mut entities: Vec<p4v1::Entity> = Vec::new();

        while let Some(res) = stream.get_mut().message().await? {
            entities.extend(res.entities);
        }

        Ok(entities)
    }

    /// Send StreamMessageRequest
    pub async fn send_message_request(
        &mut self,
        req: p4v1::StreamMessageRequest,
    ) -> Result<(), crate::SendStreamMessageRequestError> {
        self.stream_request_sender
            .as_ref()
            .unwrap()
            .send(req)
            .await?;

        Ok(())
    }
}
