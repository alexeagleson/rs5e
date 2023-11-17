use super::{attack_roll::AttackRoll, critical_state::CriticalState};
use crate::{ability_modifier::AbilityModifier, roll::Roll};

/// This type is meant to remain in context of the roll itself, and factors that
/// affect the value of the roll. Other concepts like the actual damage should
/// be tracked elsewhere
#[derive(Debug)]
pub struct DamageRoll {
    pub(crate) roll_value: u32,
    pub(crate) ability_modifier: AbilityModifier,
    // pub(crate) damage_type: DamageType,
    pub(crate) damage_roll_type: DamageRollType,
}

#[derive(Debug)]
pub enum DamageRollType {
    Normal,
    Critical { bonus_roll_value: u32 },
}

impl DamageRollType {
    #[must_use]
    pub const fn is_critical(&self) -> bool {
        matches!(self, Self::Critical { .. })
    }
}

impl DamageRoll {
    #[must_use]
    pub const fn new(
        roll_value: u32,
        ability_modifier: AbilityModifier,
        damage_roll_type: DamageRollType,
    ) -> Self {
        Self {
            roll_value,
            ability_modifier,
            damage_roll_type,
        }
    }

    #[must_use]
    pub const fn is_critical(&self) -> bool {
        self.damage_roll_type.is_critical()
    }

    #[must_use]
    pub const fn damage_roll_type(&self) -> &DamageRollType {
        &self.damage_roll_type
    }

    #[must_use]
    pub const fn ability_modifier(&self) -> &AbilityModifier {
        &self.ability_modifier
    }

    pub fn from_attack_roll(attack_roll: &AttackRoll, damage_roll: &impl Fn() -> u32) -> Self {
        Self {
            roll_value: damage_roll(),
            ability_modifier: attack_roll.ability_modifier().clone(),
            // damage_type,
            damage_roll_type: match attack_roll.critical_state() {
                CriticalState::Critical => DamageRollType::Critical {
                    bonus_roll_value: damage_roll(),
                },
                CriticalState::Normal | CriticalState::CriticalFail => DamageRollType::Normal,
            },
        }
    }

    #[must_use]
    pub fn from_attack_roll_unarmed(attack_roll: &AttackRoll) -> Self {
        Self {
            // Rule for unarmed (default) is 1 damage + STR
            // Since stats are not yet implemented it's just 1 + 0
            roll_value: 1,
            ability_modifier: attack_roll.ability_modifier().clone(),
            // damage_type: DamageType::Bludgeoning,
            damage_roll_type: match attack_roll.critical_state() {
                CriticalState::Critical => DamageRollType::Critical {
                    // Rule for unarmed damage is 1 damage + STR
                    // Since stats are not yet implemented it's just 1 + 0
                    // Reference: <https://rpg.stackexchange.com/a/57054>
                    bonus_roll_value: 0,
                },
                CriticalState::Normal | CriticalState::CriticalFail => DamageRollType::Normal,
            },
        }
    }
}

impl Roll for DamageRoll {
    fn raw_value(&self) -> u32 {
        self.roll_value
    }

    fn bonus_value(&self) -> i32 {
        match self.damage_roll_type {
            DamageRollType::Normal => self.ability_modifier().value(),
            DamageRollType::Critical { bonus_roll_value } => {
                bonus_roll_value as i32 + self.ability_modifier().value()
            }
        }
    }
}

#[cfg(any(test, feature = "test"))]
pub mod mocks {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    impl DamageRoll {
        /// Total damage will be 20 (18 plus ability modifier of 2)
        pub fn mock_normal() -> Self {
            let normal_attack_roll = AttackRoll::mock_normal();
            Self::from_attack_roll(&normal_attack_roll, &|| 18)
        }

        /// Total damage will be 36 (18, then an identical critical roll with no ability modifier)
        pub fn mock_critical() -> Self {
            let critical_attack_roll = AttackRoll::mock_critical();
            Self::from_attack_roll(
                &critical_attack_roll,
                // DamageType::Bludgeoning,
                &|| 18,
            )
        }
    }
}

#[cfg(any(test, feature = "test"))]
mod tests {
    use super::*;
    use std::ops::Not;

    #[test]
    fn normal_damage_roll_works() {
        let damage_roll = DamageRoll::mock_normal();

        assert!(damage_roll.is_critical().not());
        assert_eq!(damage_roll.total_value(), 20);
    }

    #[test]
    fn critical_damage_roll_works() {
        let damage_roll = DamageRoll::mock_critical();

        assert!(damage_roll.is_critical());
        assert_eq!(damage_roll.total_value(), 36);
    }
}
