use crate::{
    advantage_type::{AdvantageType, Disadvantage},
    armor_category::ArmorCategory,
    class_type::ClassType,
    material::Material,
    proficiency_type::{proficiency_type_by_class_map, ProficiencyType},
};
use std::ops::{Deref, Not};

#[derive(Debug)]
pub struct ArmorProficiencyContext<'a> {
    pub class: &'a ClassType,
    pub armor_category: Option<&'a ArmorCategory>,
    pub armor_primary_material: Option<&'a Material>,
}

#[derive(Debug)]
pub struct ArmorProficiencyPenalty(Option<Disadvantage>);

impl Deref for ArmorProficiencyPenalty {
    type Target = Option<Disadvantage>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ArmorProficiencyContext<'_>> for ArmorProficiencyPenalty {
    fn from(
        ArmorProficiencyContext {
            armor_category,
            armor_primary_material,
            class,
        }: ArmorProficiencyContext,
    ) -> Self {
        let mut has_proficiency = armor_category
            // Unarmored targets have proficiency
            .map_or(true, |cat| {
                proficiency_type_by_class_map()
                    .get(class)
                    .is_some_and(|class_proficiency_list| {
                        class_proficiency_list
                            .contains(&ProficiencyType::ArmorCategory(cat.clone()))
                    })
            });

        if has_proficiency && class == &ClassType::Druid {
            has_proficiency = armor_primary_material
                // Unarmored targets have proficiency
                .map_or(true, |mat| {
                    proficiency_type_by_class_map().get(class).is_some_and(
                        |class_proficiency_list| {
                            class_proficiency_list
                                .contains(&ProficiencyType::ArmorMaterial(mat.clone()))
                        },
                    )
                });
        }

        Self(has_proficiency.not().then_some(Disadvantage))
    }
}

impl From<&ArmorProficiencyPenalty> for AdvantageType {
    fn from(value: &ArmorProficiencyPenalty) -> Self {
        value.deref().as_ref().map_or(Self::Normal, |disadvantage| {
            Self::Disadvantage(disadvantage.clone())
        })
    }
}
