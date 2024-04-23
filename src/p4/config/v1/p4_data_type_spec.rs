#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Oneof)]
pub enum TypeSpec {
    #[prost(message, tag = "1")]
    Bitstring(super::P4BitstringLikeTypeSpec),
    #[prost(message, tag = "2")]
    Bool(super::P4BoolType),
    #[prost(message, tag = "3")]
    Tuple(super::P4TupleTypeSpec),
    #[prost(message, tag = "4")]
    Struct(super::P4NamedType),
    #[prost(message, tag = "5")]
    Header(super::P4NamedType),
    #[prost(message, tag = "6")]
    HeaderUnion(super::P4NamedType),
    #[prost(message, tag = "7")]
    HeaderStack(super::P4HeaderStackTypeSpec),
    #[prost(message, tag = "8")]
    HeaderUnionStack(super::P4HeaderUnionStackTypeSpec),
    #[prost(message, tag = "9")]
    Enum(super::P4NamedType),
    #[prost(message, tag = "10")]
    Error(super::P4ErrorType),
    #[prost(message, tag = "11")]
    SerializableEnum(super::P4NamedType),
    #[prost(message, tag = "12")]
    NewType(super::P4NamedType),
}
