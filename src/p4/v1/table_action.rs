#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Oneof)]
pub enum Type {
    #[prost(message, tag = "1")]
    Action(super::Action),
    #[prost(uint32, tag = "2")]
    ActionProfileMemberId(u32),
    #[prost(uint32, tag = "3")]
    ActionProfileGroupId(u32),
    #[prost(message, tag = "4")]
    ActionProfileActionSet(super::ActionProfileActionSet),
}
