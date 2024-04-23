#[doc = " Metadata (cookie) opaque to the target. A control plane may use this field"]
#[doc = " to uniquely identify this config. There are no restrictions on how such"]
#[doc = " value is computed, or where this is stored on the target, as long as it is"]
#[doc = " returned with a GetForwardingPipelineConfig RPC. When reading the cookie,"]
#[doc = " we need to distinguish those cases where a cookie is NOT present (e.g. not"]
#[doc = " set in the SetForwardingPipelineConfigRequest, therefore we wrap the actual"]
#[doc = " uint64 value in a protobuf message."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Cookie {
    #[prost(uint64, tag = "1")]
    pub cookie: u64,
}
