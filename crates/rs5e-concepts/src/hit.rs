use crate::{damage_roll::DamageRoll, damage_type::DamageType, hp_change::HpChange};

#[derive(Debug)]
pub enum Hit {
    Success {
        damage_roll: DamageRoll,
        hp_change: HpChange,
        damage_type: DamageType,
    },
    Miss,
}

impl Hit {
    #[inline]
    #[must_use]
    pub const fn is_miss(&self) -> bool {
        matches!(self, Self::Miss)
    }
}
