#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct SdnString {}
#[doc = " The object is either represented as an unsigned integer with a bitwidth of"]
#[doc = " `sdn_bitwidth`, or as a string."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Oneof)]
pub enum SdnType {
    #[prost(int32, tag = "2")]
    SdnBitwidth(i32),
    #[prost(message, tag = "3")]
    SdnString(SdnString),
}
