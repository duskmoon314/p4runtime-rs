#[doc = " Used by the server to convey additional information about the error. One of"]
#[doc = " the fields must be set (so that the client can identify which type of"]
#[doc = " stream message triggered the error), but that field may be set to its"]
#[doc = " default value."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Oneof)]
pub enum Details {
    #[prost(message, tag = "5")]
    PacketOut(super::PacketOutError),
    #[prost(message, tag = "6")]
    DigestListAck(super::DigestListAckError),
    #[prost(message, tag = "7")]
    Other(super::StreamOtherError),
}
