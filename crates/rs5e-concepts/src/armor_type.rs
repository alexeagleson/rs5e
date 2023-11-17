#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use crate::{material::Material, util::macros::implement_from_str};

#[typeshare]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[serde(rename_all = "camelCase")]
pub enum ArmorType {
    Breastplate,
    ChainMail,
    ChainShirt,
    HalfPlateArmor,
    HideArmor,
    LeatherArmor,
    PaddedArmor,
    PlateArmor,
    RingMail,
    ScaleMail,
    Shield,
    SplintArmor,
    StuddedLeatherArmor,
}

impl  ArmorType {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Breastplate => "Breastplate",
            Self::ChainMail => "Chain Mail",
            Self::ChainShirt => "Chain Shirt",
            Self::HalfPlateArmor => "Half Plate Armor",
            Self::HideArmor => "Hide Armor",
            Self::LeatherArmor => "Leather Armor",
            Self::PaddedArmor => "Padded Armor",
            Self::PlateArmor => "Plate Armor",
            Self::RingMail => "Ring Mail",
            Self::ScaleMail => "Scale Mail",
            Self::Shield => "Shield",
            Self::SplintArmor => "Splint Armor",
            Self::StuddedLeatherArmor => "Studded Leather Armor",
        }
    }
}

impl ArmorType {
    #[must_use]
    /// Resources:
    /// <https://rpg.stackexchange.com/questions/105486/what-is-metal-armor>
    /// <https://roll20.net/compendium/dnd5e/Armor#content>
    pub const fn primary_material(&self) -> Material {
        match self {
            | Self::HideArmor
            | Self::LeatherArmor
            | Self::PaddedArmor
            | Self::RingMail
            // Rules say shields can technically come in metal or wood variants
            // we are defaulting to wood for this basic one
            | Self::Shield => Material::Unknown,
            Self::Breastplate 
            | Self::ChainMail 
            | Self::ChainShirt 
            | Self::HalfPlateArmor  
            | Self::PlateArmor 
            | Self::ScaleMail  
            | Self::SplintArmor 
            | Self::StuddedLeatherArmor => Material::Metal,
        }
    }
}

implement_from_str!(
    ArmorType,
    [
        ["breastplate", Breastplate],
        ["chain-mail", ChainMail],
        ["chain-shirt", ChainShirt],
        ["half-plate-armor", HalfPlateArmor],
        ["hide-armor", HideArmor],
        ["leather-armor", LeatherArmor],
        ["padded-armor", PaddedArmor],
        ["plate-armor", PlateArmor],
        ["ring-mail", RingMail],
        ["scale-mail", ScaleMail],
        ["shield", Shield],
        ["splint-armor", SplintArmor],
        ["studded-leather-armor", StuddedLeatherArmor]
    ]
);
