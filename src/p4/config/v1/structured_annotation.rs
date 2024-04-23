#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Oneof)]
pub enum Body {
    #[prost(message, tag = "2")]
    ExpressionList(super::ExpressionList),
    #[prost(message, tag = "3")]
    KvPairList(super::KeyValuePairList),
}
