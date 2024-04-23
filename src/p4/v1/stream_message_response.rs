#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Oneof)]
pub enum Update {
    #[prost(message, tag = "1")]
    Arbitration(super::MasterArbitrationUpdate),
    #[prost(message, tag = "2")]
    Packet(super::PacketIn),
    #[prost(message, tag = "3")]
    Digest(super::DigestList),
    #[prost(message, tag = "4")]
    IdleTimeoutNotification(super::IdleTimeoutNotification),
    #[prost(message, tag = "5")]
    Other(::prost_types::Any),
    #[doc = " Used by the server to asynchronously report errors which occur when"]
    #[doc = " processing StreamMessageRequest messages."]
    #[prost(message, tag = "6")]
    Error(super::StreamError),
}
