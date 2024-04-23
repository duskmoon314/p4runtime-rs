#[doc = " Corresponds to 'type' constructor parameter for Meter / DirectMeter in PSA"]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, :: prost :: Enumeration)]
#[repr(i32)]
pub enum Unit {
    Unspecified = 0,
    Bytes = 1,
    Packets = 2,
}
impl Unit {
    #[doc = " String value of the enum field names used in the ProtoBuf definition."]
    #[doc = ""]
    #[doc = " The values are not transformed in any way and thus are considered stable"]
    #[doc = " (if the ProtoBuf definition does not change) and safe for programmatic use."]
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Unit::Unspecified => "UNSPECIFIED",
            Unit::Bytes => "BYTES",
            Unit::Packets => "PACKETS",
        }
    }
    #[doc = " Creates an enum from field names used in the ProtoBuf definition."]
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "BYTES" => Some(Self::Bytes),
            "PACKETS" => Some(Self::Packets),
            _ => None,
        }
    }
}
