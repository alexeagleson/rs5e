use crate::equipment::{
    EquipmentCostSchema, EquipmentSchema, RangeSchema, RelationSchema, WeaponDamageSchema,
};

#[derive(Debug)]
pub struct WeaponSchema<'a, 'b> {
    pub index: &'a str,
    pub name: &'a str,
    pub url: &'a str,
    pub equipment_category: &'b RelationSchema<'a>,
    pub cost: &'b EquipmentCostSchema,
    pub weight: Option<f32>,

    pub weapon_category: &'a str,
    pub weapon_range: &'a str,
    pub category_range: &'a str,
    pub damage: &'b WeaponDamageSchema<'a>,
    pub range: &'b RangeSchema,
    pub properties: &'b [RelationSchema<'a>],
}

impl<'a, 'b> TryFrom<&'b EquipmentSchema<'a>> for WeaponSchema<'a, 'b> {
    type Error = ();

    fn try_from(e: &'b EquipmentSchema<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            index: e.index,
            name: e.name,
            url: e.url,
            equipment_category: &e.equipment_category,
            cost: &e.cost,
            weight: e.weight,

            // Failing on this field will gracefully fail and say this is
            // not a weapon. Any other missing field will panic.
            weapon_category: e.weapon_category.ok_or(())?,
            weapon_range: e.weapon_range.unwrap(),
            category_range: e.category_range.unwrap(),

            // The "Net" weapon does not have damage and 
            // we acknowledge this will exclude it and that's fine
            damage: e.damage.as_ref().ok_or(())?,
            range: e.range.as_ref().unwrap(),
            properties: e.properties.as_ref().unwrap(),
        })
    }
}
