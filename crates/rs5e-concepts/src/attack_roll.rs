use super::critical_state::CriticalState;
use crate::{ability_modifier::AbilityModifier, proficiency_bonus::ProficiencyBonus, roll::Roll};

#[derive(Debug)]
pub struct AttackRoll {
    pub(crate) roll_value: u32,
    pub(crate) ability_modifier: AbilityModifier,
    pub(crate) proficiency_bonus: Option<ProficiencyBonus>,
}

impl AttackRoll {
    #[must_use]
    pub const fn new(
        roll_value: u32,
        ability_modifier: AbilityModifier,
        proficiency_bonus: Option<ProficiencyBonus>,
    ) -> Self {
        Self {
            roll_value,
            ability_modifier,
            proficiency_bonus,
        }
    }

    #[must_use]
    pub const fn critical_state(&self) -> CriticalState {
        match self.roll_value {
            20 => CriticalState::Critical,
            1 => CriticalState::CriticalFail,
            _ => CriticalState::Normal,
        }
    }

    #[must_use]
    pub const fn is_critical(&self) -> bool {
        self.critical_state().is_critical()
    }

    #[must_use]
    pub const fn ability_modifier(&self) -> &AbilityModifier {
        &self.ability_modifier
    }

    #[must_use]
    pub const fn proficiency_bonus(&self) -> Option<&ProficiencyBonus> {
        self.proficiency_bonus.as_ref()
    }
}

impl Roll for AttackRoll {
    fn raw_value(&self) -> u32 {
        self.roll_value
    }

    fn bonus_value(&self) -> i32 {
        self.ability_modifier().value()
            + (self.proficiency_bonus().map(|p| p.value()).unwrap_or(0) as i32)
    }
}

#[cfg(any(test, feature = "test"))]
pub mod mocks {
    use super::*;

    impl AttackRoll {
        #[must_use]
        pub const fn mock_critical_failure() -> Self {
            Self {
                roll_value: 1,
                ability_modifier: AbilityModifier::new(1),
                proficiency_bonus: Some(ProficiencyBonus::new(0)),
            }
        }

        #[must_use]
        pub const fn mock_normal() -> Self {
            Self {
                roll_value: 10,
                ability_modifier: AbilityModifier::new(2),
                proficiency_bonus: Some(ProficiencyBonus::new(1)),
            }
        }

        #[must_use]
        pub const fn mock_critical() -> Self {
            Self {
                roll_value: 20,
                ability_modifier: AbilityModifier::new(0),
                proficiency_bonus: Some(ProficiencyBonus::new(1)),
            }
        }
    }
}

#[cfg(any(test, feature = "test"))]
pub mod tests {
    use super::*;

    #[test]
    fn critical_failure_works() {
        let normal_attack_roll = AttackRoll::mock_critical_failure();

        assert_eq!(normal_attack_roll.raw_value(), 1);
        assert_eq!(normal_attack_roll.total_value(), 2);
        assert_eq!(
            normal_attack_roll.critical_state(),
            CriticalState::CriticalFail
        );
    }

    #[test]
    fn normal_works() {
        let normal_attack_roll = AttackRoll::mock_normal();

        assert_eq!(normal_attack_roll.total_value(), 13);
        assert_eq!(normal_attack_roll.critical_state(), CriticalState::Normal);
    }

    #[test]
    fn critical_works() {
        let normal_attack_roll = AttackRoll::mock_critical();

        assert_eq!(normal_attack_roll.raw_value(), 20);
        assert_eq!(normal_attack_roll.total_value(), 21);
        assert_eq!(normal_attack_roll.critical_state(), CriticalState::Critical);
    }
}
