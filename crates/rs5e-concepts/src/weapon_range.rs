#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use typeshare::typeshare;

#[derive(Debug)]
pub struct ParseWeaponRangeError;

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WeaponRange {
    Melee,
    Ranged,
}

impl FromStr for WeaponRange {
    type Err = ParseWeaponRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "melee" | "Melee" => Self::Melee,
            "ranged" | "Ranged" => Self::Ranged,
            _ => {
                return Err(ParseWeaponRangeError);
            }
        })
    }
}
