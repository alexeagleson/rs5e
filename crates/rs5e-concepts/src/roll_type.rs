use crate::{advantage_type::AdvantageType, roll::Roll};

#[derive(Debug)]
pub enum RollType<T: Roll> {
    Advantage { chosen_roll: T, discarded_roll: T },
    Normal { roll: T },
    Disadvantage { chosen_roll: T, discarded_roll: T },
}

impl<T: Roll> RollType<T> {
    #[must_use]
    pub const fn chosen_roll(&self) -> &T {
        match self {
            Self::Advantage { chosen_roll, .. }
            | Self::Normal { roll: chosen_roll }
            | Self::Disadvantage { chosen_roll, .. } => chosen_roll,
        }
    }
}

impl<T: Roll, RollFunc: Fn() -> T> From<(AdvantageType, RollFunc)> for RollType<T> {
    fn from((advantage_type, roll): (AdvantageType, RollFunc)) -> Self {
        let base_roll = roll();
        match advantage_type {
            AdvantageType::Advantage(_) => {
                let extra_roll = roll();
                let (chosen_roll, discarded_roll) =
                    if base_roll.total_value() > extra_roll.total_value() {
                        (base_roll, extra_roll)
                    } else {
                        (extra_roll, base_roll)
                    };
                Self::Advantage {
                    chosen_roll,
                    discarded_roll,
                }
            }
            AdvantageType::Normal => Self::Normal { roll: base_roll },
            AdvantageType::Disadvantage(_) => {
                let extra_roll = roll();
                let (chosen_roll, discarded_roll) =
                    if base_roll.total_value() > extra_roll.total_value() {
                        (extra_roll, base_roll)
                    } else {
                        (base_roll, extra_roll)
                    };
                Self::Disadvantage {
                    chosen_roll,
                    discarded_roll,
                }
            }
        }
    }
}
