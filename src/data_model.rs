use rs5e_concepts::{
    armor::ArmorModel, armor_category::ArmorCategory, armor_class::ArmorClass,
    armor_type::ArmorType, damage_type::DamageType, weapon::WeaponModel,
    weapon_category::WeaponCategory, weapon_range::WeaponRange, weapon_type::WeaponType,
};
use rs5e_dice::Dice;
use rs5e_schema::equipment::EquipmentSchema;
use rs5e_schema::{armor::ArmorSchema, weapon::WeaponSchema};
use std::path::Path;
use std::str::FromStr;

pub(crate) fn read_equipment_file<P>(data_dir: P) -> String
where
    P: AsRef<Path>,
{
    let file_path = data_dir.as_ref().join("5e-SRD-Equipment.json");

    std::fs::read_to_string(file_path).expect("Should have been able to read the file")
}

pub(crate) fn deserialize_equipment(equipment_file_string: &str) -> Vec<EquipmentSchema<'_>> {
    serde_json::from_str(equipment_file_string).unwrap()
}

pub(crate) fn weapon_model_from_weapon_schema(weapon_schema: &WeaponSchema<'_, '_>) -> WeaponModel {
    WeaponModel {
        weapon_type: WeaponType::from_str(weapon_schema.index).unwrap(),
        damage_type: DamageType::from_str(weapon_schema.damage.damage_type.index).unwrap(),
        damage_dice: Dice::from_str(weapon_schema.damage.damage_dice).unwrap(),
        weapon_range: WeaponRange::from_str(weapon_schema.weapon_range).unwrap(),
        weapon_category: WeaponCategory::from_str(weapon_schema.weapon_category).unwrap(),
    }
}

pub(crate) fn armor_model_from_armor_schema(armor_schema: &ArmorSchema<'_, '_>) -> ArmorModel {
    let armor_type = ArmorType::from_str(armor_schema.index).unwrap();
    ArmorModel {
        primary_material: armor_type.primary_material(),
        armor_type,
        armor_class: ArmorClass::new(armor_schema.armor_class.base),
        armor_category: ArmorCategory::from_str(armor_schema.armor_category).unwrap(),
    }
}
