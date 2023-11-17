use crate::{
    armor_category::ArmorCategory, armor_class::ArmorClass, armor_type::ArmorType,
    material::Material,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArmorModel {
    pub armor_type: ArmorType,
    pub armor_class: ArmorClass,
    pub armor_category: ArmorCategory,
    pub primary_material: Material,
}
