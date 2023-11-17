use super::{damages::Damages, identifiable::Identifiable};
use rs5e_concepts::{
    weapon_category::WeaponCategory, weapon_range::WeaponRange, weapon_type::WeaponType,
};

pub trait Weapon: Identifiable + Damages {
    fn weapon_range(&self) -> &WeaponRange;

    fn weapon_category(&self) -> &WeaponCategory;

    fn weapon_type(&self) -> &WeaponType;
}

#[cfg(any(test, feature = "test"))]
pub mod mocks {
    #[allow(clippy::wildcard_imports)]
    use super::*;
    use rs5e_concepts::{damage_source::DamageSource, damage_type::DamageType, id::Id};
    use rs5e_dice::{Dice, Die, DieLoading, DieType};
    use rs5e_macro_derive::Identifiable;

    #[derive(Debug, Identifiable)]
    pub struct MockWeapon {
        id: Id,
        damage_type: DamageType,
        damage_dice: Dice,
        weapon_range: WeaponRange,
        weapon_category: WeaponCategory,
        weapon_type: WeaponType,
    }

    impl Damages for MockWeapon {
        fn damage_type(&self) -> DamageType {
            self.damage_type
        }

        fn damage_dice(&self) -> Dice {
            self.damage_dice
        }

        fn damage_source(&self) -> DamageSource {
            DamageSource::Weapon
        }
    }

    impl Weapon for MockWeapon {
        fn weapon_range(&self) -> &WeaponRange {
            &self.weapon_range
        }

        fn weapon_category(&self) -> &WeaponCategory {
            &self.weapon_category
        }

        fn weapon_type(&self) -> &WeaponType {
            &self.weapon_type
        }
    }

    impl MockWeapon {
        #[must_use]
        pub fn new_d10_max_damage() -> Self {
            Self {
                id: Id::new_incremental(),
                damage_type: DamageType::Slashing,
                damage_dice: Dice::new(1, Die::new_loaded(DieType::D10, DieLoading::Maximum)),
                weapon_range: WeaponRange::Melee,
                weapon_category: WeaponCategory::Simple,
                weapon_type: WeaponType::Dagger,
            }
        }
    }
}
