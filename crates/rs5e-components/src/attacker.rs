use super::{damages::Damages, identifiable::Identifiable, weapon::Weapon};
use crate::{
    combatant::Combatant, has_abilities::HasAbilities, has_armor::HasArmor, has_class::HasClass,
    has_level::HasLevel,
};
use rs5e_concepts::{
    ability_modifier::AbilityModifier, attack_roll::AttackRoll, proficiency_bonus::ProficiencyBonus,
};

pub trait Attacker:
    Identifiable + Damages + Combatant + HasArmor + HasLevel + HasClass + HasAbilities
{
    type Weapon: Weapon;

    fn unmodified_attack_roll(&self) -> u32;

    fn equipped_weapon(&self) -> Option<&Self::Weapon>;

    fn roll_attack(
        &self,
        ability_modifier: AbilityModifier,
        proficiency_bonus: Option<ProficiencyBonus>,
    ) -> AttackRoll {
        AttackRoll::new(
            self.unmodified_attack_roll(),
            ability_modifier,
            proficiency_bonus,
        )
    }
}

#[cfg(any(test, feature = "test"))]
pub mod mocks {
    use crate::armor::mocks::MockArmor;

    #[allow(clippy::wildcard_imports)]
    use super::*;
    use rs5e_concepts::{
        ability_scores::AbilityScores,
        class_type::ClassType,
        cover_state::{CoverState, DEFAULT_COVER_STATE},
        damage_source::DamageSource,
        damage_type::DamageType,
        id::Id,
        level::Level,
        prone_state::{ProneState, DEFAULT_PRONE_STATE},
    };
    use rs5e_dice::{Dice, Die, DieLoading, DieType};

    #[derive(Debug)]
    pub struct MockAttacker<W: Weapon> {
        id: Id,
        equipped_weapon: W,
        level: Level,
        class: ClassType,
        ability_scores: AbilityScores,
    }

    impl<W: Weapon> Identifiable for MockAttacker<W> {
        fn id(&self) -> Id {
            self.id
        }

        fn set_id(&mut self, new_id: Id) {
            self.id = new_id;
        }
    }

    impl<W: Weapon> Damages for MockAttacker<W> {
        fn damage_source(&self) -> DamageSource {
            DamageSource::Weapon
        }

        fn damage_type(&self) -> DamageType {
            self.equipped_weapon.damage_type()
        }

        fn damage_dice(&self) -> Dice {
            self.equipped_weapon.damage_dice()
        }
    }

    impl<W: Weapon> Combatant for MockAttacker<W> {
        fn prone_state(&self) -> &ProneState {
            &DEFAULT_PRONE_STATE
        }

        fn cover_state(&self) -> &CoverState {
            &DEFAULT_COVER_STATE
        }
    }

    impl<W: Weapon> HasArmor for MockAttacker<W> {
        type Armor = MockArmor;

        fn equipped_armor(&self) -> Option<&Self::Armor> {
            None
        }
    }

    impl<W: Weapon> HasLevel for MockAttacker<W> {
        fn level(&self) -> &Level {
            &self.level
        }
    }

    impl<W: Weapon> HasClass for MockAttacker<W> {
        fn class(&self) -> &ClassType {
            &self.class
        }
    }

    impl<W: Weapon> HasAbilities for MockAttacker<W> {
        fn ability_scores(&self) -> &AbilityScores {
            &self.ability_scores
        }

        fn ability_scores_mut(&mut self) -> &mut AbilityScores {
            &mut self.ability_scores
        }
    }

    impl<W: Weapon> Attacker for MockAttacker<W> {
        type Weapon = W;

        fn unmodified_attack_roll(&self) -> u32 {
            Die::new_loaded(DieType::D20, DieLoading::MaximumMinusOne).roll()
        }

        fn equipped_weapon(&self) -> Option<&Self::Weapon> {
            Some(&self.equipped_weapon)
        }
    }

    impl<W: Weapon> MockAttacker<W> {
        pub fn new_with_weapon(equipped_weapon: W) -> Self {
            Self {
                id: Id::new_incremental(),
                equipped_weapon,
                level: Level::default(),
                class: ClassType::Barbarian,
                ability_scores: AbilityScores::default(),
            }
        }
    }
}
