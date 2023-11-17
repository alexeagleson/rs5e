use rs5e_concepts::{damage_roll::DamageRollType, hit::Hit, roll::Roll, roll_type::RollType};
use rs5e_systems::physical_attack::PhysicalAttackOutcome;
use serde::Serialize;
use std::ops::Deref;
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalAttackOutcomeSummary {
    attack: String,
    attack_roll: String,
    target_ac: String,
    damage_roll: Option<String>,
    hit: String,
    hp_change: Option<String>,
}

impl From<&PhysicalAttackOutcome<'_>> for PhysicalAttackOutcomeSummary {
    fn from(outcome: &PhysicalAttackOutcome<'_>) -> Self {
        // Attack description (weapon vs unarmed)
        let attack = match outcome.state_and_context.state.attacker.weapon {
            Some(attacker_weapon) => {
                format!(
                    "{} attacks {} with {}.",
                    "Attacker",
                    "Defender",
                    attacker_weapon.weapon_type.name()
                )
            }
            None => {
                format!("{} attacks {} unarmed.", "Attacker", "Defender",)
            }
        };

        let weapon_proficiency_bonus = outcome
            .state_and_context
            .context
            .attacker
            .weapon_proficiency_bonus
            .deref()
            .as_ref()
            .map(|b| b.value())
            .unwrap_or(0);

        let ability_modifier = outcome
            .state_and_context
            .context
            .attacker
            .ability_modifier_context
            .ability_modifier
            .value();

        let attack_roll = match &outcome.attack_roll_type {
            RollType::Advantage {
                chosen_roll,
                discarded_roll,
            } => {
                format!(
                     "Attack rolled with advantage lands on {} ({} + {} + {}) and {} ({} + {} + {}). Higher roll {} is taken.",
                     chosen_roll.total_value(),
                     chosen_roll.raw_value(),
                     ability_modifier,
                     weapon_proficiency_bonus,
                     discarded_roll.total_value(),
                     discarded_roll.raw_value(),
                     ability_modifier,
                     weapon_proficiency_bonus,
                     chosen_roll.total_value(),
                 )
            }
            RollType::Normal { roll } => {
                format!(
                    "Attack roll lands on {} ({} + {} + {}).",
                    roll.total_value(),
                    roll.raw_value(),
                    ability_modifier,
                    weapon_proficiency_bonus
                )
            }
            RollType::Disadvantage {
                chosen_roll,
                discarded_roll,
            } => {
                format!(
                    "Attack rolled with disadvantage lands on {} ({} + {} + {}) and {} ({} + {} + {}). Lower roll {} is taken.",
                    chosen_roll.total_value(),
                    chosen_roll.raw_value(),
                    ability_modifier,
                    weapon_proficiency_bonus,
                    discarded_roll.total_value(),
                    discarded_roll.raw_value(),
                    ability_modifier,
                    weapon_proficiency_bonus,
                    chosen_roll.total_value()
                )
            }
        };

        // Target AC
        let target_ac = match outcome.state_and_context.state.defender.equipped_armor {
            Some(armor) => {
                format!(
                    "Target's AC wearing {} is {} ({}+{}).",
                    armor.armor_type.name(),
                    outcome
                        .state_and_context
                        .context
                        .defender
                        .computed_armor_class
                        .value(),
                    outcome
                        .state_and_context
                        .context
                        .defender
                        .base_armor_class
                        .value(),
                    outcome
                        .state_and_context
                        .context
                        .defender
                        .cover_state_armor_class_bonus
                        .value(),
                )
            }
            None => {
                format!(
                    "Target's AC with no armor is {} ({}+{}).",
                    outcome
                        .state_and_context
                        .context
                        .defender
                        .computed_armor_class
                        .value(),
                    outcome
                        .state_and_context
                        .context
                        .defender
                        .cover_state_armor_class_bonus
                        .value(),
                    outcome
                        .state_and_context
                        .context
                        .defender
                        .cover_state_armor_class_bonus
                        .value(),
                )
            }
        };

        // let damage_roll =
        //     if let Some(attacker_weapon) = outcome.state_and_context.state.attacker.weapon {
        //         format!(
        //             "Damage dice {} is rolled for {}.",
        //             attacker_weapon.damage_dice,
        //             attacker_weapon.weapon_type.name(),
        //         )
        //     } else {
        //         format!(
        //             "Damage dice {} is rolled for {}.",
        //             outcome.attack_roll_type.,
        //             attacker_weapon.weapon_type.name(),
        //         )
        //     }

        let (damage_roll, hit, hp_change) = match &outcome.hit {
            Hit::Success {
                damage_roll,
                hp_change,
                damage_type,
            } => match damage_roll.damage_roll_type() {
                DamageRollType::Normal => {
                    let roll = Some(format!(
                        "Damage roll lands on {} ({}+{}).",
                        damage_roll.total_value(),
                        damage_roll.raw_value(),
                        ability_modifier
                    ));

                    let hit = format!(
                        "Attack hits for {} {:?} damage.",
                        damage_roll.total_value(),
                        damage_type
                    );

                    let hp_change = Some(format!(
                        "Target's HP changes from {} to {}.",
                        hp_change.before, hp_change.after
                    ));

                    (roll, hit, hp_change)
                }
                DamageRollType::Critical { bonus_roll_value } => {
                    let roll = Some(format!(
                        "Critical damage roll lands on {} ({}+{}+{}).",
                        damage_roll.total_value(),
                        damage_roll.raw_value(),
                        bonus_roll_value,
                        ability_modifier
                    ));

                    let hit = format!(
                        "Attack critically hits for {} {:?} damage!",
                        damage_roll.total_value(),
                        damage_type
                    );

                    let hp_change = Some(format!(
                        "Target's HP changes from {} to {}.",
                        hp_change.before, hp_change.after
                    ));

                    (roll, hit, hp_change)
                }
            },
            Hit::Miss => {
                let hit = format!("Attack misses!");

                (None, hit, None)
            }
        };

        Self {
            attack,
            attack_roll,
            target_ac,
            damage_roll,
            hit,
            hp_change,
        }
    }
}
