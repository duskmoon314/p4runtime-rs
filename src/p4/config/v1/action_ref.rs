#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, :: prost :: Enumeration)]
#[repr(i32)]
pub enum Scope {
    TableAndDefault = 0,
    TableOnly = 1,
    DefaultOnly = 2,
}
impl Scope {
    #[doc = " String value of the enum field names used in the ProtoBuf definition."]
    #[doc = ""]
    #[doc = " The values are not transformed in any way and thus are considered stable"]
    #[doc = " (if the ProtoBuf definition does not change) and safe for programmatic use."]
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Scope::TableAndDefault => "TABLE_AND_DEFAULT",
            Scope::TableOnly => "TABLE_ONLY",
            Scope::DefaultOnly => "DEFAULT_ONLY",
        }
    }
    #[doc = " Creates an enum from field names used in the ProtoBuf definition."]
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TABLE_AND_DEFAULT" => Some(Self::TableAndDefault),
            "TABLE_ONLY" => Some(Self::TableOnly),
            "DEFAULT_ONLY" => Some(Self::DefaultOnly),
            _ => None,
        }
    }
}
