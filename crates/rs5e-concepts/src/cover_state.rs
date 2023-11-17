use crate::armor_class::ArmorClass;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

pub const DEFAULT_COVER_STATE: CoverState = CoverState::None;

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct CannotBeTargeted;

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum CoverState {
    /// No impact on attacks or saving throws
    None,
    /// A target with half cover has a +2 bonus to AC and Dexterity saving throws.
    Half,
    /// A target with three-quarters cover has a +5 bonus to AC and Dexterity saving throws.
    ThreeQuarters,
    /// A target with total cover can’t be targeted directly by an attack or a spell
    Total,
}

impl Default for CoverState {
    fn default() -> Self {
        DEFAULT_COVER_STATE
    }
}

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "t", content = "c")]
pub enum CoverBonus {
    CanBeTargeted(CanBeTargetedCoverBonus),
    CannotBeTargeted(CannotBeTargeted),
}

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase", tag = "t", content = "c")]
pub enum CanBeTargetedCoverBonus {
    None,
    ArmorClass(ArmorClass),
}

impl CanBeTargetedCoverBonus {
    #[must_use]
    pub const fn armor_class_bonus(&self) -> Option<&ArmorClass> {
        match self {
            Self::ArmorClass(ac) => Some(ac),
            Self::None => None,
        }
    }
}

impl From<&CoverState> for CoverBonus {
    fn from(state: &CoverState) -> Self {
        match state {
            CoverState::None => Self::CanBeTargeted(CanBeTargetedCoverBonus::None),
            CoverState::Half => {
                Self::CanBeTargeted(CanBeTargetedCoverBonus::ArmorClass(ArmorClass::new(2)))
            }
            CoverState::ThreeQuarters => {
                Self::CanBeTargeted(CanBeTargetedCoverBonus::ArmorClass(ArmorClass::new(5)))
            }
            CoverState::Total => Self::CannotBeTargeted(CannotBeTargeted),
        }
    }
}

impl TryFrom<&CoverBonus> for CanBeTargetedCoverBonus {
    type Error = CannotBeTargeted;

    fn try_from(value: &CoverBonus) -> Result<Self, Self::Error> {
        match value {
            CoverBonus::CanBeTargeted(targeted_bonus) => Ok(targeted_bonus.clone()),
            CoverBonus::CannotBeTargeted(cannot_be_targeted) => Err(cannot_be_targeted.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cover_order() {
        assert!(CoverState::Total > CoverState::None);
        assert!(CoverState::Half < CoverState::ThreeQuarters);
    }
}
