use std::ops::Deref;

use crate::p4::v1 as p4v1;

pub struct Params(pub Vec<p4v1::action::Param>);

impl Deref for Params {
    type Target = Vec<p4v1::action::Param>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Params> for Vec<p4v1::action::Param> {
    fn from(value: Params) -> Self {
        value.0
    }
}

impl From<Vec<p4v1::action::Param>> for Params {
    fn from(value: Vec<p4v1::action::Param>) -> Self {
        Params(value)
    }
}

impl From<Vec<Vec<u8>>> for Params {
    fn from(value: Vec<Vec<u8>>) -> Self {
        value
            .into_iter()
            .enumerate()
            .map(|(i, v)| p4v1::action::Param {
                param_id: (i + 1) as u32,
                value: v,
            })
            .collect::<Vec<_>>()
            .into()
    }
}
