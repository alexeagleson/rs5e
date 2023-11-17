use rs5e_components::{damages::Damages, identifiable::Identifiable, weapon::Weapon};
use rs5e_concepts::{
    damage_source::DamageSource, damage_type::DamageType, id::Id, weapon::WeaponModel,
    weapon_category::WeaponCategory, weapon_range::WeaponRange, weapon_type::WeaponType,
};
use rs5e_dice::Dice;
use rs5e_macro_derive::Identifiable;

#[derive(Debug, Identifiable)]
pub struct WeaponEntity<'a> {
    pub id: Id,
    pub model: &'a WeaponModel,
}

impl Damages for WeaponEntity<'_> {
    fn damage_source(&self) -> DamageSource {
        DamageSource::Weapon
    }

    fn damage_type(&self) -> DamageType {
        self.model.damage_type
    }

    fn damage_dice(&self) -> Dice {
        self.model.damage_dice
    }
}

impl Weapon for WeaponEntity<'_> {
    fn weapon_range(&self) -> &WeaponRange {
        &self.model.weapon_range
    }

    fn weapon_category(&self) -> &WeaponCategory {
        &self.model.weapon_category
    }

    fn weapon_type(&self) -> &WeaponType {
        &self.model.weapon_type
    }
}
