#[doc = " a DigestList message is streamed when the following conditions are met:"]
#[doc = "    - there is at least one digest ready"]
#[doc = "    - the oldest digest in the list has been waiting for at least"]
#[doc = "      max_timeout_ns nanoseconds or we have gathered max_list_size digests"]
#[doc = "      already"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Config {
    #[doc = " max timeout for outstanding digest data"]
    #[prost(int64, tag = "1")]
    pub max_timeout_ns: i64,
    #[doc = " max size for a digest list"]
    #[prost(int32, tag = "2")]
    pub max_list_size: i32,
    #[doc = " timeout for DigestListAck message"]
    #[prost(int64, tag = "3")]
    pub ack_timeout_ns: i64,
}
