use p4runtime::p4::v1 as p4v1;

#[derive(Debug, thiserror::Error)]
pub enum SendStreamMessageRequestError {
    #[error("tonic status: {0}")]
    TonicStatus(#[from] tonic::Status),

    #[error("tokio mpsc sender: {0}")]
    TokioMpscSender(#[from] tokio::sync::mpsc::error::SendError<p4v1::StreamMessageRequest>),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadEntitySingleError {
    #[error("Expected exactly one entity, found none")]
    NoneFound,

    #[error("Expected exactly one entity, found {0}")]
    MultipleFound(usize),

    #[error("Expected entity {0}, found {1}")]
    UnexpectedEntity(String, String),

    #[error("tonic status: {0}")]
    TonicStatus(#[from] tonic::Status),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadEntitiesError {
    #[error("tonic status: {0}")]
    TonicStatus(#[from] tonic::Status),

    #[error("Expected entity {0}, found {1}")]
    UnexpectedEntity(String, String),
}
