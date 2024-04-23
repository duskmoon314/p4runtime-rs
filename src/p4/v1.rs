#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4Data {
    #[prost(
        oneof = "p4_data::Data",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12"
    )]
    pub data: ::core::option::Option<p4_data::Data>,
}
#[doc = " Nested message and enum types in `P4Data`."]
pub mod p4_data;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4Varbit {
    #[prost(bytes = "vec", tag = "1")]
    pub bitstring: ::prost::alloc::vec::Vec<u8>,
    #[doc = " dynamic bitwidth of the field"]
    #[prost(int32, tag = "2")]
    pub bitwidth: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4StructLike {
    #[prost(message, repeated, tag = "1")]
    pub members: ::prost::alloc::vec::Vec<P4Data>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4Header {
    #[doc = " If the header is invalid (is_valid is \"false\"), then the bitstrings"]
    #[doc = " repeated field must be empty."]
    #[prost(bool, tag = "1")]
    pub is_valid: bool,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub bitstrings: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4HeaderUnion {
    #[doc = " An empty string indicates that none of the union members are valid and"]
    #[doc = " valid_header must therefore be unset."]
    #[prost(string, tag = "1")]
    pub valid_header_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub valid_header: ::core::option::Option<P4Header>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4HeaderStack {
    #[doc = " The length of this repeated field must always be equal to the compile-time"]
    #[doc = " size of the header stack, which is specified in P4Info."]
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<P4Header>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct P4HeaderUnionStack {
    #[doc = " The length of this repeated field must always be equal to the compile-time"]
    #[doc = " size of the header union stack, which is specified in P4Info."]
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<P4HeaderUnion>,
}
#[doc = " ------------------------------------------------------------------------------"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct WriteRequest {
    #[prost(uint64, tag = "1")]
    pub device_id: u64,
    #[deprecated]
    #[prost(uint64, tag = "2")]
    pub role_id: u64,
    #[prost(string, tag = "6")]
    pub role: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub election_id: ::core::option::Option<Uint128>,
    #[doc = " The write batch, comprising a list of Update operations. The P4Runtime"]
    #[doc = " server may arbitrarily reorder messages within a batch to maximize"]
    #[doc = " performance."]
    #[prost(message, repeated, tag = "4")]
    pub updates: ::prost::alloc::vec::Vec<Update>,
    #[prost(enumeration = "write_request::Atomicity", tag = "5")]
    pub atomicity: i32,
}
#[doc = " Nested message and enum types in `WriteRequest`."]
pub mod write_request;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct WriteResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ReadRequest {
    #[prost(uint64, tag = "1")]
    pub device_id: u64,
    #[doc = " When specified, only return table entries for the given role."]
    #[prost(string, tag = "3")]
    pub role: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub entities: ::prost::alloc::vec::Vec<Entity>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ReadResponse {
    #[prost(message, repeated, tag = "1")]
    pub entities: ::prost::alloc::vec::Vec<Entity>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Update {
    #[prost(enumeration = "update::Type", tag = "1")]
    pub r#type: i32,
    #[prost(message, optional, tag = "2")]
    pub entity: ::core::option::Option<Entity>,
}
#[doc = " Nested message and enum types in `Update`."]
pub mod update;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Entity {
    #[prost(
        oneof = "entity::Entity",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12"
    )]
    pub entity: ::core::option::Option<entity::Entity>,
}
#[doc = " Nested message and enum types in `Entity`."]
pub mod entity;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ExternEntry {
    #[doc = " the extern_type_id is unique for a given architecture and must be in the"]
    #[doc = " range \\[0x81, 0xfe\\]."]
    #[prost(uint32, tag = "1")]
    pub extern_type_id: u32,
    #[doc = " id of the instance"]
    #[prost(uint32, tag = "2")]
    pub extern_id: u32,
    #[prost(message, optional, tag = "3")]
    pub entry: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct TableEntry {
    #[prost(uint32, tag = "1")]
    pub table_id: u32,
    #[prost(message, repeated, tag = "2")]
    pub r#match: ::prost::alloc::vec::Vec<FieldMatch>,
    #[prost(message, optional, tag = "3")]
    pub action: ::core::option::Option<TableAction>,
    #[doc = " Should only be set if the match implies a TCAM lookup, i.e. at least one of"]
    #[doc = " the match fields is Optional, Ternary or Range.  A higher number indicates"]
    #[doc = " higher priority.  Only a highest priority entry that matches the packet"]
    #[doc = " must be selected.  Multiple entries in the same table with the same"]
    #[doc = " priority value are permitted.  See Section \"TableEntry\" in the"]
    #[doc = " specification for details of the behavior."]
    #[prost(int32, tag = "4")]
    pub priority: i32,
    #[doc = " Metadata (cookie) opaque to the target. There is no requirement of where"]
    #[doc = " this is stored, as long as it is returned with the rest of the entry in"]
    #[doc = " a Read RPC. This is deprecated in favor of the more flexible metadata"]
    #[doc = " field."]
    #[deprecated]
    #[prost(uint64, tag = "5")]
    pub controller_metadata: u64,
    #[doc = " meter_config, counter_data and meter_counter_data are convenience fields"]
    #[doc = " that enable the controller to configure the direct resources associated"]
    #[doc = " with the table at the same time as a match-action entry is inserted or"]
    #[doc = " modified."]
    #[doc = " Table write:"]
    #[doc = "    - If the table does not contain a direct resource, then setting the"]
    #[doc = "      corresponding direct resource field in any table write operation will"]
    #[doc = "      return an error."]
    #[doc = "    - When inserting a new table entry, leaving these fields unset means that"]
    #[doc = "      the direct resources of this table (if any) will assume default values."]
    #[doc = "      For counters, the default value is 0, and for meters, the default value"]
    #[doc = "      is always green."]
    #[doc = "    - When updating a table entry, leaving meter_config unset will reset the"]
    #[doc = "      meter (if any) to its default configuration, while leaving counter_data"]
    #[doc = "      or meter_counter_data unset means that the counter (if any) will not be"]
    #[doc = "      updated."]
    #[doc = " Table read:"]
    #[doc = "    - If the table does not contain a direct resource, then the corresponding"]
    #[doc = "      field will not be set in the read table entry."]
    #[doc = "    - If meter_config is unset in the request, or if the meter has a default"]
    #[doc = "      configuration, meter_config will not be set in the response."]
    #[doc = "    - If counter_data or meter_counter_data is unset in the request, it will"]
    #[doc = "      be unset in the response as well."]
    #[prost(message, optional, tag = "6")]
    pub meter_config: ::core::option::Option<MeterConfig>,
    #[prost(message, optional, tag = "7")]
    pub counter_data: ::core::option::Option<CounterData>,
    #[doc = " Per color counters for tables with a direct meter."]
    #[prost(message, optional, tag = "12")]
    pub meter_counter_data: ::core::option::Option<MeterCounterData>,
    #[doc = " Set to true if the table entry is being used to update the non-const"]
    #[doc = " default action of the table. If true, the \"match\" field must be empty and"]
    #[doc = " the \"action\" field must be populated with a valid direct action."]
    #[prost(bool, tag = "8")]
    pub is_default_action: bool,
    #[doc = " The TTL for the entry, in nanoseconds. A value of 0 means that the table"]
    #[doc = " entry never \"expires\"."]
    #[prost(int64, tag = "9")]
    pub idle_timeout_ns: i64,
    #[doc = " Table write: this field should be left unset."]
    #[doc = " Table read: if the table supports idle timeout and time_since_last_hit is"]
    #[doc = " set in the request, this field will be set in the response."]
    #[prost(message, optional, tag = "10")]
    pub time_since_last_hit: ::core::option::Option<table_entry::IdleTimeout>,
    #[doc = " Arbitrary metadata from the controller that is opaque to the target."]
    #[prost(bytes = "vec", tag = "11")]
    pub metadata: ::prost::alloc::vec::Vec<u8>,
}
#[doc = " Nested message and enum types in `TableEntry`."]
pub mod table_entry;
#[doc = " field_match_type ::= exact | ternary | lpm | range | optional"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct FieldMatch {
    #[prost(uint32, tag = "1")]
    pub field_id: u32,
    #[prost(oneof = "field_match::FieldMatchType", tags = "2, 3, 4, 6, 7, 100")]
    pub field_match_type: ::core::option::Option<field_match::FieldMatchType>,
}
#[doc = " Nested message and enum types in `FieldMatch`."]
pub mod field_match;
#[doc = " table_actions ::= action_specification | action_profile_specification"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct TableAction {
    #[prost(oneof = "table_action::Type", tags = "1, 2, 3, 4")]
    pub r#type: ::core::option::Option<table_action::Type>,
}
#[doc = " Nested message and enum types in `TableAction`."]
pub mod table_action;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Action {
    #[prost(uint32, tag = "1")]
    pub action_id: u32,
    #[prost(message, repeated, tag = "4")]
    pub params: ::prost::alloc::vec::Vec<action::Param>,
}
#[doc = " Nested message and enum types in `Action`."]
pub mod action;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ActionProfileActionSet {
    #[prost(message, repeated, tag = "1")]
    pub action_profile_actions: ::prost::alloc::vec::Vec<ActionProfileAction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ActionProfileAction {
    #[prost(message, optional, tag = "1")]
    pub action: ::core::option::Option<Action>,
    #[prost(int32, tag = "2")]
    pub weight: i32,
    #[prost(oneof = "action_profile_action::WatchKind", tags = "3, 4")]
    pub watch_kind: ::core::option::Option<action_profile_action::WatchKind>,
}
#[doc = " Nested message and enum types in `ActionProfileAction`."]
pub mod action_profile_action;
#[doc = " ------------------------------------------------------------------------------"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ActionProfileMember {
    #[prost(uint32, tag = "1")]
    pub action_profile_id: u32,
    #[prost(uint32, tag = "2")]
    pub member_id: u32,
    #[prost(message, optional, tag = "3")]
    pub action: ::core::option::Option<Action>,
}
#[doc = " ------------------------------------------------------------------------------"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ActionProfileGroup {
    #[prost(uint32, tag = "1")]
    pub action_profile_id: u32,
    #[prost(uint32, tag = "2")]
    pub group_id: u32,
    #[prost(message, repeated, tag = "3")]
    pub members: ::prost::alloc::vec::Vec<action_profile_group::Member>,
    #[doc = " Max number of weighted member entries in this group. It cannot be modified"]
    #[doc = " after a group has been created. It must not exceed the static"]
    #[doc = " max_group_size included in P4Info. If the max size is not known at group"]
    #[doc = " creation-time, the client may leave this field unset (default value 0), in"]
    #[doc = " which case the static max_group_size value will be used and the group will"]
    #[doc = " be able to include up to max_group_size weighted member entries."]
    #[prost(int32, tag = "4")]
    pub max_size: i32,
}
#[doc = " Nested message and enum types in `ActionProfileGroup`."]
pub mod action_profile_group;
#[doc = " An index as a protobuf message. In proto3, fields cannot be optional and"]
#[doc = " there is no difference between an unset integer field and an integer field"]
#[doc = " set to 0. This is inconvenient for reading from P4 array-like structures,"]
#[doc = " such as indirect counters and meters. We need a way to do a wildcard read on"]
#[doc = " these but we cannot use a default zero index value to do so, as zero is a"]
#[doc = " valid index (first entry in the array). We therefore wrap the index in a"]
#[doc = " message."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Index {
    #[prost(int64, tag = "1")]
    pub index: i64,
}
#[doc = " ------------------------------------------------------------------------------"]
#[doc = " For WriteRequest, Update.Type must be MODIFY."]
#[doc = " For ReadRequest, the scope is defined as follows:"]
#[doc = " - All meter cells for all meters if meter_id = 0 (default)."]
#[doc = " - All meter cells for given meter_id if index is unset (default)."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct MeterEntry {
    #[prost(uint32, tag = "1")]
    pub meter_id: u32,
    #[prost(message, optional, tag = "2")]
    pub index: ::core::option::Option<Index>,
    #[prost(message, optional, tag = "3")]
    pub config: ::core::option::Option<MeterConfig>,
    #[prost(message, optional, tag = "4")]
    pub counter_data: ::core::option::Option<MeterCounterData>,
}
#[doc = " ------------------------------------------------------------------------------"]
#[doc = " For WriteRequest, Update.Type must be MODIFY. INSERT and DELETE on direct"]
#[doc = " meters is not allowed and will return an error. The insertion/deletion"]
#[doc = " should happen as part of INSERT/DELETE on the associated table-entry."]
#[doc = " For ReadRequest, the scope is defined as follows:"]
#[doc = " - All meter cells for all tables if table_entry.table_id = 0."]
#[doc = " - All meter cells of a table if table_entry.table_id is present and"]
#[doc = "    table_entry.match is empty."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct DirectMeterEntry {
    #[doc = " The associated table entry. This field is required."]
    #[doc = " table_entry.action is ignored. Other fields specify the match."]
    #[prost(message, optional, tag = "1")]
    pub table_entry: ::core::option::Option<TableEntry>,
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<MeterConfig>,
    #[prost(message, optional, tag = "3")]
    pub counter_data: ::core::option::Option<MeterCounterData>,
}
#[doc = " Modeled as RFC 2698: A Two Rate Three Color Marker (trTCM)"]
#[doc = " The trTCM meters a packet stream and marks its packets based on two rates,"]
#[doc = " Peak Information Rate (PIR) and Committed Information Rate (CIR), and their"]
#[doc = " associated burst sizes to be either green, yellow, or red.  A packet is"]
#[doc = " marked red if it exceeds the PIR.  Otherwise it is marked either yellow or"]
#[doc = " green depending on whether it exceeds or doesn't exceed the CIR."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct MeterConfig {
    #[doc = " Committed information rate (units per sec)"]
    #[prost(int64, tag = "1")]
    pub cir: i64,
    #[doc = " Committed burst size"]
    #[prost(int64, tag = "2")]
    pub cburst: i64,
    #[doc = " Peak information rate (units per sec)"]
    #[prost(int64, tag = "3")]
    pub pir: i64,
    #[doc = " Peak burst size"]
    #[prost(int64, tag = "4")]
    pub pburst: i64,
}
#[doc = " ------------------------------------------------------------------------------"]
#[doc = " For WriteRequest, Update.Type must be MODIFY."]
#[doc = " For ReadRequest, the scope is defined as follows:"]
#[doc = " - All counter cells for all counters if counter_id = 0 (default)."]
#[doc = " - All counter cells for given counter_id if index is unset (default)."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct CounterEntry {
    #[prost(uint32, tag = "1")]
    pub counter_id: u32,
    #[prost(message, optional, tag = "2")]
    pub index: ::core::option::Option<Index>,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<CounterData>,
}
#[doc = " ------------------------------------------------------------------------------"]
#[doc = " For WriteRequest, Update.Type must be MODIFY. INSERT and DELETE on direct"]
#[doc = " counters is not allowed and will return an error. The insertion/deletion"]
#[doc = " should happen as part of INSERT/DELETE on the associated table-entry."]
#[doc = " For ReadRequest, the scope is defined as follows:"]
#[doc = " - All counter cells for all tables if table_entry.table_id = 0."]
#[doc = " - All counter cells of a table if table_entry.table_id is present and"]
#[doc = "    table_entry.match is empty."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct DirectCounterEntry {
    #[doc = " The associated table entry. This field is required."]
    #[doc = " table_entry.action is ignored. Other fields specify the match."]
    #[prost(message, optional, tag = "1")]
    pub table_entry: ::core::option::Option<TableEntry>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<CounterData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct CounterData {
    #[prost(int64, tag = "1")]
    pub byte_count: i64,
    #[prost(int64, tag = "2")]
    pub packet_count: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct MeterCounterData {
    #[prost(message, optional, tag = "1")]
    pub green: ::core::option::Option<CounterData>,
    #[prost(message, optional, tag = "2")]
    pub yellow: ::core::option::Option<CounterData>,
    #[prost(message, optional, tag = "3")]
    pub red: ::core::option::Option<CounterData>,
}
#[doc = " ------------------------------------------------------------------------------"]
#[doc = " Only one instance of a Packet Replication Engine (PRE) is expected in the"]
#[doc = " P4 pipeline. Hence, no instance id is needed to access the PRE."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct PacketReplicationEngineEntry {
    #[prost(oneof = "packet_replication_engine_entry::Type", tags = "1, 2")]
    pub r#type: ::core::option::Option<packet_replication_engine_entry::Type>,
}
#[doc = " Nested message and enum types in `PacketReplicationEngineEntry`."]
pub mod packet_replication_engine_entry;
#[doc = " Used for replicas created for cloning and multicasting actions."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Replica {
    #[prost(uint32, tag = "1")]
    pub egress_port: u32,
    #[prost(uint32, tag = "2")]
    pub instance: u32,
}
#[doc = " The (egress_port, instance) pair must be unique for each replica in a given"]
#[doc = " multicast group entry. A packet may be multicast by setting the"]
#[doc = " multicast_group field of PSA ingress output metadata to multicast_group_id"]
#[doc = " of a programmed multicast group entry. The egress_port and instance fields of"]
#[doc = " each replica's egress input metadata will be set to the respective values"]
#[doc = " programmed in the multicast group entry."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct MulticastGroupEntry {
    #[prost(uint32, tag = "1")]
    pub multicast_group_id: u32,
    #[prost(message, repeated, tag = "2")]
    pub replicas: ::prost::alloc::vec::Vec<Replica>,
}
#[doc = " A packet may be cloned by setting the clone_session_id field of PSA"]
#[doc = " ingress/egress output metadata to session_id of a programmed clone session"]
#[doc = " entry. Multiple clones may be created via a single clone session entry by"]
#[doc = " using the replicas message. The clones may be distinguished in the egress"]
#[doc = " using the instance field. The class_of_service field of the clone's egress"]
#[doc = " input metadata will be set to the respective value programmed in the clone"]
#[doc = " session entry. Note that in case of multiple clones, all clones, defined"]
#[doc = " for a clone session, will get the same class of service. The"]
#[doc = " packet_length_bytes field must be set to a non-zero value if the clone"]
#[doc = " packet(s) should be truncated to the given value (in bytes). The packet"]
#[doc = " length is also common to all clones in the clone session. If the"]
#[doc = " packet_length_bytes field is 0, no truncation on the clone(s) will be"]
#[doc = " performed."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct CloneSessionEntry {
    #[prost(uint32, tag = "1")]
    pub session_id: u32,
    #[prost(message, repeated, tag = "2")]
    pub replicas: ::prost::alloc::vec::Vec<Replica>,
    #[prost(uint32, tag = "3")]
    pub class_of_service: u32,
    #[prost(int32, tag = "4")]
    pub packet_length_bytes: i32,
}
#[doc = " A member in a P4 value set. Each member defines a list of matches, which can"]
#[doc = " have different match types."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ValueSetMember {
    #[prost(message, repeated, tag = "1")]
    pub r#match: ::prost::alloc::vec::Vec<FieldMatch>,
}
#[doc = " ------------------------------------------------------------------------------"]
#[doc = " For writing and reading matches in a parser value set. A state transition"]
#[doc = " on an empty value set will never be taken. The number of matches must be at"]
#[doc = " most the size of the value set as specified by the size argument of the"]
#[doc = " value_set constructor in the P4 program."]
#[doc = ""]
#[doc = " For Write Requests:"]
#[doc = "    - MODIFY will write the given matches in the repeated field to the value"]
#[doc = "      set."]
#[doc = "    - INSERT and DELETE are not allowed."]
#[doc = ""]
#[doc = " For Read Requests:"]
#[doc = "    - All matches for all value-set entries if value_set_id = 0"]
#[doc = "    - All matches of the value-set if a valid value_set_id is specified"]
#[doc = "    - The 'match' field must never be set in the ReadRequest"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ValueSetEntry {
    #[prost(uint32, tag = "1")]
    pub value_set_id: u32,
    #[prost(message, repeated, tag = "2")]
    pub members: ::prost::alloc::vec::Vec<ValueSetMember>,
}
#[doc = " ------------------------------------------------------------------------------"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct RegisterEntry {
    #[prost(uint32, tag = "1")]
    pub register_id: u32,
    #[prost(message, optional, tag = "2")]
    pub index: ::core::option::Option<Index>,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<P4Data>,
}
#[doc = " ------------------------------------------------------------------------------"]
#[doc = " Used to configure the digest extern only, not to stream digests or acks"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct DigestEntry {
    #[prost(uint32, tag = "1")]
    pub digest_id: u32,
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<digest_entry::Config>,
}
#[doc = " Nested message and enum types in `DigestEntry`."]
pub mod digest_entry;
#[doc = " ------------------------------------------------------------------------------"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct StreamMessageRequest {
    #[prost(oneof = "stream_message_request::Update", tags = "1, 2, 3, 4")]
    pub update: ::core::option::Option<stream_message_request::Update>,
}
#[doc = " Nested message and enum types in `StreamMessageRequest`."]
pub mod stream_message_request;
#[doc = " Packet sent from the controller to the switch."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct PacketOut {
    #[prost(bytes = "vec", tag = "1")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    #[doc = " This will be based on P4 header annotated as"]
    #[doc = " @controller_header(\"packet_out\")."]
    #[doc = " At most one P4 header can have this annotation."]
    #[prost(message, repeated, tag = "2")]
    pub metadata: ::prost::alloc::vec::Vec<PacketMetadata>,
}
#[doc = " Used by the controller to ack a DigestList message. To avoid flooding the"]
#[doc = " controller, the switch must not generate digest notifications for the same"]
#[doc = " data until a DigestListAck message with the same list_id is received or the"]
#[doc = " ack timeout (ack_timeout_ns field in DigestEntry.Config) expires."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct DigestListAck {
    #[prost(uint32, tag = "1")]
    pub digest_id: u32,
    #[prost(uint64, tag = "2")]
    pub list_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct StreamMessageResponse {
    #[prost(oneof = "stream_message_response::Update", tags = "1, 2, 3, 4, 5, 6")]
    pub update: ::core::option::Option<stream_message_response::Update>,
}
#[doc = " Nested message and enum types in `StreamMessageResponse`."]
pub mod stream_message_response;
#[doc = " Packet sent from the switch to the controller."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct PacketIn {
    #[prost(bytes = "vec", tag = "1")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    #[doc = " This will be based on P4 header annotated as"]
    #[doc = " @controller_header(\"packet_in\")."]
    #[doc = " At most one P4 header can have this annotation."]
    #[prost(message, repeated, tag = "2")]
    pub metadata: ::prost::alloc::vec::Vec<PacketMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct DigestList {
    #[doc = " identifies the digest extern instance"]
    #[prost(uint32, tag = "1")]
    pub digest_id: u32,
    #[doc = " identifies a list of entries, used by receiver to ack"]
    #[prost(uint64, tag = "2")]
    pub list_id: u64,
    #[doc = " List of entries: each call to the Digest<T>::pack() method corresponds to"]
    #[doc = " one entry and we can have as little as one entry."]
    #[prost(message, repeated, tag = "3")]
    pub data: ::prost::alloc::vec::Vec<P4Data>,
    #[doc = " Timestamp at which the server generated the message (in nanoseconds since"]
    #[doc = " Epoch)"]
    #[prost(int64, tag = "4")]
    pub timestamp: i64,
}
#[doc = " Any metadata associated with Packet-IO (controller Packet-In or Packet-Out)"]
#[doc = " needs to be modeled as P4 headers carrying special annotations"]
#[doc = " @controller_header(\"packet_out\") and @controller_header(\"packet_in\")"]
#[doc = " respectively. There can be at most one header each with these annotations."]
#[doc = " These special headers are captured in P4Info ControllerPacketMetadata."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct PacketMetadata {
    #[doc = " This refers to Metadata.id coming from P4Info ControllerPacketMetadata."]
    #[prost(uint32, tag = "1")]
    pub metadata_id: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct MasterArbitrationUpdate {
    #[prost(uint64, tag = "1")]
    pub device_id: u64,
    #[doc = " The role for which the primary client is being arbitrated. For use-cases"]
    #[doc = " where multiple roles are not needed, the controller can leave this unset,"]
    #[doc = " implying default role and full pipeline access."]
    #[prost(message, optional, tag = "2")]
    pub role: ::core::option::Option<Role>,
    #[doc = " The stream RPC with the highest election_id is the primary. The 'primary'"]
    #[doc = " controller instance populates this with its latest election_id. Switch"]
    #[doc = " populates with the highest election ID it has received from all connected"]
    #[doc = " controllers."]
    #[prost(message, optional, tag = "3")]
    pub election_id: ::core::option::Option<Uint128>,
    #[doc = " Switch populates this with OK for the client that is the primary, and"]
    #[doc = " with an error status for all other connected clients (at every primary"]
    #[doc = " client change). The controller does not populate this field."]
    #[prost(message, optional, tag = "4")]
    pub status: ::core::option::Option<super::super::google::rpc::Status>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Role {
    #[doc = " Uniquely identifies this role."]
    #[deprecated]
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[doc = " Describes the role configuration, i.e. what operations, P4 entities,"]
    #[doc = " behaviors, etc. are in the scope of a given role. If config is not set"]
    #[doc = " (default case), it implies all P4 objects and control behaviors are in"]
    #[doc = " scope, i.e. full pipeline access. The format of this message is"]
    #[doc = " out-of-scope of P4Runtime."]
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct IdleTimeoutNotification {
    #[doc = " The only fields that are required to be set in each TableEntry are the"]
    #[doc = " \"key\" fields (table_id, match and priority) along with controller_metadata,"]
    #[doc = " metadata and idle_timeout_ns."]
    #[prost(message, repeated, tag = "1")]
    pub table_entry: ::prost::alloc::vec::Vec<TableEntry>,
    #[doc = " Timestamp at which the server generated the message (in nanoseconds since"]
    #[doc = " Epoch)"]
    #[prost(int64, tag = "2")]
    pub timestamp: i64,
}
#[doc = " Used by the server to asynchronously report errors which occur when"]
#[doc = " processing StreamMessageRequest messages."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct StreamError {
    #[doc = " gRPC canonical error code (see"]
    #[doc = " <https://developers.google.com/maps-booking/reference/grpc-api/status_codes>)"]
    #[prost(int32, tag = "1")]
    pub canonical_code: i32,
    #[doc = " Optional. An explanation of the error."]
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[doc = " Optional. Target and architecture specific space to which this error"]
    #[doc = " belongs."]
    #[doc = " We encourage using triplet: <target>-<arch>-<vendor>,"]
    #[doc = " e.g.\"targetX-psa-vendor1\" or \"targetY-psa-vendor2\"."]
    #[prost(string, tag = "3")]
    pub space: ::prost::alloc::string::String,
    #[doc = " Optional. Numeric code drawn from target-specific error space above."]
    #[prost(int32, tag = "4")]
    pub code: i32,
    #[doc = " Used by the server to convey additional information about the error. One of"]
    #[doc = " the fields must be set (so that the client can identify which type of"]
    #[doc = " stream message triggered the error), but that field may be set to its"]
    #[doc = " default value."]
    #[prost(oneof = "stream_error::Details", tags = "5, 6, 7")]
    pub details: ::core::option::Option<stream_error::Details>,
}
#[doc = " Nested message and enum types in `StreamError`."]
pub mod stream_error;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct PacketOutError {
    #[doc = " Optional. The packet out message that caused the error."]
    #[prost(message, optional, tag = "1")]
    pub packet_out: ::core::option::Option<PacketOut>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct DigestListAckError {
    #[doc = " Optional. The digest list acknowledgement message that caused the error."]
    #[prost(message, optional, tag = "1")]
    pub digest_list_ack: ::core::option::Option<DigestListAck>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct StreamOtherError {
    #[doc = " Optional. The architecture-specific stream message that caused the error."]
    #[prost(message, optional, tag = "1")]
    pub other: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Uint128 {
    #[doc = " Highest 64 bits of a 128 bit number."]
    #[prost(uint64, tag = "1")]
    pub high: u64,
    #[doc = " Lowest 64 bits of a 128 bit number."]
    #[prost(uint64, tag = "2")]
    pub low: u64,
}
#[doc = " ------------------------------------------------------------------------------"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct SetForwardingPipelineConfigRequest {
    #[prost(uint64, tag = "1")]
    pub device_id: u64,
    #[deprecated]
    #[prost(uint64, tag = "2")]
    pub role_id: u64,
    #[prost(string, tag = "6")]
    pub role: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub election_id: ::core::option::Option<Uint128>,
    #[prost(
        enumeration = "set_forwarding_pipeline_config_request::Action",
        tag = "4"
    )]
    pub action: i32,
    #[prost(message, optional, tag = "5")]
    pub config: ::core::option::Option<ForwardingPipelineConfig>,
}
#[doc = " Nested message and enum types in `SetForwardingPipelineConfigRequest`."]
pub mod set_forwarding_pipeline_config_request;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct SetForwardingPipelineConfigResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ForwardingPipelineConfig {
    #[prost(message, optional, tag = "1")]
    pub p4info: ::core::option::Option<super::config::v1::P4Info>,
    #[doc = " Target-specific P4 configuration."]
    #[prost(bytes = "vec", tag = "2")]
    pub p4_device_config: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub cookie: ::core::option::Option<forwarding_pipeline_config::Cookie>,
}
#[doc = " Nested message and enum types in `ForwardingPipelineConfig`."]
pub mod forwarding_pipeline_config;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct GetForwardingPipelineConfigRequest {
    #[prost(uint64, tag = "1")]
    pub device_id: u64,
    #[prost(
        enumeration = "get_forwarding_pipeline_config_request::ResponseType",
        tag = "2"
    )]
    pub response_type: i32,
}
#[doc = " Nested message and enum types in `GetForwardingPipelineConfigRequest`."]
pub mod get_forwarding_pipeline_config_request;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct GetForwardingPipelineConfigResponse {
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<ForwardingPipelineConfig>,
}
#[doc = " Error message used to report a single P4-entity error for a Write RPC."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Error {
    #[doc = " gRPC canonical error code (see"]
    #[doc = " <https://developers.google.com/maps-booking/reference/grpc-api/status_codes>)"]
    #[prost(int32, tag = "1")]
    pub canonical_code: i32,
    #[doc = " Detailed error message."]
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[doc = " Target and architecture specific space to which this error belongs."]
    #[doc = " We encourage using triplet: <target>-<arch>-<vendor>,"]
    #[doc = " e.g.\"targetX-psa-vendor1\" or \"targetY-psa-vendor2\"."]
    #[prost(string, tag = "3")]
    pub space: ::prost::alloc::string::String,
    #[doc = " Numeric code drawn from target-specific error space above."]
    #[prost(int32, tag = "4")]
    pub code: i32,
    #[doc = " Optional: Allows reporting back additional target-specific details on the"]
    #[doc = " error."]
    #[prost(message, optional, tag = "5")]
    pub details: ::core::option::Option<::prost_types::Any>,
}
#[doc = " ------------------------------------------------------------------------------"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct CapabilitiesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct CapabilitiesResponse {
    #[doc = " The full semantic version string (e.g. \"1.1.0-rc.1\") corresponding to the"]
    #[doc = " version of the P4Runtime API currently implemented by the server."]
    #[prost(string, tag = "1")]
    pub p4runtime_api_version: ::prost::alloc::string::String,
}
#[doc = " ------------------------------------------------------------------------------"]
#[doc = " Reserved controller-specified SDN port numbers for reference."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, :: prost :: Enumeration)]
#[repr(i32)]
pub enum SdnPort {
    Unknown = 0,
    #[doc = " SDN ports are numbered starting form 1."]
    Min = 1,
    #[doc = " 0xfffffeff: The maximum value of an SDN port (physical or logical)."]
    Max = -257,
    #[doc = " Reserved SDN port numbers (0xffffff00 - 0xffffffff)"]
    #[doc = " 0xfffffffa: Recirculate the packet back to ingress"]
    Recirculate = -6,
    #[doc = " 0xfffffffd: Send to CPU"]
    Cpu = -3,
}
impl SdnPort {
    #[doc = " String value of the enum field names used in the ProtoBuf definition."]
    #[doc = ""]
    #[doc = " The values are not transformed in any way and thus are considered stable"]
    #[doc = " (if the ProtoBuf definition does not change) and safe for programmatic use."]
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SdnPort::Unknown => "SDN_PORT_UNKNOWN",
            SdnPort::Min => "SDN_PORT_MIN",
            SdnPort::Max => "SDN_PORT_MAX",
            SdnPort::Recirculate => "SDN_PORT_RECIRCULATE",
            SdnPort::Cpu => "SDN_PORT_CPU",
        }
    }
    #[doc = " Creates an enum from field names used in the ProtoBuf definition."]
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SDN_PORT_UNKNOWN" => Some(Self::Unknown),
            "SDN_PORT_MIN" => Some(Self::Min),
            "SDN_PORT_MAX" => Some(Self::Max),
            "SDN_PORT_RECIRCULATE" => Some(Self::Recirculate),
            "SDN_PORT_CPU" => Some(Self::Cpu),
            _ => None,
        }
    }
}
#[doc = " Generated client implementations."]
pub mod p4_runtime_client;
#[doc = " Generated server implementations."]
pub mod p4_runtime_server;
