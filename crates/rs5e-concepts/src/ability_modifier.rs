use crate::ability_scores::AbilityScore;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use typeshare::typeshare;

pub const ABILITY_MODIFIER_TABLE: [i32; 30] = [
    -5, -4, -4, -3, -3, -2, -2, -1, -1, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9,
    10,
];

impl AbilityScore {
    #[must_use]
    pub fn modifier(&self) -> AbilityModifier {
        // Safety: AbilityScore type guarantees a stored value between 1 and 30
        AbilityModifier(ABILITY_MODIFIER_TABLE[*self.clone() as usize - 1])
    }
}

#[typeshare]
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct AbilityModifier(i32);

impl AbilityModifier {
    #[must_use]
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn value(&self) -> i32 {
        self.0
    }
}

impl Deref for AbilityModifier {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
