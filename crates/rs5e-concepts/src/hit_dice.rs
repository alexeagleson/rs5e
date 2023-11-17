use crate::class_type::ClassType;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::OnceLock};
use typeshare::typeshare;

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HitDie {
    D6,
    D8,
    D10,
    D12,
}

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HitDice {
    pub quantity: u32,
    pub die: HitDie,
}

#[typeshare]
pub type HitDieByClassMap = HashMap<ClassType, HitDie>;

pub fn hit_die_by_class_map() -> &'static HitDieByClassMap {
    static MAP: OnceLock<HitDieByClassMap> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut m = HashMap::with_capacity(12);
        m.insert(ClassType::Barbarian, HitDie::D12);
        m.insert(ClassType::Bard, HitDie::D8);
        m.insert(ClassType::Cleric, HitDie::D8);
        m.insert(ClassType::Druid, HitDie::D8);
        m.insert(ClassType::Fighter, HitDie::D10);
        m.insert(ClassType::Monk, HitDie::D8);
        m.insert(ClassType::Paladin, HitDie::D10);
        m.insert(ClassType::Ranger, HitDie::D10);
        m.insert(ClassType::Rogue, HitDie::D8);
        m.insert(ClassType::Sorcerer, HitDie::D6);
        m.insert(ClassType::Warlock, HitDie::D8);
        m.insert(ClassType::Wizard, HitDie::D6);
        m
    })
}
