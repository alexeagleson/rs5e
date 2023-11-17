#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
// #![warn(missing_docs)]

use rand::Rng;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use typeshare::typeshare;

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum DieLoading {
    Minimum,
    MinimumPlusOne,
    AverageRoundedDown,
    MaximumMinusOne,
    Maximum,
}

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum DieType {
    D1,
    D2,
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

impl Display for DieType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl DieType {
    const fn max_value(&self) -> u32 {
        match self {
            Self::D1 => 1,
            Self::D2 => 2,
            Self::D4 => 4,
            Self::D6 => 6,
            Self::D8 => 8,
            Self::D10 => 10,
            Self::D12 => 12,
            Self::D20 => 20,
        }
    }

    #[allow(clippy::unused_self)]
    const fn min_value(&self) -> u32 {
        1
    }

    /// Rounded down
    const fn avg_value(&self) -> u32 {
        self.max_value() / 2
    }
}

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Die {
    die_type: DieType,
    #[cfg(feature = "loaded-dice")]
    die_loading: Option<DieLoading>,
}

impl Display for Die {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.die_type)
    }
}

impl Die {
    #[must_use]
    pub fn roll(&self) -> u32 {
        #[cfg(feature = "loaded-dice")]
        if let Some(die_loading) = self.die_loading {
            return match die_loading {
                DieLoading::Minimum => self.die_type.min_value(),
                DieLoading::MinimumPlusOne => self.die_type.min_value() + 1,
                DieLoading::AverageRoundedDown => self.die_type.avg_value(),
                DieLoading::MaximumMinusOne => self.die_type.max_value() - 1,
                DieLoading::Maximum => self.die_type.max_value(),
            };
        }

        let mut rng = rand::thread_rng();
        rng.gen_range(1..=self.die_type.max_value())
    }

    #[must_use]
    pub const fn new(die_type: DieType) -> Self {
        Self {
            die_type,
            #[cfg(feature = "loaded-dice")]
            die_loading: None,
        }
    }

    #[cfg(feature = "loaded-dice")]
    #[must_use]
    pub const fn new_loaded(die_type: DieType, die_loading: DieLoading) -> Self {
        Self {
            die_type,
            die_loading: Some(die_loading),
        }
    }
}

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Eq, Clone, Copy)]
pub struct Dice {
    quantity: u32,
    die: Die,
}

impl Display for Dice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.quantity, self.die)
    }
}

impl PartialEq for Dice {
    fn eq(&self, other: &Self) -> bool {
        // This is not strictly true given that 2D6 is a lower expected value than 3D4
        // but it's close enough for our current needs
        self.max_value() == other.max_value()
    }
}

impl PartialOrd for Dice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Dice {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.max_value().cmp(&other.max_value())
    }
}

impl From<Die> for Dice {
    fn from(die: Die) -> Self {
        Self { quantity: 1, die }
    }
}

impl Dice {
    #[inline]
    #[must_use]
    pub fn roll(&self) -> u32 {
        (1..=self.quantity).fold(0, |acc, _| acc + self.die.roll())
    }

    /// The maximum value these dice can roll
    #[inline]
    #[must_use]
    pub const fn max_value(&self) -> u32 {
        self.quantity * self.die.die_type.max_value()
    }

    /// The minimum value these dice can roll
    #[inline]
    #[must_use]
    pub const fn min_value(&self) -> u32 {
        self.quantity * self.die.die_type.min_value()
    }

    /// The average (rounded down for each individual die)
    /// value these dice can roll
    #[inline]
    #[must_use]
    pub const fn avg_value(&self) -> u32 {
        self.quantity * self.die.die_type.avg_value()
    }

    /// Crate a new set of dice
    #[inline]
    #[must_use]
    pub const fn new(quantity: u32, die: Die) -> Self {
        Self { quantity, die }
    }
}

#[derive(Debug)]
pub enum ParseDiceStringError {
    /// Dice string should be in format `1d6` or `1D6`
    InvalidFormat,
    /// Could not parse first digit, e.g. the 1 in `1d6`
    CannotParseFirstDigit,
    /// Could not parse first second, e.g. the 6 in `1d6`
    CannotParseSecondDigit,
    /// Die did not match a supported size of [`Die`]
    UnsupportedDieSize,
}

impl FromStr for Dice {
    type Err = ParseDiceStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = if s.contains('d') {
            s.split('d')
        } else if s.contains('D') {
            s.split('D')
        } else {
            return Err(ParseDiceStringError::InvalidFormat);
        };

        let num_dice = split
            .next()
            .ok_or(ParseDiceStringError::CannotParseFirstDigit)?
            .parse::<u32>()
            .map_err(|_| ParseDiceStringError::CannotParseFirstDigit)?;

        let dice_size = split
            .next()
            .ok_or(ParseDiceStringError::CannotParseSecondDigit)?
            .parse::<u32>()
            .map_err(|_| ParseDiceStringError::CannotParseSecondDigit)?;

        Ok(Self {
            quantity: num_dice,
            die: Die {
                die_type: match dice_size {
                    1 => DieType::D1,
                    2 => DieType::D2,
                    4 => DieType::D4,
                    6 => DieType::D6,
                    8 => DieType::D8,
                    10 => DieType::D10,
                    12 => DieType::D12,
                    20 => DieType::D20,
                    _ => {
                        return Err(ParseDiceStringError::UnsupportedDieSize);
                    }
                },
                #[cfg(feature = "loaded-dice")]
                die_loading: None,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn die_ordering() {
        assert!(DieType::D2 < DieType::D8);
        assert!(DieType::D8 < DieType::D20);
        assert!(DieType::D6 < DieType::D12);
        assert!(DieType::D20 > DieType::D2);
    }

    #[test]
    fn dice_ordering() {
        let two_d6 = Dice::new(2, Die::new(DieType::D6));
        let three_d4 = Dice::new(3, Die::new(DieType::D4));
        let one_d20 = Dice::new(1, Die::new(DieType::D20));
        let five_d2 = Dice::new(5, Die::new(DieType::D2));

        assert!(two_d6 == three_d4);
        assert!(two_d6 < one_d20);
        assert!(five_d2 < three_d4);
    }

    #[test]
    fn die_display() {
        assert_eq!(DieType::D12.to_string(), "D12".to_string());
    }

    #[test]
    fn dice_display() {
        let two_d6 = Dice::new(2, Die::new(DieType::D6));
        let three_d4 = Dice::new(3, Die::new(DieType::D4));
        let one_d20 = Dice::new(1, Die::new(DieType::D20));
        let five_d2 = Dice::new(5, Die::new(DieType::D2));

        assert_eq!(two_d6.to_string(), "2D6".to_string());
        assert_eq!(three_d4.to_string(), "3D4".to_string());
        assert_eq!(one_d20.to_string(), "1D20".to_string());
        assert_eq!(five_d2.to_string(), "5D2".to_string());
    }

    #[test]
    fn parse_lowercase_dice_str() {
        let dice = Dice::from_str("4d6").expect("Should be valid");
        assert_eq!(dice.quantity, 4);
        assert_eq!(dice.die.die_type, DieType::D6);
    }

    #[test]
    #[cfg(feature = "loaded-dice")]
    fn maximum_loaded_die_works() {
        let die = Die {
            die_type: DieType::D20,
            die_loading: Some(DieLoading::Maximum),
        };
        let result = die.roll();
        assert_eq!(result, 20);
    }

    #[test]
    #[cfg(feature = "loaded-dice")]
    fn maximum_minus_one_loaded_die_works() {
        let die = Die {
            die_type: DieType::D20,
            die_loading: Some(DieLoading::MaximumMinusOne),
        };
        let result = die.roll();
        assert_eq!(result, 19);
    }

    #[test]
    #[cfg(feature = "loaded-dice")]
    fn maximum_minus_one_loaded_dice_works() {
        let dice = Dice {
            quantity: 5,
            die: Die {
                die_type: DieType::D20,
                die_loading: Some(DieLoading::MaximumMinusOne),
            },
        };
        let result = dice.roll();
        assert_eq!(result, 95);
    }
}
