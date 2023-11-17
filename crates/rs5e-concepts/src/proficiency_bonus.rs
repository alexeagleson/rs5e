#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct ProficiencyBonus(u32);

impl ProficiencyBonus {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn value(&self) -> u32 {
        self.0
    }
}

impl Deref for ProficiencyBonus {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
