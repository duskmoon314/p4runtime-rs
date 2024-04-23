#[doc = " Matches can be performed on arbitrarily-large inputs; the protobuf type"]
#[doc = " 'bytes' is used to model arbitrarily-large values."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Exact {
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Ternary {
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub mask: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Lpm {
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[doc = " in bits"]
    #[prost(int32, tag = "2")]
    pub prefix_len: i32,
}
#[doc = " A Range is logically a set that contains all values numerically between"]
#[doc = " 'low' and 'high' inclusively."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Range {
    #[prost(bytes = "vec", tag = "1")]
    pub low: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub high: ::prost::alloc::vec::Vec<u8>,
}
#[doc = " If the Optional match should be a wildcard, the FieldMatch must be omitted."]
#[doc = " Otherwise, this behaves like an exact match."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Optional {
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Oneof)]
pub enum FieldMatchType {
    #[prost(message, tag = "2")]
    Exact(Exact),
    #[prost(message, tag = "3")]
    Ternary(Ternary),
    #[prost(message, tag = "4")]
    Lpm(Lpm),
    #[prost(message, tag = "6")]
    Range(Range),
    #[prost(message, tag = "7")]
    Optional(Optional),
    #[doc = " Architecture-specific match value; it corresponds to the other_match_type"]
    #[doc = " in the P4Info MatchField message."]
    #[prost(message, tag = "100")]
    Other(::prost_types::Any),
}
