#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase", tag = "t", content = "c")]
pub enum AdvantageType {
    Advantage(Advantage),
    Normal,
    Disadvantage(Disadvantage),
}

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Advantage;

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Disadvantage;

impl AdvantageType {
    #[must_use]
    pub fn from_all_sources(sources: &[&Self]) -> Self {
        let mut current: Self = Self::Normal;
        for advantage_type in sources {
            match (&current, advantage_type) {
                (Self::Advantage(_), Self::Disadvantage(_))
                | (Self::Disadvantage(_), Self::Advantage(_)) => return Self::Normal,
                (Self::Normal, Self::Advantage(_) | Self::Disadvantage(_)) => {
                    current = (*advantage_type).clone();
                }
                _ => {
                    continue;
                }
            }
        }
        current
    }
}
