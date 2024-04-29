use std::collections::HashMap;

use p4runtime::p4::config::v1 as p4cfgv1;
use prost::Message;

#[derive(Debug, Default)]
pub struct P4Info {
    p4info: Option<p4cfgv1::P4Info>,

    table_name_to_id: HashMap<String, u32>,
    table_match_field_name_to_id: HashMap<(String, String), u32>,
    action_name_to_id: HashMap<String, u32>,
    action_profile_name_to_id: HashMap<String, u32>,
    counter_name_to_id: HashMap<String, u32>,
    direct_counter_name_to_id: HashMap<String, u32>,
    meter_name_to_id: HashMap<String, u32>,
    direct_meter_name_to_id: HashMap<String, u32>,
    value_set_name_to_id: HashMap<String, u32>,
    register_name_to_id: HashMap<String, u32>,
    digest_name_to_id: HashMap<String, u32>,
}

impl AsRef<p4cfgv1::P4Info> for P4Info {
    fn as_ref(&self) -> &p4cfgv1::P4Info {
        self.p4info.as_ref().unwrap()
    }
}

impl P4Info {
    pub fn load_bytes(&mut self, bytes: &[u8]) -> Result<(), prost::DecodeError> {
        self.p4info = Some(p4cfgv1::P4Info::decode(bytes)?);

        self.table_name_to_id = self
            .p4info
            .as_ref()
            .unwrap()
            .tables
            .iter()
            .flat_map(|table| {
                [
                    (
                        table.preamble.as_ref().unwrap().name.clone(),
                        table.preamble.as_ref().unwrap().id,
                    ),
                    (
                        table.preamble.as_ref().unwrap().alias.clone(),
                        table.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.table_match_field_name_to_id.clear();
        self.p4info
            .as_ref()
            .unwrap()
            .tables
            .iter()
            .for_each(|table| {
                table.match_fields.iter().for_each(|match_field| {
                    self.table_match_field_name_to_id.insert(
                        (
                            table.preamble.as_ref().unwrap().name.clone(),
                            match_field.name.clone(),
                        ),
                        match_field.id,
                    );
                    self.table_match_field_name_to_id.insert(
                        (
                            table.preamble.as_ref().unwrap().alias.clone(),
                            match_field.name.clone(),
                        ),
                        match_field.id,
                    );
                })
            });

        self.action_name_to_id = self
            .p4info
            .as_ref()
            .unwrap()
            .actions
            .iter()
            .flat_map(|action| {
                [
                    (
                        action.preamble.as_ref().unwrap().name.clone(),
                        action.preamble.as_ref().unwrap().id,
                    ),
                    (
                        action.preamble.as_ref().unwrap().alias.clone(),
                        action.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.action_profile_name_to_id = self
            .p4info
            .as_ref()
            .unwrap()
            .action_profiles
            .iter()
            .flat_map(|action_profile| {
                [
                    (
                        action_profile.preamble.as_ref().unwrap().name.clone(),
                        action_profile.preamble.as_ref().unwrap().id,
                    ),
                    (
                        action_profile.preamble.as_ref().unwrap().alias.clone(),
                        action_profile.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.counter_name_to_id = self
            .p4info
            .as_ref()
            .unwrap()
            .counters
            .iter()
            .flat_map(|counter| {
                [
                    (
                        counter.preamble.as_ref().unwrap().name.clone(),
                        counter.preamble.as_ref().unwrap().id,
                    ),
                    (
                        counter.preamble.as_ref().unwrap().alias.clone(),
                        counter.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.direct_counter_name_to_id = self
            .p4info
            .as_ref()
            .unwrap()
            .direct_counters
            .iter()
            .flat_map(|direct_counter| {
                [
                    (
                        direct_counter.preamble.as_ref().unwrap().name.clone(),
                        direct_counter.preamble.as_ref().unwrap().id,
                    ),
                    (
                        direct_counter.preamble.as_ref().unwrap().alias.clone(),
                        direct_counter.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.meter_name_to_id = self
            .p4info
            .as_ref()
            .unwrap()
            .meters
            .iter()
            .flat_map(|meter| {
                [
                    (
                        meter.preamble.as_ref().unwrap().name.clone(),
                        meter.preamble.as_ref().unwrap().id,
                    ),
                    (
                        meter.preamble.as_ref().unwrap().alias.clone(),
                        meter.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.direct_meter_name_to_id = self
            .p4info
            .as_ref()
            .unwrap()
            .direct_meters
            .iter()
            .flat_map(|direct_meter| {
                [
                    (
                        direct_meter.preamble.as_ref().unwrap().name.clone(),
                        direct_meter.preamble.as_ref().unwrap().id,
                    ),
                    (
                        direct_meter.preamble.as_ref().unwrap().alias.clone(),
                        direct_meter.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.value_set_name_to_id = self
            .p4info
            .as_ref()
            .unwrap()
            .value_sets
            .iter()
            .flat_map(|value_set| {
                [
                    (
                        value_set.preamble.as_ref().unwrap().name.clone(),
                        value_set.preamble.as_ref().unwrap().id,
                    ),
                    (
                        value_set.preamble.as_ref().unwrap().alias.clone(),
                        value_set.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.register_name_to_id = self
            .p4info
            .as_ref()
            .unwrap()
            .registers
            .iter()
            .flat_map(|register| {
                [
                    (
                        register.preamble.as_ref().unwrap().name.clone(),
                        register.preamble.as_ref().unwrap().id,
                    ),
                    (
                        register.preamble.as_ref().unwrap().alias.clone(),
                        register.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        self.digest_name_to_id = self
            .p4info
            .as_ref()
            .unwrap()
            .digests
            .iter()
            .flat_map(|digest| {
                [
                    (
                        digest.preamble.as_ref().unwrap().name.clone(),
                        digest.preamble.as_ref().unwrap().id,
                    ),
                    (
                        digest.preamble.as_ref().unwrap().alias.clone(),
                        digest.preamble.as_ref().unwrap().id,
                    ),
                ]
            })
            .collect();

        Ok(())
    }

    pub fn table_id(&self, table_name: &str) -> u32 {
        *self.table_name_to_id.get(table_name).unwrap_or(&0)
    }

    pub fn table(&self, table_name: &str) -> Option<&p4cfgv1::Table> {
        self.p4info.as_ref().unwrap().tables.iter().find(|table| {
            table.preamble.as_ref().unwrap().name == table_name
                || table.preamble.as_ref().unwrap().alias == table_name
        })
    }

    pub fn table_match_field_id(&self, table_name: &str, field_name: &str) -> u32 {
        *self
            .table_match_field_name_to_id
            .get(&(table_name.to_string(), field_name.to_string()))
            .unwrap_or(&0)
    }

    pub fn table_match_field(
        &self,
        table_name: &str,
        field_name: &str,
    ) -> Option<&p4cfgv1::MatchField> {
        self.table(table_name).and_then(|table| {
            table
                .match_fields
                .iter()
                .find(|match_field| match_field.name == field_name)
        })
    }

    pub fn action_id(&self, action_name: &str) -> u32 {
        *self.action_name_to_id.get(action_name).unwrap_or(&0)
    }

    pub fn action(&self, action_name: &str) -> Option<&p4cfgv1::Action> {
        self.p4info.as_ref().unwrap().actions.iter().find(|action| {
            action.preamble.as_ref().unwrap().name == action_name
                || action.preamble.as_ref().unwrap().alias == action_name
        })
    }

    pub fn action_profile_id(&self, action_profile_name: &str) -> u32 {
        *self
            .action_profile_name_to_id
            .get(action_profile_name)
            .unwrap_or(&0)
    }

    pub fn action_profile(&self, action_profile_name: &str) -> Option<&p4cfgv1::ActionProfile> {
        self.p4info
            .as_ref()
            .unwrap()
            .action_profiles
            .iter()
            .find(|action_profile| {
                action_profile.preamble.as_ref().unwrap().name == action_profile_name
                    || action_profile.preamble.as_ref().unwrap().alias == action_profile_name
            })
    }

    pub fn counter_id(&self, counter_name: &str) -> u32 {
        *self.counter_name_to_id.get(counter_name).unwrap_or(&0)
    }

    pub fn counter(&self, counter_name: &str) -> Option<&p4cfgv1::Counter> {
        self.p4info
            .as_ref()
            .unwrap()
            .counters
            .iter()
            .find(|counter| {
                counter.preamble.as_ref().unwrap().name == counter_name
                    || counter.preamble.as_ref().unwrap().alias == counter_name
            })
    }

    pub fn direct_counter_id(&self, direct_counter_name: &str) -> u32 {
        *self
            .direct_counter_name_to_id
            .get(direct_counter_name)
            .unwrap_or(&0)
    }

    pub fn direct_counter(&self, direct_counter_name: &str) -> Option<&p4cfgv1::DirectCounter> {
        self.p4info
            .as_ref()
            .unwrap()
            .direct_counters
            .iter()
            .find(|direct_counter| {
                direct_counter.preamble.as_ref().unwrap().name == direct_counter_name
                    || direct_counter.preamble.as_ref().unwrap().alias == direct_counter_name
            })
    }

    pub fn meter_id(&self, meter_name: &str) -> u32 {
        *self.meter_name_to_id.get(meter_name).unwrap_or(&0)
    }

    pub fn meter(&self, meter_name: &str) -> Option<&p4cfgv1::Meter> {
        self.p4info.as_ref().unwrap().meters.iter().find(|meter| {
            meter.preamble.as_ref().unwrap().name == meter_name
                || meter.preamble.as_ref().unwrap().alias == meter_name
        })
    }

    pub fn direct_meter_id(&self, direct_meter_name: &str) -> u32 {
        *self
            .direct_meter_name_to_id
            .get(direct_meter_name)
            .unwrap_or(&0)
    }

    pub fn direct_meter(&self, direct_meter_name: &str) -> Option<&p4cfgv1::DirectMeter> {
        self.p4info
            .as_ref()
            .unwrap()
            .direct_meters
            .iter()
            .find(|direct_meter| {
                direct_meter.preamble.as_ref().unwrap().name == direct_meter_name
                    || direct_meter.preamble.as_ref().unwrap().alias == direct_meter_name
            })
    }

    pub fn value_set_id(&self, value_set_name: &str) -> u32 {
        *self.value_set_name_to_id.get(value_set_name).unwrap_or(&0)
    }

    pub fn value_set(&self, value_set_name: &str) -> Option<&p4cfgv1::ValueSet> {
        self.p4info
            .as_ref()
            .unwrap()
            .value_sets
            .iter()
            .find(|value_set| {
                value_set.preamble.as_ref().unwrap().name == value_set_name
                    || value_set.preamble.as_ref().unwrap().alias == value_set_name
            })
    }

    pub fn register_id(&self, register_name: &str) -> u32 {
        *self.register_name_to_id.get(register_name).unwrap_or(&0)
    }

    pub fn register(&self, register_name: &str) -> Option<&p4cfgv1::Register> {
        self.p4info
            .as_ref()
            .unwrap()
            .registers
            .iter()
            .find(|register| {
                register.preamble.as_ref().unwrap().name == register_name
                    || register.preamble.as_ref().unwrap().alias == register_name
            })
    }

    pub fn digest_id(&self, digest_name: &str) -> u32 {
        *self.digest_name_to_id.get(digest_name).unwrap_or(&0)
    }

    pub fn digest(&self, digest_name: &str) -> Option<&p4cfgv1::Digest> {
        self.p4info.as_ref().unwrap().digests.iter().find(|digest| {
            digest.preamble.as_ref().unwrap().name == digest_name
                || digest.preamble.as_ref().unwrap().alias == digest_name
        })
    }
}
