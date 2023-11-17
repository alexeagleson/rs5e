use rs5e_concepts::{
    attack_roll::AttackRoll, damage_roll::DamageRoll, damage_source::DamageSource,
    damage_type::DamageType,
};
use rs5e_dice::Dice;

pub trait Damages {
    fn damage_source(&self) -> DamageSource;

    fn damage_type(&self) -> DamageType;

    fn damage_dice(&self) -> Dice;

    fn roll_damage(&self, attack_roll: Option<&AttackRoll>) -> DamageRoll {
        match (self.damage_source(), attack_roll) {
            (DamageSource::Unarmed, Some(attack_roll)) => {
                DamageRoll::from_attack_roll_unarmed(attack_roll)
            }
            (DamageSource::Weapon, Some(attack_roll)) => {
                DamageRoll::from_attack_roll(attack_roll, &|| self.damage_dice().roll())
            }
            // TODO: Better type safety for attacks like weapon where attack roll
            // should not be optional.  Probably break this into separate methods.
            _ => todo!("see comment"),
        }
    }
}

#[cfg(any(test, feature = "test"))]
pub mod mocks {
    #[allow(clippy::wildcard_imports)]
    use super::*;
    use rs5e_dice::{Die, DieLoading, DieType};

    #[derive(Debug)]
    pub struct MockDamages;

    impl Damages for MockDamages {
        fn damage_source(&self) -> DamageSource {
            DamageSource::Unarmed
        }

        fn damage_type(&self) -> DamageType {
            DamageType::Bludgeoning
        }

        fn damage_dice(&self) -> Dice {
            Dice::new(1, Die::new_loaded(DieType::D1, DieLoading::Maximum))
        }
    }
}

#[cfg(any(test, feature = "test"))]
pub mod tests {
    use crate::damages::{mocks::MockDamages, Damages};
    use rs5e_concepts::{attack_roll::AttackRoll, roll::Roll};
    use std::ops::Not;

    #[test]
    fn normal_damage_works() {
        let damage_roll = MockDamages.roll_damage(Some(&AttackRoll::mock_normal()));

        // 1 plus ability modifier of 2
        assert_eq!(damage_roll.total_value(), 3);
        assert!(damage_roll.is_critical().not());
    }

    #[test]
    fn critical_damage_works() {
        let damage_roll = MockDamages.roll_damage(Some(&AttackRoll::mock_critical()));

        assert_eq!(damage_roll.total_value(), 1);
        assert!(damage_roll.is_critical());
    }
}
