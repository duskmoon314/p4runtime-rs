#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Member {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "3")]
    pub annotations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[doc = " Optional. If present, the location of `annotations\\[i\\]` is given by"]
    #[doc = " `annotation_locations\\[i\\]`."]
    #[prost(message, repeated, tag = "5")]
    pub annotation_locations: ::prost::alloc::vec::Vec<super::SourceLocation>,
    #[prost(message, repeated, tag = "4")]
    pub structured_annotations: ::prost::alloc::vec::Vec<super::StructuredAnnotation>,
}
