use crate::{
    armor_category::ArmorCategory, class_type::ClassType, weapon_category::WeaponCategory,
    weapon_type::WeaponType, material::Material,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::OnceLock};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[serde(rename_all = "camelCase", tag = "t", content = "c")]
pub enum ProficiencyType {
    WeaponCategory(WeaponCategory),
    WeaponType(WeaponType),
    ArmorCategory(ArmorCategory),
    ArmorMaterial(Material),
}

#[typeshare]
pub type ProficiencyTypeByClassMap = HashMap<ClassType, Vec<ProficiencyType>>;

// Source: <https://www.dndbeyond.com/sources/basic-rules/classes#ClassesSummary>
#[allow(clippy::too_many_lines)]
pub fn proficiency_type_by_class_map() -> &'static ProficiencyTypeByClassMap {
    static MAP: OnceLock<ProficiencyTypeByClassMap> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut m: HashMap<ClassType, Vec<ProficiencyType>> = HashMap::with_capacity(12);
        m.insert(
            ClassType::Barbarian,
            // Light and medium armor, shields, simple and martial weapons
            Vec::from([
                ProficiencyType::ArmorCategory(ArmorCategory::Light),
                ProficiencyType::ArmorCategory(ArmorCategory::Medium),
                ProficiencyType::ArmorCategory(ArmorCategory::Shield),
                ProficiencyType::WeaponCategory(WeaponCategory::Simple),
                ProficiencyType::WeaponCategory(WeaponCategory::Martial),
            ]),
        );
        m.insert(
            ClassType::Bard,
            // Light armor, simple weapons, hand crossbows, longswords, rapiers, shortswords
            Vec::from([
                ProficiencyType::ArmorCategory(ArmorCategory::Light),
                ProficiencyType::WeaponCategory(WeaponCategory::Simple),
                ProficiencyType::WeaponType(WeaponType::CrossbowHand),
                ProficiencyType::WeaponType(WeaponType::Longsword),
                ProficiencyType::WeaponType(WeaponType::Rapier),
                ProficiencyType::WeaponType(WeaponType::Shortsword),
            ]),
        );
        m.insert(
            ClassType::Cleric,
            // Light and medium armor, shields, simple weapons
            Vec::from([
                ProficiencyType::ArmorCategory(ArmorCategory::Light),
                ProficiencyType::ArmorCategory(ArmorCategory::Medium),
                ProficiencyType::ArmorCategory(ArmorCategory::Shield),
                ProficiencyType::WeaponCategory(WeaponCategory::Simple),
            ]),
        );
        m.insert(
            ClassType::Druid,
            // Light and medium armor (nonmetal), shields (nonmetal), clubs, daggers, darts, javelins, maces, quarterstaffs, scimitars, sickles, slings, spears
            Vec::from([
                ProficiencyType::ArmorCategory(ArmorCategory::Light),
                ProficiencyType::ArmorCategory(ArmorCategory::Medium),
                ProficiencyType::ArmorCategory(ArmorCategory::Shield),
                ProficiencyType::ArmorMaterial(Material::Unknown),
                ProficiencyType::ArmorCategory(ArmorCategory::Shield),

            ]),
        );
        m.insert(
            ClassType::Fighter,
            // All armor, shields, simple and martial weapons
            Vec::from([
                ProficiencyType::ArmorCategory(ArmorCategory::Light),
                ProficiencyType::ArmorCategory(ArmorCategory::Medium),
                ProficiencyType::ArmorCategory(ArmorCategory::Heavy),
                ProficiencyType::ArmorCategory(ArmorCategory::Shield),
                ProficiencyType::WeaponCategory(WeaponCategory::Simple),
                ProficiencyType::WeaponCategory(WeaponCategory::Martial),
            ]),
        );
        m.insert(
            ClassType::Monk,
            // Simple weapons, shortswords
            Vec::from([
                ProficiencyType::WeaponCategory(WeaponCategory::Simple),
                ProficiencyType::WeaponType(WeaponType::Shortsword),
            ]),
        );
        m.insert(
            ClassType::Paladin,
            // All armor, shields, simple and martial weapons
            Vec::from([
                ProficiencyType::ArmorCategory(ArmorCategory::Light),
                ProficiencyType::ArmorCategory(ArmorCategory::Medium),
                ProficiencyType::ArmorCategory(ArmorCategory::Heavy),
                ProficiencyType::ArmorCategory(ArmorCategory::Shield),
                ProficiencyType::WeaponCategory(WeaponCategory::Simple),
                ProficiencyType::WeaponCategory(WeaponCategory::Martial),
            ]),
        );
        m.insert(
            ClassType::Ranger,
            // Light and medium armor, shields, simple and martial weapons
            Vec::from([
                ProficiencyType::ArmorCategory(ArmorCategory::Light),
                ProficiencyType::ArmorCategory(ArmorCategory::Medium),
                ProficiencyType::ArmorCategory(ArmorCategory::Shield),
                ProficiencyType::WeaponCategory(WeaponCategory::Simple),
                ProficiencyType::WeaponCategory(WeaponCategory::Martial),
            ]),
        );
        m.insert(
            ClassType::Rogue,
            // Light armor, simple weapons, hand crossbows, longswords, rapiers, shortswords
            Vec::from([
                ProficiencyType::ArmorCategory(ArmorCategory::Light),
                ProficiencyType::WeaponCategory(WeaponCategory::Simple),
                ProficiencyType::WeaponType(WeaponType::CrossbowHand),
                ProficiencyType::WeaponType(WeaponType::Longsword),
                ProficiencyType::WeaponType(WeaponType::Rapier),
                ProficiencyType::WeaponType(WeaponType::Shortsword),
            ]),
        );
        m.insert(
            ClassType::Sorcerer,
            // Daggers, darts, slings, quarterstaffs, light crossbows
            Vec::from([
                ProficiencyType::WeaponType(WeaponType::Dagger),
                ProficiencyType::WeaponType(WeaponType::Dart),
                ProficiencyType::WeaponType(WeaponType::Sling),
                ProficiencyType::WeaponType(WeaponType::Quarterstaff),
                ProficiencyType::WeaponType(WeaponType::CrossbowLight),
            ]),
        );
        m.insert(
            ClassType::Warlock,
            // Light armor, simple weapons
            Vec::from([
                ProficiencyType::ArmorCategory(ArmorCategory::Light),
                ProficiencyType::WeaponCategory(WeaponCategory::Simple),
            ]),
        );
        m.insert(
            ClassType::Wizard,
            // Daggers, darts, slings, quarterstaffs, light crossbows
            Vec::from([
                ProficiencyType::WeaponType(WeaponType::Dagger),
                ProficiencyType::WeaponType(WeaponType::Dart),
                ProficiencyType::WeaponType(WeaponType::Sling),
                ProficiencyType::WeaponType(WeaponType::Quarterstaff),
                ProficiencyType::WeaponType(WeaponType::CrossbowLight),
            ]),
        );
        m
    })
}
