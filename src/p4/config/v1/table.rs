#[doc = " this enum can be extended in the future with other behaviors, such as"]
#[doc = " \"HARD_EVICTION\""]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, :: prost :: Enumeration)]
#[repr(i32)]
pub enum IdleTimeoutBehavior {
    NoTimeout = 0,
    NotifyControl = 1,
}
impl IdleTimeoutBehavior {
    #[doc = " String value of the enum field names used in the ProtoBuf definition."]
    #[doc = ""]
    #[doc = " The values are not transformed in any way and thus are considered stable"]
    #[doc = " (if the ProtoBuf definition does not change) and safe for programmatic use."]
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IdleTimeoutBehavior::NoTimeout => "NO_TIMEOUT",
            IdleTimeoutBehavior::NotifyControl => "NOTIFY_CONTROL",
        }
    }
    #[doc = " Creates an enum from field names used in the ProtoBuf definition."]
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NO_TIMEOUT" => Some(Self::NoTimeout),
            "NOTIFY_CONTROL" => Some(Self::NotifyControl),
            _ => None,
        }
    }
}
