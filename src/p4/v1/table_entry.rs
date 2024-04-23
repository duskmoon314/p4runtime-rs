#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct IdleTimeout {
    #[doc = " Time elapsed - in nanoseconds - since the table entry was last \"hit\" as"]
    #[doc = " part of a data plane table lookup."]
    #[prost(int64, tag = "1")]
    pub elapsed_ns: i64,
}
