#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use typeshare::typeshare;

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DamageType {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}

impl DamageType {
    #[must_use]
    pub const fn human_unarmed() -> Self {
        Self::Bludgeoning
    }
}

impl FromStr for DamageType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "acid" => Ok(Self::Acid),
            "bludgeoning" => Ok(Self::Bludgeoning),
            "cold" => Ok(Self::Cold),
            "fire" => Ok(Self::Fire),
            "force" => Ok(Self::Force),
            "lightning" => Ok(Self::Lightning),
            "necrotic" => Ok(Self::Necrotic),
            "piercing" => Ok(Self::Piercing),
            "poison" => Ok(Self::Poison),
            "psychic" => Ok(Self::Psychic),
            "radiant" => Ok(Self::Radiant),
            "slashing" => Ok(Self::Slashing),
            "thunder" => Ok(Self::Thunder),
            _ => Err(()),
        }
    }
}
