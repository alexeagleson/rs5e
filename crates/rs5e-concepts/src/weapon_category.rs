#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use typeshare::typeshare;

#[derive(Debug)]
pub struct ParseWeaponCategoryError;

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WeaponCategory {
    Simple,
    Martial,
}

impl FromStr for WeaponCategory {
    type Err = ParseWeaponCategoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "simple" | "Simple" => Self::Simple,
            "martial" | "Martial" => Self::Martial,
            _ => {
                return Err(ParseWeaponCategoryError);
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weapon_category_parse_string() {
        assert_eq!(
            WeaponCategory::from_str("simple").unwrap(),
            WeaponCategory::Simple
        );
        assert_eq!(
            WeaponCategory::from_str("Martial").unwrap(),
            WeaponCategory::Martial
        );
    }
}
