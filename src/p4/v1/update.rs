#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, :: prost :: Enumeration)]
#[repr(i32)]
pub enum Type {
    Unspecified = 0,
    Insert = 1,
    Modify = 2,
    Delete = 3,
}
impl Type {
    #[doc = " String value of the enum field names used in the ProtoBuf definition."]
    #[doc = ""]
    #[doc = " The values are not transformed in any way and thus are considered stable"]
    #[doc = " (if the ProtoBuf definition does not change) and safe for programmatic use."]
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Type::Unspecified => "UNSPECIFIED",
            Type::Insert => "INSERT",
            Type::Modify => "MODIFY",
            Type::Delete => "DELETE",
        }
    }
    #[doc = " Creates an enum from field names used in the ProtoBuf definition."]
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "INSERT" => Some(Self::Insert),
            "MODIFY" => Some(Self::Modify),
            "DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
