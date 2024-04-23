#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, :: prost :: Enumeration)]
#[repr(i32)]
pub enum Atomicity {
    #[doc = " Required. This is the default behavior. The batch is processed in a"]
    #[doc = " non-atomic manner from a data plane point of view. Each operation within"]
    #[doc = " the batch must be attempted even if one or more encounter errors."]
    #[doc = " Every data plane packet is guaranteed to be processed according to"]
    #[doc = " table contents as they are between two individual operations of the"]
    #[doc = " batch, but there could be several packets processed that see each of"]
    #[doc = " these intermediate stages."]
    ContinueOnError = 0,
    #[doc = " Optional. Operations within the batch are committed to data plane until"]
    #[doc = " an error is encountered. At this point, the operations must be rolled"]
    #[doc = " back such that both software and data plane state is consistent with the"]
    #[doc = " state before the batch was attempted. The resulting behavior is"]
    #[doc = " all-or-none, except the batch is not atomic from a data plane point of"]
    #[doc = " view. Every data plane packet is guaranteed to be processed according to"]
    #[doc = " table contents as they are between two individual operations of the"]
    #[doc = " batch, but there could be several packets processed that see each of"]
    #[doc = " these intermediate stages."]
    RollbackOnError = 1,
    #[doc = " Optional. Every data plane packet is guaranteed to be processed according"]
    #[doc = " to table contents before the batch began, or after the batch completed"]
    #[doc = " and the operations were programmed to the hardware."]
    #[doc = " The batch is therefore treated as a transaction."]
    DataplaneAtomic = 2,
}
impl Atomicity {
    #[doc = " String value of the enum field names used in the ProtoBuf definition."]
    #[doc = ""]
    #[doc = " The values are not transformed in any way and thus are considered stable"]
    #[doc = " (if the ProtoBuf definition does not change) and safe for programmatic use."]
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Atomicity::ContinueOnError => "CONTINUE_ON_ERROR",
            Atomicity::RollbackOnError => "ROLLBACK_ON_ERROR",
            Atomicity::DataplaneAtomic => "DATAPLANE_ATOMIC",
        }
    }
    #[doc = " Creates an enum from field names used in the ProtoBuf definition."]
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONTINUE_ON_ERROR" => Some(Self::ContinueOnError),
            "ROLLBACK_ON_ERROR" => Some(Self::RollbackOnError),
            "DATAPLANE_ATOMIC" => Some(Self::DataplaneAtomic),
            _ => None,
        }
    }
}
