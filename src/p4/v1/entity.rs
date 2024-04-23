#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Oneof)]
pub enum Entity {
    #[prost(message, tag = "1")]
    ExternEntry(super::ExternEntry),
    #[prost(message, tag = "2")]
    TableEntry(super::TableEntry),
    #[prost(message, tag = "3")]
    ActionProfileMember(super::ActionProfileMember),
    #[prost(message, tag = "4")]
    ActionProfileGroup(super::ActionProfileGroup),
    #[prost(message, tag = "5")]
    MeterEntry(super::MeterEntry),
    #[prost(message, tag = "6")]
    DirectMeterEntry(super::DirectMeterEntry),
    #[prost(message, tag = "7")]
    CounterEntry(super::CounterEntry),
    #[prost(message, tag = "8")]
    DirectCounterEntry(super::DirectCounterEntry),
    #[prost(message, tag = "9")]
    PacketReplicationEngineEntry(super::PacketReplicationEngineEntry),
    #[prost(message, tag = "10")]
    ValueSetEntry(super::ValueSetEntry),
    #[prost(message, tag = "11")]
    RegisterEntry(super::RegisterEntry),
    #[prost(message, tag = "12")]
    DigestEntry(super::DigestEntry),
}
