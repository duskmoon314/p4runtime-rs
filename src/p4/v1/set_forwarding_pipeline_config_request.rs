#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, :: prost :: Enumeration)]
#[repr(i32)]
pub enum Action {
    Unspecified = 0,
    #[doc = " Verify that the target can realize the given config. Do not modify the"]
    #[doc = " forwarding state in the target. Returns error if config is not provided"]
    #[doc = " of if the provided config cannot be realized."]
    Verify = 1,
    #[doc = " Save the config if the target can realize it. Do not modify the"]
    #[doc = " forwarding state in the target. Any subsequent read/write requests must"]
    #[doc = " refer to fields in the new config. Returns error if config is not"]
    #[doc = " provided of if the provided config cannot be realized."]
    VerifyAndSave = 2,
    #[doc = " Verify, save and realize the given config. Clear the forwarding state"]
    #[doc = " in the target. Returns error if config is not provided of if the"]
    #[doc = " provided config cannot be realized."]
    VerifyAndCommit = 3,
    #[doc = " Realize the last saved, but not yet committed, config. Update the"]
    #[doc = " forwarding state in the target by replaying the write requests since the"]
    #[doc = " last config was saved. Config should not be provided for this action"]
    #[doc = " type. Returns an error if no saved config is found or if a config is"]
    #[doc = " provided with this message."]
    Commit = 4,
    #[doc = " Verify, save and realize the given config, while preserving the"]
    #[doc = " forwarding state in the target. This is an advanced use case to enable"]
    #[doc = " changes to the P4 forwarding pipeline configuration with minimal traffic"]
    #[doc = " loss. P4Runtime does not impose any constraints on the duration of the"]
    #[doc = " traffic loss. The support for this option is not expected to be uniform"]
    #[doc = " across all P4Runtime targets. A target that does not support this option"]
    #[doc = " may return an UNIMPLEMENTED error. For targets that support this option,"]
    #[doc = " an INVALID_ARGUMENT error is returned if no config is provided, or if"]
    #[doc = " the existing forwarding state cannot be preserved for the given config"]
    #[doc = " by the target."]
    ReconcileAndCommit = 5,
}
impl Action {
    #[doc = " String value of the enum field names used in the ProtoBuf definition."]
    #[doc = ""]
    #[doc = " The values are not transformed in any way and thus are considered stable"]
    #[doc = " (if the ProtoBuf definition does not change) and safe for programmatic use."]
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Action::Unspecified => "UNSPECIFIED",
            Action::Verify => "VERIFY",
            Action::VerifyAndSave => "VERIFY_AND_SAVE",
            Action::VerifyAndCommit => "VERIFY_AND_COMMIT",
            Action::Commit => "COMMIT",
            Action::ReconcileAndCommit => "RECONCILE_AND_COMMIT",
        }
    }
    #[doc = " Creates an enum from field names used in the ProtoBuf definition."]
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "VERIFY" => Some(Self::Verify),
            "VERIFY_AND_SAVE" => Some(Self::VerifyAndSave),
            "VERIFY_AND_COMMIT" => Some(Self::VerifyAndCommit),
            "COMMIT" => Some(Self::Commit),
            "RECONCILE_AND_COMMIT" => Some(Self::ReconcileAndCommit),
            _ => None,
        }
    }
}
