use rs5e_components::{armor::Armor, defender::Defender, identifiable::Identifiable};
use rs5e_concepts::{
    armor::ArmorModel, armor_category::ArmorCategory, armor_class::ArmorClass,
    armor_type::ArmorType, id::Id, material::Material,
};
use rs5e_macro_derive::Identifiable;

#[derive(Debug, Identifiable)]
pub struct ArmorEntity<'a> {
    pub id: Id,
    pub model: &'a ArmorModel,
}

impl Defender for ArmorEntity<'_> {
    fn armor_class(&self) -> &ArmorClass {
        &self.model.armor_class
    }
}

impl Armor for ArmorEntity<'_> {
    fn armor_category(&self) -> &ArmorCategory {
        &self.model.armor_category
    }

    fn primary_material(&self) -> &Material {
        &self.model.primary_material
    }

    fn armor_type(&self) -> &ArmorType {
        &self.model.armor_type
    }

    fn armor_class(&self) -> &ArmorClass {
        &self.model.armor_class
    }
}
