#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use typeshare::typeshare;

#[derive(Debug)]
pub struct ParseArmorCategoryError;

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ArmorCategory {
    Light,
    Medium,
    Heavy,
    Shield,
}

impl FromStr for ArmorCategory {
    type Err = ParseArmorCategoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "light" | "Light" => Self::Light,
            "medium" | "Medium" => Self::Medium,
            "heavy" | "Heavy" => Self::Heavy,
            "shield" | "Shield" => Self::Shield,
            _ => {
                return Err(ParseArmorCategoryError);
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn armor_category_parse_string() {
        assert_eq!(
            ArmorCategory::from_str("shield").unwrap(),
            ArmorCategory::Shield
        );
        assert_eq!(
            ArmorCategory::from_str("Heavy").unwrap(),
            ArmorCategory::Heavy
        );
    }
}
