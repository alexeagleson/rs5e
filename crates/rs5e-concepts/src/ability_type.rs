use crate::weapon_range::WeaponRange;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AbilityType {
    /// Physical power
    Strength,
    /// Agility
    Dexterity,
    /// Endurance
    Constitution,
    /// Reasoning and memory
    Intelligence,
    /// Perception and insight
    Wisdom,
    /// Personality
    Charisma,
}

impl From<&WeaponRange> for AbilityType {
    fn from(value: &WeaponRange) -> Self {
        match value {
            WeaponRange::Melee => Self::Strength,
            WeaponRange::Ranged => Self::Dexterity,
        }
    }
}
