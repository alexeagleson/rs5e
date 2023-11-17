#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use typeshare::typeshare;

// Rule as written default is 10 + DEX modifier
pub const DEFAULT_ARMOR_CLASS: ArmorClass = ArmorClass(10);

#[typeshare]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct ArmorClass(u32);

impl Default for ArmorClass {
    fn default() -> Self {
        DEFAULT_ARMOR_CLASS
    }
}

impl ArmorClass {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn value(&self) -> u32 {
        self.0
    }
}

impl Deref for ArmorClass {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
