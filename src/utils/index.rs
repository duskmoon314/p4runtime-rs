use crate::p4::v1 as p4v1;

impl From<i64> for p4v1::Index {
    fn from(value: i64) -> Self {
        Self { index: value }
    }
}

impl From<p4v1::Index> for i64 {
    fn from(value: p4v1::Index) -> Self {
        value.index
    }
}
