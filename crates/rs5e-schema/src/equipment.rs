use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CurrencyUnitSchema {
    /// Copper pieces
    Cp,
    /// Silver pieces
    Sp,
    /// Gold pieces
    Gp,
}

#[derive(Debug, Deserialize)]
pub struct WeaponDamageSchema<'a> {
    // Had to modify blowgun from 1 to 1d1
    // In the JSON data to fit the schema
    pub damage_dice: &'a str,
    pub damage_type: RelationSchema<'a>,
}

#[derive(Debug, Deserialize)]
pub struct RangeSchema {
    normal: u32,
    #[serde(default)]
    long: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ArmorClassSchema {
    pub base: u32,
    #[serde(default)]
    pub dex_bonus: bool,
    #[serde(default)]
    pub max_bonus: u32,
}

#[derive(Debug, Deserialize)]
pub struct EquipmentCostSchema {
    quantity: u32,
    unit: CurrencyUnitSchema,
}

#[derive(Debug, Deserialize)]
pub struct RelationSchema<'a> {
    pub index: &'a str,
    pub name: &'a str,
    pub url: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct EquipmentSchema<'a> {
    pub index: &'a str,
    pub name: &'a str,
    pub url: &'a str,
    #[serde(borrow)]
    pub equipment_category: RelationSchema<'a>,
    pub cost: EquipmentCostSchema,
    pub weight: Option<f32>,

    // Weapon
    pub weapon_category: Option<&'a str>,
    pub weapon_range: Option<&'a str>,
    pub category_range: Option<&'a str>,
    // "Net" weapon does not have damage so optional here
    #[serde(borrow)]
    pub damage: Option<WeaponDamageSchema<'a>>,
    pub range: Option<RangeSchema>,
    #[serde(borrow)]
    pub properties: Option<Vec<RelationSchema<'a>>>,

    // Armor
    pub armor_category: Option<&'a str>,
    pub armor_class: Option<ArmorClassSchema>,
    pub str_minimum: Option<u32>,
    pub stealth_disadvantage: Option<bool>,
}
