use crate::ability_type::AbilityType;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use typeshare::typeshare;

pub const MIN_ABILITY_SCORE: u32 = 1;
pub const MAX_ABILITY_SCORE: u32 = 30;
pub const DEFAULT_ABILITY_SCORE: u32 = 10;

#[derive(Debug)]
pub struct InvalidAbilityScore;

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AbilityScore(u32);

impl TryFrom<u32> for AbilityScore {
    type Error = InvalidAbilityScore;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        (MIN_ABILITY_SCORE..=MAX_ABILITY_SCORE)
            .contains(&value)
            .then_some(Self(value))
            .ok_or(InvalidAbilityScore)
    }
}

impl Default for AbilityScore {
    fn default() -> Self {
        Self(DEFAULT_ABILITY_SCORE)
    }
}

impl Deref for AbilityScore {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct AbilityScores {
    pub str: AbilityScore,
    pub dex: AbilityScore,
    pub con: AbilityScore,
    pub int: AbilityScore,
    pub wis: AbilityScore,
    pub cha: AbilityScore,
}

impl AbilityScores {
    #[must_use]
    pub const fn score(&self, ability: &AbilityType) -> &AbilityScore {
        match ability {
            AbilityType::Strength => &self.str,
            AbilityType::Dexterity => &self.dex,
            AbilityType::Constitution => &self.con,
            AbilityType::Intelligence => &self.int,
            AbilityType::Wisdom => &self.wis,
            AbilityType::Charisma => &self.cha,
        }
    }

    #[must_use]
    pub fn score_mut(&mut self, ability: &AbilityType) -> &mut AbilityScore {
        match ability {
            AbilityType::Strength => &mut self.str,
            AbilityType::Dexterity => &mut self.dex,
            AbilityType::Constitution => &mut self.con,
            AbilityType::Intelligence => &mut self.int,
            AbilityType::Wisdom => &mut self.wis,
            AbilityType::Charisma => &mut self.cha,
        }
    }
}
