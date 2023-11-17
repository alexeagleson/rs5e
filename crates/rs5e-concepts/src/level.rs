use std::ops::Deref;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

pub const MIN_LEVEL: u32 = 1;
pub const MAX_LEVEL: u32 = 20;
pub const DEFAULT_LEVEL: u32 = 1;

#[derive(Debug)]
pub struct InvalidLevel;

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Level(u32);

impl TryFrom<u32> for Level {
    type Error = InvalidLevel;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        (MIN_LEVEL..=MAX_LEVEL)
            .contains(&value)
            .then_some(Self(value))
            .ok_or(InvalidLevel)
    }
}

impl Default for Level {
    fn default() -> Self {
        Self(DEFAULT_LEVEL)
    }
}

impl Deref for Level {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
