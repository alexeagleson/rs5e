use crate::weapon::WeaponModel;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[serde(rename_all = "camelCase", tag = "t", content = "c")]
pub enum PhysicalAttackType {
    Armed(WeaponModel),
    Unarmed,
}
