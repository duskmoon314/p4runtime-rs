#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Member {
    #[prost(uint32, tag = "1")]
    pub member_id: u32,
    #[prost(int32, tag = "2")]
    pub weight: i32,
    #[prost(oneof = "member::WatchKind", tags = "3, 4")]
    pub watch_kind: ::core::option::Option<member::WatchKind>,
}
#[doc = " Nested message and enum types in `Member`."]
pub mod member;
