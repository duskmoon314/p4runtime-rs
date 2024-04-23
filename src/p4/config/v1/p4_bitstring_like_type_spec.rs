#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Oneof)]
pub enum TypeSpec {
    #[doc = " bit<W>"]
    #[prost(message, tag = "1")]
    Bit(super::P4BitTypeSpec),
    #[doc = " int<W>"]
    #[prost(message, tag = "2")]
    Int(super::P4IntTypeSpec),
    #[doc = " varbit<W>"]
    #[prost(message, tag = "3")]
    Varbit(super::P4VarbitTypeSpec),
}
