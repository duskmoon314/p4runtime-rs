use crate::p4::v1 as p4v1;

impl From<u128> for p4v1::Uint128 {
    fn from(value: u128) -> Self {
        let high = (value >> 64) as u64;
        let low = value as u64;
        Self { high, low }
    }
}

impl From<p4v1::Uint128> for u128 {
    fn from(value: p4v1::Uint128) -> Self {
        (value.high as u128) << 64 | value.low as u128
    }
}
