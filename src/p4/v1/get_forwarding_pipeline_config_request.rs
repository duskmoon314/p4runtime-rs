#[doc = " Specifies the fields to populate in the response."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, :: prost :: Enumeration)]
#[repr(i32)]
pub enum ResponseType {
    #[doc = " Default behaviour. Returns a ForwardingPipelineConfig with all fields set"]
    #[doc = " as stored by the target."]
    All = 0,
    #[doc = " Reply by setting only the cookie field, omitting all other fields."]
    CookieOnly = 1,
    #[doc = " Reply by setting the p4info and cookie fields."]
    P4infoAndCookie = 2,
    #[doc = " Reply by setting the p4_device_config and cookie fields."]
    DeviceConfigAndCookie = 3,
}
impl ResponseType {
    #[doc = " String value of the enum field names used in the ProtoBuf definition."]
    #[doc = ""]
    #[doc = " The values are not transformed in any way and thus are considered stable"]
    #[doc = " (if the ProtoBuf definition does not change) and safe for programmatic use."]
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResponseType::All => "ALL",
            ResponseType::CookieOnly => "COOKIE_ONLY",
            ResponseType::P4infoAndCookie => "P4INFO_AND_COOKIE",
            ResponseType::DeviceConfigAndCookie => "DEVICE_CONFIG_AND_COOKIE",
        }
    }
    #[doc = " Creates an enum from field names used in the ProtoBuf definition."]
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ALL" => Some(Self::All),
            "COOKIE_ONLY" => Some(Self::CookieOnly),
            "P4INFO_AND_COOKIE" => Some(Self::P4infoAndCookie),
            "DEVICE_CONFIG_AND_COOKIE" => Some(Self::DeviceConfigAndCookie),
            _ => None,
        }
    }
}
