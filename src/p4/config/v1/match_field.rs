#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, :: prost :: Enumeration)]
#[repr(i32)]
pub enum MatchType {
    Unspecified = 0,
    Exact = 2,
    Lpm = 3,
    Ternary = 4,
    Range = 5,
    Optional = 6,
}
impl MatchType {
    #[doc = " String value of the enum field names used in the ProtoBuf definition."]
    #[doc = ""]
    #[doc = " The values are not transformed in any way and thus are considered stable"]
    #[doc = " (if the ProtoBuf definition does not change) and safe for programmatic use."]
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MatchType::Unspecified => "UNSPECIFIED",
            MatchType::Exact => "EXACT",
            MatchType::Lpm => "LPM",
            MatchType::Ternary => "TERNARY",
            MatchType::Range => "RANGE",
            MatchType::Optional => "OPTIONAL",
        }
    }
    #[doc = " Creates an enum from field names used in the ProtoBuf definition."]
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "EXACT" => Some(Self::Exact),
            "LPM" => Some(Self::Lpm),
            "TERNARY" => Some(Self::Ternary),
            "RANGE" => Some(Self::Range),
            "OPTIONAL" => Some(Self::Optional),
            _ => None,
        }
    }
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Oneof)]
pub enum Match {
    #[prost(enumeration = "MatchType", tag = "5")]
    MatchType(i32),
    #[doc = " used for architecture-specific match types which are not part of the core"]
    #[doc = " P4 language or of the PSA architecture."]
    #[prost(string, tag = "7")]
    OtherMatchType(::prost::alloc::string::String),
}
