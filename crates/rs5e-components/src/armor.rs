use super::{defender::Defender, identifiable::Identifiable};
use rs5e_concepts::{
    armor_category::ArmorCategory, armor_class::ArmorClass, armor_type::ArmorType,
    material::Material,
};

pub trait Armor: Identifiable + Defender {
    fn armor_type(&self) -> &ArmorType;

    fn armor_category(&self) -> &ArmorCategory;

    fn armor_class(&self) -> &ArmorClass;

    fn primary_material(&self) -> &Material;
}

#[cfg(any(test, feature = "test"))]
pub mod mocks {
    #[allow(clippy::wildcard_imports)]
    use super::*;
    use rs5e_concepts::{armor_category::ArmorCategory, id::Id};
    use rs5e_macro_derive::Identifiable;

    #[derive(Debug, Identifiable)]
    pub struct MockArmor {
        id: Id,
        armor_type: ArmorType,
        armor_class: ArmorClass,
        armor_category: ArmorCategory,
        primary_material: Material,
    }

    impl Defender for MockArmor {}

    impl Armor for MockArmor {
        fn armor_category(&self) -> &ArmorCategory {
            &self.armor_category
        }

        fn primary_material(&self) -> &Material {
            &self.primary_material
        }

        fn armor_type(&self) -> &ArmorType {
            &self.armor_type
        }

        fn armor_class(&self) -> &ArmorClass {
            &self.armor_class
        }
    }

    impl MockArmor {
        #[must_use]
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self {
                id: Id::new_incremental(),
                armor_category: ArmorCategory::Light,
                primary_material: Material::Unknown,
                armor_type: ArmorType::Breastplate,
                armor_class: ArmorClass::default(),
            }
        }
    }
}
