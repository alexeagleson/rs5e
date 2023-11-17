use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

pub const DEFAULT_CLASS_TYPE: ClassType = ClassType::Barbarian;

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ClassType {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

impl Display for ClassType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
