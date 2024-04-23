#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Oneof)]
pub enum Type {
    #[prost(message, tag = "1")]
    MulticastGroupEntry(super::MulticastGroupEntry),
    #[prost(message, tag = "2")]
    CloneSessionEntry(super::CloneSessionEntry),
}
