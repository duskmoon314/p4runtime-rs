#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Oneof)]
pub enum Data {
    #[doc = " for bit<W>, int<W>"]
    #[prost(bytes, tag = "1")]
    Bitstring(::prost::alloc::vec::Vec<u8>),
    #[doc = " for varbit<W>"]
    #[prost(message, tag = "2")]
    Varbit(super::P4Varbit),
    #[prost(bool, tag = "3")]
    Bool(bool),
    #[prost(message, tag = "4")]
    Tuple(super::P4StructLike),
    #[prost(message, tag = "5")]
    Struct(super::P4StructLike),
    #[prost(message, tag = "6")]
    Header(super::P4Header),
    #[prost(message, tag = "7")]
    HeaderUnion(super::P4HeaderUnion),
    #[prost(message, tag = "8")]
    HeaderStack(super::P4HeaderStack),
    #[prost(message, tag = "9")]
    HeaderUnionStack(super::P4HeaderUnionStack),
    #[doc = " safe (non-serializable) enums only"]
    #[prost(string, tag = "10")]
    Enum(::prost::alloc::string::String),
    #[prost(string, tag = "11")]
    Error(::prost::alloc::string::String),
    #[doc = " serializable enums only"]
    #[prost(bytes, tag = "12")]
    EnumValue(::prost::alloc::vec::Vec<u8>),
}
