use crate::{armor::ArmorEntity, weapon::WeaponEntity};
use rs5e_components::{
    attacker::Attacker, combatant::Combatant, damages::Damages, defender::Defender,
    destroyed::Destroyed, destructible::Destructible, has_abilities::HasAbilities,
    has_armor::HasArmor, has_class::HasClass, has_level::HasLevel, identifiable::Identifiable,
    named::Named,
};
use rs5e_concepts::{
    ability_scores::AbilityScores, class_type::ClassType, cover_state::CoverState,
    damage_source::DamageSource, damage_type::DamageType, hp::Hp, hp_change::HpChange, id::Id,
    level::Level, prone_state::ProneState,
};
use rs5e_dice::{Dice, Die, DieType};
use rs5e_macro_derive::{Identifiable, Named};

const DEFAULT_UNARMED_DAMAGE_SOURCE: DamageSource = DamageSource::Unarmed;
const DEFAULT_UNARMED_DAMAGE_TYPE: DamageType = DamageType::Bludgeoning;
const DEFAULT_UNARMED_DAMAGE_DICE: Dice = Dice::new(1, Die::new(DieType::D1));
const DEFAULT_ATTACK_ROLL_DICE: Dice = Dice::new(1, Die::new(DieType::D20));

#[derive(Debug, Identifiable, Named)]
pub struct DeadUnit {
    pub id: Id,
    pub name: String,
}

impl Destroyed for DeadUnit {}

#[derive(Debug, Identifiable)]
pub struct CharacterEntity<'a> {
    pub id: Id,
    pub name: String,
    pub hp: Hp,
    pub ability_scores: AbilityScores,
    pub class: ClassType,
    pub level: Level,

    pub equipped_weapon: Option<WeaponEntity<'a>>,
    pub equipped_armor: Option<ArmorEntity<'a>>,

    pub cover_state: CoverState,
    pub prone_state: ProneState,
}

impl Damages for CharacterEntity<'_> {
    fn damage_source(&self) -> DamageSource {
        DEFAULT_UNARMED_DAMAGE_SOURCE
    }

    fn damage_type(&self) -> DamageType {
        DEFAULT_UNARMED_DAMAGE_TYPE
    }

    fn damage_dice(&self) -> Dice {
        DEFAULT_UNARMED_DAMAGE_DICE
    }
}

impl Combatant for CharacterEntity<'_> {
    fn prone_state(&self) -> &ProneState {
        &self.prone_state
    }

    fn cover_state(&self) -> &CoverState {
        &self.cover_state
    }
}

impl<'a> HasArmor for CharacterEntity<'a> {
    type Armor = ArmorEntity<'a>;

    fn equipped_armor(&self) -> Option<&Self::Armor> {
        self.equipped_armor.as_ref()
    }
}

impl<'a> Attacker for CharacterEntity<'a> {
    type Weapon = WeaponEntity<'a>;

    fn unmodified_attack_roll(&self) -> u32 {
        DEFAULT_ATTACK_ROLL_DICE.roll()
    }

    fn equipped_weapon(&self) -> Option<&Self::Weapon> {
        self.equipped_weapon.as_ref()
    }
}

impl Defender for CharacterEntity<'_> {}

impl<'a> Destructible for CharacterEntity<'a> {
    type Destroyed = DeadUnit;

    fn hp(&self) -> u32 {
        self.hp.current
    }

    fn max_hp(&self) -> u32 {
        self.hp.max
    }

    fn take_damage(&mut self, damage: u32) -> HpChange {
        let before = self.hp();
        self.hp.current = self.hp.current.saturating_sub(damage);
        HpChange {
            before,
            max: self.max_hp(),
            after: self.hp(),
        }
    }

    fn destroy(self) -> Self::Destroyed {
        // Weapon and armor are lost here so we should acknowledge
        // tht and potentially handle it somehow
        DeadUnit {
            id: self.id,
            name: self.name,
        }
    }
}

impl HasAbilities for CharacterEntity<'_> {
    fn ability_scores(&self) -> &AbilityScores {
        &self.ability_scores
    }

    fn ability_scores_mut(&mut self) -> &mut AbilityScores {
        &mut self.ability_scores
    }
}

impl HasClass for CharacterEntity<'_> {
    fn class(&self) -> &ClassType {
        &self.class
    }
}

impl HasLevel for CharacterEntity<'_> {
    fn level(&self) -> &Level {
        &self.level
    }
}
