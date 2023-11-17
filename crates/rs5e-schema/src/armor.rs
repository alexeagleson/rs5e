use crate::equipment::{ArmorClassSchema, EquipmentCostSchema, EquipmentSchema, RelationSchema};

#[derive(Debug)]
pub struct ArmorSchema<'a, 'b> {
    pub index: &'a str,
    pub name: &'a str,
    pub url: &'a str,
    pub equipment_category: &'b RelationSchema<'a>,
    pub cost: &'b EquipmentCostSchema,
    pub weight: Option<f32>,

    pub armor_category: &'a str,
    pub armor_class: &'b ArmorClassSchema,
    pub str_minimum: u32,
    pub stealth_disadvantage: bool,
}

impl<'a, 'b> TryFrom<&'b EquipmentSchema<'a>> for ArmorSchema<'a, 'b> {
    type Error = ();

    fn try_from(e: &'b EquipmentSchema<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            index: e.index,
            name: e.name,
            url: e.url,
            equipment_category: &e.equipment_category,
            cost: &e.cost,
            weight: e.weight,

            armor_category: e.armor_category.ok_or(())?,
            armor_class: e.armor_class.as_ref().unwrap(),
            str_minimum: e.str_minimum.unwrap(),
            stealth_disadvantage: e.stealth_disadvantage.unwrap(),
        })
    }
}
