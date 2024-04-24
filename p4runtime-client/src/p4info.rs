use p4runtime::p4::config::v1 as p4cfgv1;
use prost::Message;

#[derive(Debug)]
pub struct P4Info {
    p4info: Option<p4cfgv1::P4Info>,
}

impl AsRef<p4cfgv1::P4Info> for P4Info {
    fn as_ref(&self) -> &p4cfgv1::P4Info {
        self.p4info.as_ref().unwrap()
    }
}

impl P4Info {
    pub fn new() -> Self {
        Self { p4info: None }
    }

    pub fn load_bytes(&mut self, bytes: &[u8]) -> Result<(), prost::DecodeError> {
        self.p4info = Some(p4cfgv1::P4Info::decode(bytes)?);
        Ok(())
    }

    pub fn table_id(&self, table_name: &str) -> u32 {
        self.p4info
            .as_ref()
            .unwrap()
            .tables
            .iter()
            .find(|table| {
                table.preamble.as_ref().unwrap().name == table_name
                    || table.preamble.as_ref().unwrap().alias == table_name
            })
            .map(|table| table.preamble.as_ref().unwrap().id)
            .unwrap_or_default() // 0 is invalid table id
    }

    pub fn table(&self, table_name: &str) -> Option<&p4cfgv1::Table> {
        self.p4info.as_ref().unwrap().tables.iter().find(|table| {
            table.preamble.as_ref().unwrap().name == table_name
                || table.preamble.as_ref().unwrap().alias == table_name
        })
    }

    pub fn table_match_field_id(&self, table_name: &str, field_name: &str) -> u32 {
        self.table(table_name)
            .and_then(|table| {
                table
                    .match_fields
                    .iter()
                    .find(|match_field| match_field.name == field_name)
                    .map(|match_field| match_field.id)
            })
            .unwrap_or_default() // 0 is invalid match field id
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
        self.p4info
            .as_ref()
            .unwrap()
            .actions
            .iter()
            .find(|action| {
                action.preamble.as_ref().unwrap().name == action_name
                    || action.preamble.as_ref().unwrap().alias == action_name
            })
            .map(|action| action.preamble.as_ref().unwrap().id)
            .unwrap_or_default() // 0 is invalid action id
    }

    pub fn action(&self, action_name: &str) -> Option<&p4cfgv1::Action> {
        self.p4info.as_ref().unwrap().actions.iter().find(|action| {
            action.preamble.as_ref().unwrap().name == action_name
                || action.preamble.as_ref().unwrap().alias == action_name
        })
    }

    pub fn action_profile_id(&self, action_profile_name: &str) -> u32 {
        self.p4info
            .as_ref()
            .unwrap()
            .action_profiles
            .iter()
            .find(|action_profile| {
                action_profile.preamble.as_ref().unwrap().name == action_profile_name
                    || action_profile.preamble.as_ref().unwrap().alias == action_profile_name
            })
            .map(|action_profile| action_profile.preamble.as_ref().unwrap().id)
            .unwrap_or_default() // 0 is invalid action profile id
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
        self.p4info
            .as_ref()
            .unwrap()
            .counters
            .iter()
            .find(|counter| {
                counter.preamble.as_ref().unwrap().name == counter_name
                    || counter.preamble.as_ref().unwrap().alias == counter_name
            })
            .map(|counter| counter.preamble.as_ref().unwrap().id)
            .unwrap_or_default() // 0 is invalid counter id
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
        self.p4info
            .as_ref()
            .unwrap()
            .direct_counters
            .iter()
            .find(|direct_counter| {
                direct_counter.preamble.as_ref().unwrap().name == direct_counter_name
                    || direct_counter.preamble.as_ref().unwrap().alias == direct_counter_name
            })
            .map(|direct_counter| direct_counter.preamble.as_ref().unwrap().id)
            .unwrap_or_default() // 0 is invalid direct counter id
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
        self.p4info
            .as_ref()
            .unwrap()
            .meters
            .iter()
            .find(|meter| {
                meter.preamble.as_ref().unwrap().name == meter_name
                    || meter.preamble.as_ref().unwrap().alias == meter_name
            })
            .map(|meter| meter.preamble.as_ref().unwrap().id)
            .unwrap_or_default() // 0 is invalid meter id
    }

    pub fn meter(&self, meter_name: &str) -> Option<&p4cfgv1::Meter> {
        self.p4info.as_ref().unwrap().meters.iter().find(|meter| {
            meter.preamble.as_ref().unwrap().name == meter_name
                || meter.preamble.as_ref().unwrap().alias == meter_name
        })
    }

    pub fn direct_meter_id(&self, direct_meter_name: &str) -> u32 {
        self.p4info
            .as_ref()
            .unwrap()
            .direct_meters
            .iter()
            .find(|direct_meter| {
                direct_meter.preamble.as_ref().unwrap().name == direct_meter_name
                    || direct_meter.preamble.as_ref().unwrap().alias == direct_meter_name
            })
            .map(|direct_meter| direct_meter.preamble.as_ref().unwrap().id)
            .unwrap_or_default() // 0 is invalid direct meter id
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
        self.p4info
            .as_ref()
            .unwrap()
            .value_sets
            .iter()
            .find(|value_set| {
                value_set.preamble.as_ref().unwrap().name == value_set_name
                    || value_set.preamble.as_ref().unwrap().alias == value_set_name
            })
            .map(|value_set| value_set.preamble.as_ref().unwrap().id)
            .unwrap_or_default() // 0 is invalid value set id
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
        self.p4info
            .as_ref()
            .unwrap()
            .registers
            .iter()
            .find(|register| {
                register.preamble.as_ref().unwrap().name == register_name
                    || register.preamble.as_ref().unwrap().alias == register_name
            })
            .map(|register| register.preamble.as_ref().unwrap().id)
            .unwrap_or_default() // 0 is invalid register id
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
        self.p4info
            .as_ref()
            .unwrap()
            .digests
            .iter()
            .find(|digest| {
                digest.preamble.as_ref().unwrap().name == digest_name
                    || digest.preamble.as_ref().unwrap().alias == digest_name
            })
            .map(|digest| digest.preamble.as_ref().unwrap().id)
            .unwrap_or_default() // 0 is invalid digest id
    }

    pub fn digest(&self, digest_name: &str) -> Option<&p4cfgv1::Digest> {
        self.p4info.as_ref().unwrap().digests.iter().find(|digest| {
            digest.preamble.as_ref().unwrap().name == digest_name
                || digest.preamble.as_ref().unwrap().alias == digest_name
        })
    }
}
