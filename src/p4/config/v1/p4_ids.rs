#[doc = " ids are allocated in such a way that it is possible based on an id to"]
#[doc = " deduce the resource type (e.g. table, action, counter, ...). The"]
#[doc = " most-significant byte of the 32-bit id encodes the resource type. The"]
#[doc = " purpose of this enum is to define which value is used as the"]
#[doc = " most-significant byte for each resource type. The P4 compiler must use"]
#[doc = " these values when allocating ids for P4 objects. Other users of P4Info can"]
#[doc = " refer to this enum to identify a resource type based on its id."]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, :: prost :: Enumeration)]
#[repr(i32)]
pub enum Prefix {
    Unspecified = 0,
    #[doc = " P4 language built-ins"]
    Action = 1,
    Table = 2,
    ValueSet = 3,
    ControllerHeader = 4,
    #[doc = " PSA externs"]
    PsaExternsStart = 16,
    ActionProfile = 17,
    Counter = 18,
    DirectCounter = 19,
    Meter = 20,
    DirectMeter = 21,
    Register = 22,
    Digest = 23,
    #[doc = " externs for other architectures (vendor extensions)"]
    OtherExternsStart = 128,
    #[doc = " max value for an unsigned 8-bit byte"]
    #[doc = ""]
    #[doc = " requires protoc >= 3.5.0"]
    #[doc = " reserved 0x100 to max;"]
    Max = 255,
}
impl Prefix {
    #[doc = " String value of the enum field names used in the ProtoBuf definition."]
    #[doc = ""]
    #[doc = " The values are not transformed in any way and thus are considered stable"]
    #[doc = " (if the ProtoBuf definition does not change) and safe for programmatic use."]
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Prefix::Unspecified => "UNSPECIFIED",
            Prefix::Action => "ACTION",
            Prefix::Table => "TABLE",
            Prefix::ValueSet => "VALUE_SET",
            Prefix::ControllerHeader => "CONTROLLER_HEADER",
            Prefix::PsaExternsStart => "PSA_EXTERNS_START",
            Prefix::ActionProfile => "ACTION_PROFILE",
            Prefix::Counter => "COUNTER",
            Prefix::DirectCounter => "DIRECT_COUNTER",
            Prefix::Meter => "METER",
            Prefix::DirectMeter => "DIRECT_METER",
            Prefix::Register => "REGISTER",
            Prefix::Digest => "DIGEST",
            Prefix::OtherExternsStart => "OTHER_EXTERNS_START",
            Prefix::Max => "MAX",
        }
    }
    #[doc = " Creates an enum from field names used in the ProtoBuf definition."]
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "ACTION" => Some(Self::Action),
            "TABLE" => Some(Self::Table),
            "VALUE_SET" => Some(Self::ValueSet),
            "CONTROLLER_HEADER" => Some(Self::ControllerHeader),
            "PSA_EXTERNS_START" => Some(Self::PsaExternsStart),
            "ACTION_PROFILE" => Some(Self::ActionProfile),
            "COUNTER" => Some(Self::Counter),
            "DIRECT_COUNTER" => Some(Self::DirectCounter),
            "METER" => Some(Self::Meter),
            "DIRECT_METER" => Some(Self::DirectMeter),
            "REGISTER" => Some(Self::Register),
            "DIGEST" => Some(Self::Digest),
            "OTHER_EXTERNS_START" => Some(Self::OtherExternsStart),
            "MAX" => Some(Self::Max),
            _ => None,
        }
    }
}
