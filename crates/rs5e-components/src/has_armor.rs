use crate::armor::Armor;

pub trait HasArmor {
    type Armor: Armor;

    fn equipped_armor(&self) -> Option<&Self::Armor>;
}
