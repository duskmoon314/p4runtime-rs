#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Oneof)]
pub enum WatchKind {
    #[doc = " Using int32 as ports is deprecated, use watch_port instead."]
    #[prost(int32, tag = "3")]
    Watch(i32),
    #[prost(bytes, tag = "4")]
    WatchPort(::prost::alloc::vec::Vec<u8>),
}
