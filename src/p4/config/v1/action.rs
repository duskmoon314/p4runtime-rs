#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Param {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub annotations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[doc = " Optional. If present, the location of `annotations\\[i\\]` is given by"]
    #[doc = " `annotation_locations\\[i\\]`."]
    #[prost(message, repeated, tag = "8")]
    pub annotation_locations: ::prost::alloc::vec::Vec<super::SourceLocation>,
    #[prost(int32, tag = "4")]
    pub bitwidth: i32,
    #[doc = " Documentation of the Param"]
    #[prost(message, optional, tag = "5")]
    pub doc: ::core::option::Option<super::Documentation>,
    #[doc = " unset if not user-defined type"]
    #[prost(message, optional, tag = "6")]
    pub type_name: ::core::option::Option<super::P4NamedType>,
    #[prost(message, repeated, tag = "7")]
    pub structured_annotations: ::prost::alloc::vec::Vec<super::StructuredAnnotation>,
}
