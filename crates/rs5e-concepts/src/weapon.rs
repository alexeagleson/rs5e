use crate::{
    damage_type::DamageType, weapon_category::WeaponCategory, weapon_range::WeaponRange,
    weapon_type::WeaponType,
};
use rs5e_dice::Dice;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WeaponModel {
    pub weapon_type: WeaponType,
    pub weapon_range: WeaponRange,
    pub weapon_category: WeaponCategory,
    pub damage_type: DamageType,
    pub damage_dice: Dice,
}
