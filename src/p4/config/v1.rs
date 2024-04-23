#[doc = " These P4 types (struct, header_type, header_union and enum) are guaranteed to"]
#[doc = " have a fully-qualified name (e.g. you cannot use an anonymous struct to"]
#[doc = " declare a variable like in C). Instead of duplicating the type spec for these"]
#[doc = " every time the type is used, we include the type spec once in this P4TypeInfo"]
#[doc = " message and refer to the types by name in the P4DataTypeSpec message. We also"]
#[doc = " support annotations for these type specs which can be useful, e.g. to"]
#[doc = " identify well-known headers (such as ipv4)."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4TypeInfo {
    #[prost(map = "string, message", tag = "1")]
    pub structs: ::std::collections::HashMap<::prost::alloc::string::String, P4StructTypeSpec>,
    #[prost(map = "string, message", tag = "2")]
    pub headers: ::std::collections::HashMap<::prost::alloc::string::String, P4HeaderTypeSpec>,
    #[prost(map = "string, message", tag = "3")]
    pub header_unions:
        ::std::collections::HashMap<::prost::alloc::string::String, P4HeaderUnionTypeSpec>,
    #[prost(map = "string, message", tag = "4")]
    pub enums: ::std::collections::HashMap<::prost::alloc::string::String, P4EnumTypeSpec>,
    #[prost(message, optional, tag = "5")]
    pub error: ::core::option::Option<P4ErrorTypeSpec>,
    #[prost(map = "string, message", tag = "6")]
    pub serializable_enums:
        ::std::collections::HashMap<::prost::alloc::string::String, P4SerializableEnumTypeSpec>,
    #[prost(map = "string, message", tag = "7")]
    pub new_types: ::std::collections::HashMap<::prost::alloc::string::String, P4NewTypeSpec>,
}
#[doc = " Describes a P4_16 type."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4DataTypeSpec {
    #[prost(
        oneof = "p4_data_type_spec::TypeSpec",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12"
    )]
    pub type_spec: ::core::option::Option<p4_data_type_spec::TypeSpec>,
}
#[doc = " Nested message and enum types in `P4DataTypeSpec`."]
pub mod p4_data_type_spec;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4NamedType {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = " Empty message as no type information needed, just used as a placeholder in"]
#[doc = " the oneof to identify boolean types."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4BoolType {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4ErrorType {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4BitstringLikeTypeSpec {
    #[doc = " Useful to identify well-known types, such as IP address or Ethernet MAC"]
    #[doc = " address."]
    #[prost(string, repeated, tag = "4")]
    pub annotations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[doc = " Optional. If present, the location of `annotations\\[i\\]` is given by"]
    #[doc = " `annotation_locations\\[i\\]`."]
    #[prost(message, repeated, tag = "5")]
    pub annotation_locations: ::prost::alloc::vec::Vec<SourceLocation>,
    #[prost(message, repeated, tag = "6")]
    pub structured_annotations: ::prost::alloc::vec::Vec<StructuredAnnotation>,
    #[prost(oneof = "p4_bitstring_like_type_spec::TypeSpec", tags = "1, 2, 3")]
    pub type_spec: ::core::option::Option<p4_bitstring_like_type_spec::TypeSpec>,
}
#[doc = " Nested message and enum types in `P4BitstringLikeTypeSpec`."]
pub mod p4_bitstring_like_type_spec;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4BitTypeSpec {
    #[prost(int32, tag = "1")]
    pub bitwidth: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4IntTypeSpec {
    #[prost(int32, tag = "1")]
    pub bitwidth: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4VarbitTypeSpec {
    #[prost(int32, tag = "1")]
    pub max_bitwidth: i32,
}
#[doc = " From the P4_16 spec: \"A tuple is similar to a struct, in that it holds"]
#[doc = " multiple values. Unlike a struct type, tuples have no named fields.\""]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4TupleTypeSpec {
    #[prost(message, repeated, tag = "1")]
    pub members: ::prost::alloc::vec::Vec<P4DataTypeSpec>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4StructTypeSpec {
    #[prost(message, repeated, tag = "1")]
    pub members: ::prost::alloc::vec::Vec<p4_struct_type_spec::Member>,
    #[prost(string, repeated, tag = "2")]
    pub annotations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[doc = " Optional. If present, the location of `annotations\\[i\\]` is given by"]
    #[doc = " `annotation_locations\\[i\\]`."]
    #[prost(message, repeated, tag = "3")]
    pub annotation_locations: ::prost::alloc::vec::Vec<SourceLocation>,
    #[prost(message, repeated, tag = "4")]
    pub structured_annotations: ::prost::alloc::vec::Vec<StructuredAnnotation>,
}
#[doc = " Nested message and enum types in `P4StructTypeSpec`."]
pub mod p4_struct_type_spec;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4HeaderTypeSpec {
    #[prost(message, repeated, tag = "1")]
    pub members: ::prost::alloc::vec::Vec<p4_header_type_spec::Member>,
    #[prost(string, repeated, tag = "2")]
    pub annotations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[doc = " Optional. If present, the location of `annotations\\[i\\]` is given by"]
    #[doc = " `annotation_locations\\[i\\]`."]
    #[prost(message, repeated, tag = "3")]
    pub annotation_locations: ::prost::alloc::vec::Vec<SourceLocation>,
    #[prost(message, repeated, tag = "4")]
    pub structured_annotations: ::prost::alloc::vec::Vec<StructuredAnnotation>,
}
#[doc = " Nested message and enum types in `P4HeaderTypeSpec`."]
pub mod p4_header_type_spec;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4HeaderUnionTypeSpec {
    #[prost(message, repeated, tag = "1")]
    pub members: ::prost::alloc::vec::Vec<p4_header_union_type_spec::Member>,
    #[prost(string, repeated, tag = "2")]
    pub annotations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[doc = " Optional. If present, the location of `annotations\\[i\\]` is given by"]
    #[doc = " `annotation_locations\\[i\\]`."]
    #[prost(message, repeated, tag = "3")]
    pub annotation_locations: ::prost::alloc::vec::Vec<SourceLocation>,
    #[prost(message, repeated, tag = "4")]
    pub structured_annotations: ::prost::alloc::vec::Vec<StructuredAnnotation>,
}
#[doc = " Nested message and enum types in `P4HeaderUnionTypeSpec`."]
pub mod p4_header_union_type_spec;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4HeaderStackTypeSpec {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<P4NamedType>,
    #[prost(int32, tag = "2")]
    pub size: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4HeaderUnionStackTypeSpec {
    #[prost(message, optional, tag = "1")]
    pub header_union: ::core::option::Option<P4NamedType>,
    #[prost(int32, tag = "2")]
    pub size: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct KeyValuePair {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Expression>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct KeyValuePairList {
    #[prost(message, repeated, tag = "1")]
    pub kv_pairs: ::prost::alloc::vec::Vec<KeyValuePair>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Expression {
    #[prost(oneof = "expression::Value", tags = "1, 2, 3")]
    pub value: ::core::option::Option<expression::Value>,
}
#[doc = " Nested message and enum types in `Expression`."]
pub mod expression;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ExpressionList {
    #[prost(message, repeated, tag = "1")]
    pub expressions: ::prost::alloc::vec::Vec<Expression>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct StructuredAnnotation {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[doc = " Optional. Location of the '@' symbol of this annotation in the source code."]
    #[prost(message, optional, tag = "4")]
    pub source_location: ::core::option::Option<SourceLocation>,
    #[prost(oneof = "structured_annotation::Body", tags = "2, 3")]
    pub body: ::core::option::Option<structured_annotation::Body>,
}
#[doc = " Nested message and enum types in `StructuredAnnotation`."]
pub mod structured_annotation;
#[doc = " Location of code relative to a given source file."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct SourceLocation {
    #[doc = " Path to the source file (absolute or relative to the working directory)."]
    #[prost(string, tag = "1")]
    pub file: ::prost::alloc::string::String,
    #[doc = " Line and column numbers within the source file, 1-based."]
    #[prost(int32, tag = "2")]
    pub line: i32,
    #[prost(int32, tag = "3")]
    pub column: i32,
}
#[doc = " For \"safe\" enums with no underlying representation and no member integer"]
#[doc = " values."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4EnumTypeSpec {
    #[prost(message, repeated, tag = "1")]
    pub members: ::prost::alloc::vec::Vec<p4_enum_type_spec::Member>,
    #[prost(string, repeated, tag = "2")]
    pub annotations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[doc = " Optional. If present, the location of `annotations\\[i\\]` is given by"]
    #[doc = " `annotation_locations\\[i\\]`."]
    #[prost(message, repeated, tag = "4")]
    pub annotation_locations: ::prost::alloc::vec::Vec<SourceLocation>,
    #[prost(message, repeated, tag = "3")]
    pub structured_annotations: ::prost::alloc::vec::Vec<StructuredAnnotation>,
}
#[doc = " Nested message and enum types in `P4EnumTypeSpec`."]
pub mod p4_enum_type_spec;
#[doc = " For serializable (or \"unsafe\") enums, which have an underlying type. Note"]
#[doc = " that as per the P4_16 specification, the underlying representation can only"]
#[doc = " be a bit<W> type."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4SerializableEnumTypeSpec {
    #[prost(message, optional, tag = "1")]
    pub underlying_type: ::core::option::Option<P4BitTypeSpec>,
    #[prost(message, repeated, tag = "2")]
    pub members: ::prost::alloc::vec::Vec<p4_serializable_enum_type_spec::Member>,
    #[prost(string, repeated, tag = "3")]
    pub annotations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[doc = " Optional. If present, the location of `annotations\\[i\\]` is given by"]
    #[doc = " `annotation_locations\\[i\\]`."]
    #[prost(message, repeated, tag = "5")]
    pub annotation_locations: ::prost::alloc::vec::Vec<SourceLocation>,
    #[prost(message, repeated, tag = "4")]
    pub structured_annotations: ::prost::alloc::vec::Vec<StructuredAnnotation>,
}
#[doc = " Nested message and enum types in `P4SerializableEnumTypeSpec`."]
pub mod p4_serializable_enum_type_spec;
#[doc = " Similar to an enum, but there is always one and only one instance per P4"]
#[doc = " program."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4ErrorTypeSpec {
    #[prost(string, repeated, tag = "1")]
    pub members: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4NewTypeTranslation {
    #[doc = " the URI uniquely identifies the translation in order to enable the"]
    #[doc = " P4Runtime agent to perform value-mapping appropriately when required. It is"]
    #[doc = " recommended that the URI includes at least the P4 architecture name and the"]
    #[doc = " type name."]
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[doc = " The object is either represented as an unsigned integer with a bitwidth of"]
    #[doc = " `sdn_bitwidth`, or as a string."]
    #[prost(oneof = "p4_new_type_translation::SdnType", tags = "2, 3")]
    pub sdn_type: ::core::option::Option<p4_new_type_translation::SdnType>,
}
#[doc = " Nested message and enum types in `P4NewTypeTranslation`."]
pub mod p4_new_type_translation;
#[doc = " New types introduced with the \"type\" keyword"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4NewTypeSpec {
    #[doc = " for other annotations (not @p4runtime_translation)"]
    #[prost(string, repeated, tag = "3")]
    pub annotations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[doc = " Optional. If present, the location of `annotations\\[i\\]` is given by"]
    #[doc = " `annotation_locations\\[i\\]`."]
    #[prost(message, repeated, tag = "5")]
    pub annotation_locations: ::prost::alloc::vec::Vec<SourceLocation>,
    #[prost(message, repeated, tag = "4")]
    pub structured_annotations: ::prost::alloc::vec::Vec<StructuredAnnotation>,
    #[prost(oneof = "p4_new_type_spec::Representation", tags = "1, 2")]
    pub representation: ::core::option::Option<p4_new_type_spec::Representation>,
}
#[doc = " Nested message and enum types in `P4NewTypeSpec`."]
pub mod p4_new_type_spec;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4Info {
    #[prost(message, optional, tag = "1")]
    pub pkg_info: ::core::option::Option<PkgInfo>,
    #[prost(message, repeated, tag = "2")]
    pub tables: ::prost::alloc::vec::Vec<Table>,
    #[prost(message, repeated, tag = "3")]
    pub actions: ::prost::alloc::vec::Vec<Action>,
    #[prost(message, repeated, tag = "4")]
    pub action_profiles: ::prost::alloc::vec::Vec<ActionProfile>,
    #[prost(message, repeated, tag = "5")]
    pub counters: ::prost::alloc::vec::Vec<Counter>,
    #[prost(message, repeated, tag = "6")]
    pub direct_counters: ::prost::alloc::vec::Vec<DirectCounter>,
    #[prost(message, repeated, tag = "7")]
    pub meters: ::prost::alloc::vec::Vec<Meter>,
    #[prost(message, repeated, tag = "8")]
    pub direct_meters: ::prost::alloc::vec::Vec<DirectMeter>,
    #[prost(message, repeated, tag = "9")]
    pub controller_packet_metadata: ::prost::alloc::vec::Vec<ControllerPacketMetadata>,
    #[prost(message, repeated, tag = "10")]
    pub value_sets: ::prost::alloc::vec::Vec<ValueSet>,
    #[prost(message, repeated, tag = "11")]
    pub registers: ::prost::alloc::vec::Vec<Register>,
    #[prost(message, repeated, tag = "12")]
    pub digests: ::prost::alloc::vec::Vec<Digest>,
    #[prost(message, repeated, tag = "100")]
    pub externs: ::prost::alloc::vec::Vec<Extern>,
    #[prost(message, optional, tag = "200")]
    pub type_info: ::core::option::Option<P4TypeInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Documentation {
    #[doc = " A brief description of something, e.g. one sentence"]
    #[prost(string, tag = "1")]
    pub brief: ::prost::alloc::string::String,
    #[doc = " A more verbose description of something. Multiline is accepted. Markup"]
    #[doc = " format (if any) is TBD."]
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
#[doc = " Top-level package documentation describing the forwarding pipeline config"]
#[doc = " Can be used to manage multiple P4 packages."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct PkgInfo {
    #[doc = " a definitive name for this configuration, e.g. switch.p4_v1.0"]
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[doc = " configuration version, free-format string"]
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[doc = " brief and detailed descriptions"]
    #[prost(message, optional, tag = "3")]
    pub doc: ::core::option::Option<Documentation>,
    #[doc = " Miscellaneous metadata, free-form; a way to extend PkgInfo"]
    #[prost(string, repeated, tag = "4")]
    pub annotations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[doc = " Optional. If present, the location of `annotations\\[i\\]` is given by"]
    #[doc = " `annotation_locations\\[i\\]`."]
    #[prost(message, repeated, tag = "10")]
    pub annotation_locations: ::prost::alloc::vec::Vec<SourceLocation>,
    #[doc = " the target architecture, e.g. \"psa\""]
    #[prost(string, tag = "5")]
    pub arch: ::prost::alloc::string::String,
    #[doc = " organization which produced the configuration, e.g. \"p4.org\""]
    #[prost(string, tag = "6")]
    pub organization: ::prost::alloc::string::String,
    #[doc = " contact info for support,e.g. \"tech-support@acme.org\""]
    #[prost(string, tag = "7")]
    pub contact: ::prost::alloc::string::String,
    #[doc = " url for more information, e.g. \"<http://support.p4.org/ref/p4/switch.p4_v1.0\">"]
    #[prost(string, tag = "8")]
    pub url: ::prost::alloc::string::String,
    #[doc = " Miscellaneous metadata, structured; a way to extend PkgInfo"]
    #[prost(message, repeated, tag = "9")]
    pub structured_annotations: ::prost::alloc::vec::Vec<StructuredAnnotation>,
}
#[doc = " wrapping the enum in a message to avoid name collisions in C++, where \"enum"]
#[doc = " values are siblings of their type, not children of it\""]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4Ids {}
#[doc = " Nested message and enum types in `P4Ids`."]
pub mod p4_ids;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Preamble {
    #[doc = " ids share the same number-space; e.g. table ids cannot overlap with counter"]
    #[doc = " ids. Even though this is irrelevant to this proto definition, the ids are"]
    #[doc = " allocated in such a way that it is possible based on an id to deduce the"]
    #[doc = " resource type (e.g. table, action, counter, ...). This means that code"]
    #[doc = " using these ids can detect if the wrong resource type is used"]
    #[doc = " somewhere. This also means that ids of different types can be mixed"]
    #[doc = " (e.g. direct resource list for a table) without ambiguity. Note that id 0"]
    #[doc = " is reserved and means \"invalid id\"."]
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[doc = " fully qualified name of the P4 object, e.g. c1.c2.ipv4_lpm"]
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[doc = " an alias (alternative name) for the P4 object, probably shorter than its"]
    #[doc = " fully qualified name. The only constraint is for it to be unique with"]
    #[doc = " respect to other P4 objects of the same type. By default, the compiler uses"]
    #[doc = " the shortest suffix of the name that uniquely identifies the object. For"]
    #[doc = " example if the P4 program contains two tables with names s.c1.t and s.c2.t,"]
    #[doc = " the default aliases will respectively be c1.t and c2.t. In the future, the"]
    #[doc = " P4 programmer may also be able to override the default alias for any P4"]
    #[doc = " object (TBD)."]
    #[prost(string, tag = "3")]
    pub alias: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub annotations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[doc = " Optional. If present, the location of `annotations\\[i\\]` is given by"]
    #[doc = " `annotation_locations\\[i\\]`."]
    #[prost(message, repeated, tag = "7")]
    pub annotation_locations: ::prost::alloc::vec::Vec<SourceLocation>,
    #[doc = " Documentation of the entity"]
    #[prost(message, optional, tag = "5")]
    pub doc: ::core::option::Option<Documentation>,
    #[prost(message, repeated, tag = "6")]
    pub structured_annotations: ::prost::alloc::vec::Vec<StructuredAnnotation>,
}
#[doc = " used to group all extern instances of the same type in one message"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Extern {
    #[doc = " the extern_type_id is unique for a given architecture and must be in the"]
    #[doc = " range \\[0x81, 0xfe\\]."]
    #[prost(uint32, tag = "1")]
    pub extern_type_id: u32,
    #[prost(string, tag = "2")]
    pub extern_type_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub instances: ::prost::alloc::vec::Vec<ExternInstance>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ExternInstance {
    #[prost(message, optional, tag = "1")]
    pub preamble: ::core::option::Option<Preamble>,
    #[doc = " specific to the extern type, declared in a separate vendor-specific proto"]
    #[doc = " file"]
    #[prost(message, optional, tag = "2")]
    pub info: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct MatchField {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub annotations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[doc = " Optional. If present, the location of `annotations\\[i\\]` is given by"]
    #[doc = " `annotation_locations\\[i\\]`."]
    #[prost(message, repeated, tag = "10")]
    pub annotation_locations: ::prost::alloc::vec::Vec<SourceLocation>,
    #[prost(int32, tag = "4")]
    pub bitwidth: i32,
    #[doc = " Documentation of the match field"]
    #[prost(message, optional, tag = "6")]
    pub doc: ::core::option::Option<Documentation>,
    #[doc = " unset if not user-defined type"]
    #[prost(message, optional, tag = "8")]
    pub type_name: ::core::option::Option<P4NamedType>,
    #[prost(message, repeated, tag = "9")]
    pub structured_annotations: ::prost::alloc::vec::Vec<StructuredAnnotation>,
    #[prost(oneof = "match_field::Match", tags = "5, 7")]
    pub r#match: ::core::option::Option<match_field::Match>,
}
#[doc = " Nested message and enum types in `MatchField`."]
pub mod match_field;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Table {
    #[prost(message, optional, tag = "1")]
    pub preamble: ::core::option::Option<Preamble>,
    #[prost(message, repeated, tag = "2")]
    pub match_fields: ::prost::alloc::vec::Vec<MatchField>,
    #[doc = " even when the table is indirect (see implementation_id) below, this field"]
    #[doc = " includes all possible actions for the table; by using ActionRef instead of"]
    #[doc = " a repeated field of action ids, each action reference in a P4 table is able"]
    #[doc = " to have its own annotations"]
    #[prost(message, repeated, tag = "3")]
    pub action_refs: ::prost::alloc::vec::Vec<ActionRef>,
    #[doc = " 0 (default value) means that the table does not have a const default action"]
    #[prost(uint32, tag = "4")]
    pub const_default_action_id: u32,
    #[doc = " P4 id of the \"implementation\" for this table (e.g. action profile id); 0"]
    #[doc = " (default value) means that the table is a regular (direct) match table. As"]
    #[doc = " of today, only action profiles are supported but other table"]
    #[doc = " implementations may be added in the future"]
    #[prost(uint32, tag = "6")]
    pub implementation_id: u32,
    #[doc = " ids of the direct resources (if any) attached to this table; for now this"]
    #[doc = " includes only direct counters and direct meters, but other resources may be"]
    #[doc = " added in the future"]
    #[prost(uint32, repeated, tag = "7")]
    pub direct_resource_ids: ::prost::alloc::vec::Vec<u32>,
    #[doc = " max number of entries in table"]
    #[prost(int64, tag = "8")]
    pub size: i64,
    #[doc = " is idle timeout supported for this table?"]
    #[prost(enumeration = "table::IdleTimeoutBehavior", tag = "9")]
    pub idle_timeout_behavior: i32,
    #[doc = " table with static P4 entries, cannot be modified at runtime"]
    #[prost(bool, tag = "10")]
    pub is_const_table: bool,
    #[doc = " architecture-specific table properties which are not part of the core P4"]
    #[doc = " language or of the PSA architecture."]
    #[prost(message, optional, tag = "100")]
    pub other_properties: ::core::option::Option<::prost_types::Any>,
}
#[doc = " Nested message and enum types in `Table`."]
pub mod table;
#[doc = " used to list all possible actions in a Table"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ActionRef {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(enumeration = "action_ref::Scope", tag = "3")]
    pub scope: i32,
    #[prost(string, repeated, tag = "2")]
    pub annotations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[doc = " Optional. If present, the location of `annotations\\[i\\]` is given by"]
    #[doc = " `annotation_locations\\[i\\]`."]
    #[prost(message, repeated, tag = "5")]
    pub annotation_locations: ::prost::alloc::vec::Vec<SourceLocation>,
    #[prost(message, repeated, tag = "4")]
    pub structured_annotations: ::prost::alloc::vec::Vec<StructuredAnnotation>,
}
#[doc = " Nested message and enum types in `ActionRef`."]
pub mod action_ref;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Action {
    #[prost(message, optional, tag = "1")]
    pub preamble: ::core::option::Option<Preamble>,
    #[prost(message, repeated, tag = "2")]
    pub params: ::prost::alloc::vec::Vec<action::Param>,
}
#[doc = " Nested message and enum types in `Action`."]
pub mod action;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ActionProfile {
    #[prost(message, optional, tag = "1")]
    pub preamble: ::core::option::Option<Preamble>,
    #[doc = " the ids of the tables sharing this action profile"]
    #[prost(uint32, repeated, tag = "2")]
    pub table_ids: ::prost::alloc::vec::Vec<u32>,
    #[doc = " true iff the action profile used dynamic selection"]
    #[prost(bool, tag = "3")]
    pub with_selector: bool,
    #[doc = " max number of weighted member entries in action profile (across all"]
    #[doc = " selector groups, if the action profile has a selector)"]
    #[prost(int64, tag = "4")]
    pub size: i64,
    #[doc = " max number of weighted member entries in any given selector group, or 0 if"]
    #[doc = " the action profile does not have a selector"]
    #[prost(int32, tag = "5")]
    pub max_group_size: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct CounterSpec {
    #[prost(enumeration = "counter_spec::Unit", tag = "1")]
    pub unit: i32,
}
#[doc = " Nested message and enum types in `CounterSpec`."]
pub mod counter_spec;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Counter {
    #[prost(message, optional, tag = "1")]
    pub preamble: ::core::option::Option<Preamble>,
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<CounterSpec>,
    #[doc = " number of entries in the counter array"]
    #[prost(int64, tag = "3")]
    pub size: i64,
    #[doc = " unset if index is not user-defined type"]
    #[prost(message, optional, tag = "4")]
    pub index_type_name: ::core::option::Option<P4NamedType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct DirectCounter {
    #[prost(message, optional, tag = "1")]
    pub preamble: ::core::option::Option<Preamble>,
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<CounterSpec>,
    #[doc = " the id of the table to which the counter is attached"]
    #[prost(uint32, tag = "3")]
    pub direct_table_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct MeterSpec {
    #[prost(enumeration = "meter_spec::Unit", tag = "1")]
    pub unit: i32,
}
#[doc = " Nested message and enum types in `MeterSpec`."]
pub mod meter_spec;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Meter {
    #[prost(message, optional, tag = "1")]
    pub preamble: ::core::option::Option<Preamble>,
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<MeterSpec>,
    #[doc = " number of entries in the meter array"]
    #[prost(int64, tag = "3")]
    pub size: i64,
    #[doc = " unset if index is not user-defined type"]
    #[prost(message, optional, tag = "4")]
    pub index_type_name: ::core::option::Option<P4NamedType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct DirectMeter {
    #[prost(message, optional, tag = "1")]
    pub preamble: ::core::option::Option<Preamble>,
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<MeterSpec>,
    #[doc = " the id of the table to which the meter is attached"]
    #[prost(uint32, tag = "3")]
    pub direct_table_id: u32,
}
#[doc = " Any metadata associated with controller Packet-IO (Packet-In or Packet-Out)"]
#[doc = " is modeled as P4 headers carrying special annotations"]
#[doc = " @controller_header(\"packet_out\") and @controller_header(\"packet_in\")"]
#[doc = " respectively. There can be at most one header each with these annotations."]
#[doc = " This message captures the info contained within these special headers,"]
#[doc = " and used in p4runtime.proto to supply the metadata."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ControllerPacketMetadata {
    #[doc = " preamble.name and preamble.id will specify header type (\"packet_out\" or"]
    #[doc = " \"packet_in\" for now)."]
    #[prost(message, optional, tag = "1")]
    pub preamble: ::core::option::Option<Preamble>,
    #[doc = " Ordered based on header layout."]
    #[doc = " This is a constraint on the generator of this P4Info."]
    #[prost(message, repeated, tag = "2")]
    pub metadata: ::prost::alloc::vec::Vec<controller_packet_metadata::Metadata>,
}
#[doc = " Nested message and enum types in `ControllerPacketMetadata`."]
pub mod controller_packet_metadata;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ValueSet {
    #[prost(message, optional, tag = "1")]
    pub preamble: ::core::option::Option<Preamble>,
    #[prost(message, repeated, tag = "2")]
    pub r#match: ::prost::alloc::vec::Vec<MatchField>,
    #[doc = " number of entries in the value_set, as per the P4 constructor call."]
    #[prost(int32, tag = "3")]
    pub size: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Register {
    #[prost(message, optional, tag = "1")]
    pub preamble: ::core::option::Option<Preamble>,
    #[prost(message, optional, tag = "2")]
    pub type_spec: ::core::option::Option<P4DataTypeSpec>,
    #[prost(int32, tag = "3")]
    pub size: i32,
    #[doc = " unset if index is not user-defined type"]
    #[prost(message, optional, tag = "4")]
    pub index_type_name: ::core::option::Option<P4NamedType>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Digest {
    #[prost(message, optional, tag = "1")]
    pub preamble: ::core::option::Option<Preamble>,
    #[prost(message, optional, tag = "2")]
    pub type_spec: ::core::option::Option<P4DataTypeSpec>,
}
