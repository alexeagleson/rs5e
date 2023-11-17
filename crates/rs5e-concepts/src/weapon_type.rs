use crate::util::macros::implement_from_str;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[serde(rename_all = "camelCase")]
pub enum WeaponType {
    Battleaxe,
    Blowgun,
    Club,
    CrossbowHand,
    CrossbowHeavy,
    CrossbowLight,
    Dagger,
    Dart,
    Flail,
    Glaive,
    Greataxe,
    Greatclub,
    Greatsword,
    Halberd,
    Handaxe,
    Javelin,
    Lance,
    LightHammer,
    Longbow,
    Longsword,
    Mace,
    Maul,
    Morningstar,
    Pike,
    Quarterstaff,
    Rapier,
    Scimitar,
    Shortbow,
    Shortsword,
    Sickle,
    Sling,
    Spear,
    Trident,
    WarPick,
    Warhammer,
    Whip,
}

impl WeaponType {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Battleaxe => "Battleaxe",
            Self::Blowgun => "Blowgun",
            Self::Club => "Club",
            Self::CrossbowHand => "Crossbow Hand",
            Self::CrossbowHeavy => "Crossbow Heavy",
            Self::CrossbowLight => "Crossbow Light",
            Self::Dagger => "Dagger",
            Self::Dart => "Dart",
            Self::Flail => "Flail",
            Self::Glaive => "Glaive",
            Self::Greataxe => "Greataxe",
            Self::Greatclub => "Greatclub",
            Self::Greatsword => "Greatsword",
            Self::Halberd => "Halberd",
            Self::Handaxe => "Handaxe",
            Self::Javelin => "Javelin",
            Self::Lance => "Lance",
            Self::LightHammer => "LightHammer",
            Self::Longbow => "Longbow",
            Self::Longsword => "Longsword",
            Self::Mace => "Mace",
            Self::Maul => "Maul",
            Self::Morningstar => "Morningstar",
            Self::Pike => "Pike",
            Self::Quarterstaff => "Quarterstaff",
            Self::Rapier => "Rapier",
            Self::Scimitar => "Scimitar",
            Self::Shortbow => "Shortbow",
            Self::Shortsword => "Shortsword",
            Self::Sickle => "Sickle",
            Self::Sling => "Sling",
            Self::Spear => "Spear",
            Self::Trident => "Trident",
            Self::WarPick => "WarPick",
            Self::Warhammer => "Warhammer",
            Self::Whip => "Whip",
        }
    }
}

implement_from_str!(
    WeaponType,
    [
        ["battleaxe", Battleaxe],
        ["blowgun", Blowgun],
        ["club", Club],
        ["crossbow-hand", CrossbowHand],
        ["crossbow-heavy", CrossbowHeavy],
        ["crossbow-light", CrossbowLight],
        ["dagger", Dagger],
        ["dart", Dart],
        ["flail", Flail],
        ["glaive", Glaive],
        ["greataxe", Greataxe],
        ["greatclub", Greatclub],
        ["greatsword", Greatsword],
        ["halberd", Halberd],
        ["handaxe", Handaxe],
        ["javelin", Javelin],
        ["lance", Lance],
        ["light-hammer", LightHammer],
        ["longbow", Longbow],
        ["longsword", Longsword],
        ["mace", Mace],
        ["maul", Maul],
        ["morningstar", Morningstar],
        ["pike", Pike],
        ["quarterstaff", Quarterstaff],
        ["rapier", Rapier],
        ["scimitar", Scimitar],
        ["shortbow", Shortbow],
        ["shortsword", Shortsword],
        ["sickle", Sickle],
        ["sling", Sling],
        ["spear", Spear],
        ["trident", Trident],
        ["war-pick", WarPick],
        ["warhammer", Warhammer],
        ["whip", Whip]
    ]
);