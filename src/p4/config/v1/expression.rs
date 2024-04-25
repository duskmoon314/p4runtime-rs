#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Oneof)]
pub enum Value {
    #[prost(string, tag = "1")]
    StringValue(::prost::alloc::string::String),
    #[prost(int64, tag = "2")]
    Int64Value(i64),
    #[prost(bool, tag = "3")]
    BoolValue(bool),
}
