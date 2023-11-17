use crate::{describe::Describe, describe_physical_attack_outcome::PhysicalAttackOutcomeSummary};
use rs5e_concepts::weapon_ability_modifier::AbilityModifierContext;
use rs5e_systems::physical_attack::PhysicalAttackStateAndContext;
use serde::Serialize;
use std::{borrow::Cow, ops::Deref};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase", tag = "t", content = "c")]
pub enum PhysicalAttackAttemptSummary {
    Failure(String),
    Success {
        // before: Box<PhysicalAttackSummary>,
        after: Box<PhysicalAttackOutcomeSummary>,
    },
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalAttackSummary {
    attacker_prone_context_bonus: String,
    defender_bonus_fom_cover_state: String,
    attacker_weapon_proficiency_bonus: String,
    attacker_armor_proficiency_penalty: String,
    attacker_weapon_ability_modifier: String,
    attacker_combined_advantage_type: String,
}

impl From<&PhysicalAttackStateAndContext<'_>> for PhysicalAttackSummary {
    fn from(value: &PhysicalAttackStateAndContext) -> Self {
        fn describe_attacker_prone_context_bonus(
            state_and_context: &PhysicalAttackStateAndContext,
        ) -> String {
            let prone_bonus_phrase = state_and_context
                .context
                .attacker
                .prone_context_bonus
                .describe();
            let prone_context_phrase = state_and_context.context.prone.describe();

            format!("You have {prone_bonus_phrase} for being {prone_context_phrase}.")
        }

        fn describe_defender_bonus_fom_cover_state(
            state_and_context: &PhysicalAttackStateAndContext,
        ) -> String {
            let cover_state_phrase = state_and_context.state.defender.cover_state.describe();
            let bonus_from_cover_state_phrase = state_and_context
                .context
                .defender
                .cover_state_bonus
                .describe();
            format!(
                "Your target has {bonus_from_cover_state_phrase} for having {cover_state_phrase}."
            )
        }

        fn describe_attacker_weapon_proficiency_bonus(
            state_and_context: &PhysicalAttackStateAndContext,
        ) -> String {
            let weapon_phrase = match state_and_context.state.attacker.weapon.as_ref() {
                Some(weapon) => Cow::from(format!(
                    "using {} as {}",
                    weapon.weapon_category.describe(),
                    state_and_context.state.attacker.class.describe()
                )),
                None => Cow::from("attacking unarmed"),
            };

            let level_phrase = match state_and_context
                .context
                .attacker
                .weapon_proficiency_bonus
                .deref()
            {
                None => Cow::from(""),
                Some(_) => Cow::from(format!(
                    " at level {}",
                    state_and_context.state.attacker.level.deref()
                )),
            };

            let proficiency_bonus_phrase = state_and_context
                .context
                .attacker
                .weapon_proficiency_bonus
                .describe();

            format!("You have {proficiency_bonus_phrase} for {weapon_phrase}{level_phrase}.")
        }

        fn describe_attacker_armor_proficiency_penalty(
            state_and_context: &PhysicalAttackStateAndContext,
        ) -> String {
            let armor_phrase = match state_and_context.state.attacker.armor.as_ref() {
                Some(armor) => Cow::from(format!(
                    "wearing {} as {}",
                    armor.armor_category.describe(),
                    state_and_context.state.attacker.class.describe()
                )),
                None => Cow::from("being unarmoured"),
            };

            let armor_proficiency_penalty_phrase = state_and_context
                .context
                .attacker
                .armor_proficiency_penalty
                .describe();

            format!("You have {armor_proficiency_penalty_phrase} for {armor_phrase}.")
        }

        fn describe_attacker_weapon_ability_modifier(
            state_and_context: &PhysicalAttackStateAndContext,
        ) -> String {
            let AbilityModifierContext {
                ability_modifier,
                ability_score,
                ability_type,
            } = &state_and_context.context.attacker.ability_modifier_context;

            let ability_modifier_value = ability_modifier.deref().to_owned();
            let ability_score_value = ability_score.deref().to_owned();

            let weapon_phrase = match state_and_context.state.attacker.weapon.as_ref() {
                Some(weapon) => Cow::from(format!("with {}", weapon.weapon_range.describe())),
                None => Cow::from("unarmed"),
            };

            let sign = if ability_modifier_value >= 0 { "+" } else { "" };

            format!("You have a {sign}{ability_modifier_value} ability modifier for attacking {weapon_phrase} at {ability_score_value} {ability_type:?}.")
        }

        fn describe_attacker_combined_advantage_type(
            state_and_context: &PhysicalAttackStateAndContext,
        ) -> String {
            let advantage_type_phrase = state_and_context
                .context
                .attacker
                .computed_advantage_type
                .describe();

            format!("You will roll with {advantage_type_phrase}.")
        }

        Self {
            attacker_prone_context_bonus: describe_attacker_prone_context_bonus(value),
            defender_bonus_fom_cover_state: describe_defender_bonus_fom_cover_state(value),
            attacker_weapon_proficiency_bonus: describe_attacker_weapon_proficiency_bonus(value),
            attacker_armor_proficiency_penalty: describe_attacker_armor_proficiency_penalty(value),
            attacker_weapon_ability_modifier: describe_attacker_weapon_ability_modifier(value),
            attacker_combined_advantage_type: describe_attacker_combined_advantage_type(value),
        }
    }
}
