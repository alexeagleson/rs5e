use rs5e_concepts::{
    advantage_type::{AdvantageType, Disadvantage},
    armor_category::ArmorCategory,
    armor_proficiency_penalty::ArmorProficiencyPenalty,
    class_type::ClassType,
    cover_state::{CanBeTargetedCoverBonus, CannotBeTargeted, CoverBonus, CoverState},
    prone_state::{ProneContext, ProneState},
    weapon_category::WeaponCategory,
    weapon_proficiency_bonus::WeaponProficiencyBonus,
    weapon_range::WeaponRange,
};
use std::{borrow::Cow, ops::Deref};

pub trait Describe {
    fn describe(&self) -> Cow<str>;
}

impl Describe for ProneContext {
    fn describe(&self) -> Cow<str> {
        Cow::Borrowed(match self {
            Self::UprightVsUpright => "upright against an upright target",
            Self::UprightVsProne => "upright against a prone target",
            Self::ProneVsProne => "prone against a prone target",
            Self::ProneVsUpright => "prone against an upright target",
        })
    }
}

impl Describe for CoverState {
    fn describe(&self) -> Cow<str> {
        Cow::Borrowed(match self {
            Self::None => "no cover",
            Self::Half => "half cover",
            Self::ThreeQuarters => "three-quarters cover",
            Self::Total => "total cover",
        })
    }
}

impl Describe for CoverBonus {
    fn describe(&self) -> Cow<str> {
        match self {
            CoverBonus::CanBeTargeted(bonus) => bonus.describe(),
            CoverBonus::CannotBeTargeted(cannot_be_targeted) => cannot_be_targeted.describe(),
        }
    }
}

impl Describe for CanBeTargetedCoverBonus {
    fn describe(&self) -> Cow<str> {
        match self {
            CanBeTargetedCoverBonus::None => Cow::Borrowed("no bonus"),
            CanBeTargetedCoverBonus::ArmorClass(ac) => Cow::from(format!("+{} AC", ac.value())),
        }
    }
}

impl Describe for CannotBeTargeted {
    fn describe(&self) -> Cow<str> {
        Cow::Borrowed("the inability to be targeted")
    }
}

impl Describe for AdvantageType {
    fn describe(&self) -> Cow<str> {
        Cow::Borrowed(match self {
            Self::Advantage(_) => "advantage",
            Self::Normal => "neither advantage nor disadvantage",
            Self::Disadvantage(_) => "disadvantage",
        })
    }
}

impl Describe for ArmorProficiencyPenalty {
    fn describe(&self) -> Cow<str> {
        Cow::Borrowed(match self.deref() {
            None => "no proficiency penalty",
            Some(Disadvantage) => "disadvantage",
        })
    }
}

impl Describe for ArmorCategory {
    fn describe(&self) -> Cow<str> {
        Cow::Borrowed(match self {
            Self::Light => "light armor",
            Self::Medium => "medium armor",
            Self::Heavy => "heavy armor",
            Self::Shield => "a shield",
        })
    }
}

impl Describe for ClassType {
    fn describe(&self) -> Cow<str> {
        Cow::Borrowed(match self {
            Self::Barbarian => "a barbarian",
            Self::Bard => "a bard",
            Self::Cleric => "a cleric",
            Self::Druid => "a druid",
            Self::Fighter => "a fighter",
            Self::Monk => "a monk",
            Self::Paladin => "a paladin",
            Self::Ranger => "a ranger",
            Self::Rogue => "a rogue",
            Self::Sorcerer => "a sorcerer",
            Self::Warlock => "a warlock",
            Self::Wizard => "a wizard",
        })
    }
}

impl Describe for ProneState {
    fn describe(&self) -> Cow<str> {
        Cow::Borrowed(match self {
            Self::Upright => "upright",
            Self::Prone => "prone",
        })
    }
}

impl Describe for WeaponCategory {
    fn describe(&self) -> Cow<str> {
        Cow::Borrowed(match self {
            Self::Simple => "a simple weapon",
            Self::Martial => "a martial weapon",
        })
    }
}

impl Describe for WeaponProficiencyBonus {
    fn describe(&self) -> Cow<str> {
        match self.deref() {
            None => Cow::Borrowed("no proficiency bonus"),
            Some(bonus) => Cow::from(format!("a +{} proficiency bonus", bonus.deref())),
        }
    }
}

impl Describe for WeaponRange {
    fn describe(&self) -> Cow<str> {
        Cow::Borrowed(match self {
            Self::Melee => "a melee weapon",
            Self::Ranged => "a ranged weapon",
        })
    }
}
