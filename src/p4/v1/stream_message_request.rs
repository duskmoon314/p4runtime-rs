#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Oneof)]
pub enum Update {
    #[prost(message, tag = "1")]
    Arbitration(super::MasterArbitrationUpdate),
    #[prost(message, tag = "2")]
    Packet(super::PacketOut),
    #[prost(message, tag = "3")]
    DigestAck(super::DigestListAck),
    #[prost(message, tag = "4")]
    Other(super::super::super::google::protobuf::Any),
}
