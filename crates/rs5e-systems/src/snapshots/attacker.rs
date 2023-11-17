use rs5e_components::{armor::Armor, attacker::Attacker, weapon::Weapon};
use rs5e_concepts::{
    ability_scores::AbilityScores, armor::ArmorModel, armor_type::ArmorType, class_type::ClassType,
    id::Id, level::Level, prone_state::ProneState, weapon::WeaponModel, weapon_type::WeaponType,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct AttackerSnapshot<'a> {
    pub id: Id,
    pub prone_state: ProneState,
    pub level: Level,
    pub class: ClassType,
    pub ability_scores: AbilityScores,
    pub weapon: Option<&'a WeaponModel>,
    pub armor: Option<&'a ArmorModel>,
}

impl<'a, 'b> AttackerSnapshot<'a> {
    pub fn from_attacker<A>(
        attacker: &'b A,
        weapon_model_map: &'a HashMap<WeaponType, WeaponModel>,
        armor_model_map: &'a HashMap<ArmorType, ArmorModel>,
    ) -> Self
    where
        A: Attacker,
    {
        Self {
            id: attacker.id(),
            prone_state: attacker.prone_state().clone(),
            level: attacker.level().clone(),
            class: attacker.class().clone(),
            ability_scores: attacker.ability_scores().clone(),
            weapon: attacker
                .equipped_weapon()
                .map(|w| weapon_model_map.get(w.weapon_type()).unwrap()),
            armor: attacker
                .equipped_armor()
                .map(|a| armor_model_map.get(a.armor_type()).unwrap()),
        }
    }
}
