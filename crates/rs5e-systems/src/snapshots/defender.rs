use rs5e_components::{
    armor::Armor, combatant::Combatant, defender::Defender, has_armor::HasArmor,
    identifiable::Identifiable,
};
use rs5e_concepts::{
    armor::ArmorModel, armor_class::ArmorClass, armor_type::ArmorType, cover_state::CoverState,
    id::Id, prone_state::ProneState,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct DefenderSnapshot<'a> {
    pub id: Id,
    pub prone_state: ProneState,
    pub cover_state: CoverState,
    pub armor_class: ArmorClass,
    pub equipped_armor: Option<&'a ArmorModel>,
}

impl<'a, 'b> DefenderSnapshot<'a> {
    pub fn from_defender<D>(
        defender: &'b D,
        armor_model_map: &'a HashMap<ArmorType, ArmorModel>,
    ) -> Self
    where
        D: Defender + Combatant + HasArmor + Identifiable,
    {
        Self {
            id: defender.id(),
            prone_state: defender.prone_state().clone(),
            cover_state: defender.cover_state().clone(),
            armor_class: defender.armor_class().clone(),
            equipped_armor: defender
                .equipped_armor()
                .and_then(|a| armor_model_map.get(a.armor_type())),
        }
    }
}
