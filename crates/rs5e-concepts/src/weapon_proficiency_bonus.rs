use crate::{
    class_type::ClassType,
    level::Level,
    proficiency_bonus::ProficiencyBonus,
    proficiency_type::{proficiency_type_by_class_map, ProficiencyType},
    weapon_category::WeaponCategory,
    weapon_type::WeaponType,
};
use std::ops::{Deref, Not};

pub const WEAPON_PROFICIENCY_BONUS_TABLE: [u32; 20] =
    [2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6];

#[derive(Debug)]
pub struct WeaponProficiencyContext<'a> {
    pub class: &'a ClassType,
    pub level: &'a Level,
    pub weapon_category: Option<&'a WeaponCategory>,
    pub weapon_type: Option<&'a WeaponType>,
}

#[derive(Debug)]
pub struct WeaponProficiencyBonus(Option<ProficiencyBonus>);

impl Deref for WeaponProficiencyBonus {
    type Target = Option<ProficiencyBonus>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<WeaponProficiencyContext<'_>> for WeaponProficiencyBonus {
    fn from(
        WeaponProficiencyContext {
            class,
            level,
            weapon_category,
            weapon_type,
        }: WeaponProficiencyContext,
    ) -> Self {
        let mut has_proficiency = weapon_category
            // Unarmed attacks have proficiency
            .map_or(true, |cat| {
                proficiency_type_by_class_map()
                    .get(class)
                    .is_some_and(|class_proficiency_list| {
                        class_proficiency_list
                            .contains(&ProficiencyType::WeaponCategory(cat.clone()))
                    })
            });

        if has_proficiency.not() {
            has_proficiency = weapon_type
                // Unarmed attacks have proficiency
                .map_or(true, |wep_type| {
                    proficiency_type_by_class_map().get(class).is_some_and(
                        |class_proficiency_list| {
                            class_proficiency_list
                                .contains(&ProficiencyType::WeaponType(wep_type.clone()))
                        },
                    )
                });
        }

        Self(has_proficiency.then(|| {
            ProficiencyBonus::new(
                // Safety: Level type guarantees a stored value between 1 and 20
                WEAPON_PROFICIENCY_BONUS_TABLE[*level.clone() as usize - 1],
            )
        }))
    }
}
