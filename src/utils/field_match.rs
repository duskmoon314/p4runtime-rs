use crate::p4::v1 as p4v1;

use super::Canonical;

impl p4v1::field_match::Exact {
    pub fn new(value: Vec<u8>, canonical: bool) -> Self {
        Self {
            value: if canonical { value.canonical() } else { value },
        }
    }
}

impl p4v1::field_match::Lpm {
    pub fn new(value: Vec<u8>, prefix_len: i32, canonical: bool) -> Self {
        // P4Runtime restricts the trailing bits of the value to be zero
        // Set the trailing bits to zero first
        // Then canonicalize the value

        let mut value = value;
        let masked_bytes = prefix_len as usize / 8;
        let masked_bits = prefix_len as usize % 8;
        if masked_bytes != value.len() {
            value[masked_bytes + 1..].fill(0);
            value[masked_bytes] &= (0xff_u16 << (8 - masked_bits)) as u8;
        }

        Self {
            value: if canonical { value.canonical() } else { value },
            prefix_len,
        }
    }
}

impl p4v1::field_match::Ternary {
    pub fn new(value: Vec<u8>, mask: Vec<u8>, canonical: bool) -> Self {
        // P4Runtime restricts the unmasked bits of the value to be zero
        // Set the unmasked bits to zero first
        // Then canonicalize the value

        let mut value = value;
        // pad value with leading zeros if mask is longer
        if mask.len() > value.len() {
            value = [vec![0; mask.len() - value.len()], value].concat();
        }
        for i in 0..mask.len() {
            value[i] &= mask[i];
        }

        Self {
            value: if canonical { value.canonical() } else { value },
            mask: if canonical { mask.canonical() } else { mask },
        }
    }
}

impl p4v1::field_match::Range {
    pub fn new(low: Vec<u8>, high: Vec<u8>, canonical: bool) -> Self {
        Self {
            low: if canonical { low.canonical() } else { low },
            high: if canonical { high.canonical() } else { high },
        }
    }
}

impl p4v1::field_match::Optional {
    pub fn new(value: Vec<u8>, canonical: bool) -> Self {
        Self {
            value: if canonical { value.canonical() } else { value },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lpm() {
        assert_eq!(
            p4v1::field_match::Lpm::new(vec![0x00, 0xab], 16, true),
            p4v1::field_match::Lpm {
                value: vec![0xab],
                prefix_len: 16
            }
        );

        assert_eq!(
            p4v1::field_match::Lpm::new(vec![0x00, 0xab], 16, false),
            p4v1::field_match::Lpm {
                value: vec![0x00, 0xab],
                prefix_len: 16
            }
        );

        assert_eq!(
            p4v1::field_match::Lpm::new(vec![0x00, 0xab, 0xcd], 16, true),
            p4v1::field_match::Lpm {
                value: vec![0xab, 0x00],
                prefix_len: 16
            }
        );

        assert_eq!(
            p4v1::field_match::Lpm::new(vec![0x00, 0xff, 0xcd], 15, false),
            p4v1::field_match::Lpm {
                value: vec![0x00, 0xfe, 0x00],
                prefix_len: 15
            }
        );

        assert_eq!(
            p4v1::field_match::Lpm::new(vec![0x00, 0xff, 0xff], 17, false),
            p4v1::field_match::Lpm {
                value: vec![0x00, 0xff, 0x80],
                prefix_len: 17
            }
        )
    }
}
