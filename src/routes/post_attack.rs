use crate::{character_from_builder, AppState, CharacterBuilder};
use axum::{extract::State, http::StatusCode, Json};
use rs5e_concepts::cover_state::CoverState;
use rs5e_log::{
    describe_physical_attack_context::{PhysicalAttackAttemptSummary, PhysicalAttackSummary},
    describe_physical_attack_outcome::PhysicalAttackOutcomeSummary,
};
use rs5e_systems::{
    physical_attack::{
        physical_attack_system, PhysicalAttackContext, PhysicalAttackState,
        PhysicalAttackStateAndContext,
    },
    snapshots::{attacker::AttackerSnapshot, defender::DefenderSnapshot},
};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CombatSummary {
    combatant_1_summary: Option<PhysicalAttackSummary>,
    combatant_2_summary: Option<PhysicalAttackSummary>,
    attacks: Vec<PhysicalAttackAttemptSummaryWithPhrases>,
    outcome: String,
}

#[typeshare]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalAttackAttemptSummaryWithPhrases {
    before_phrase: String,
    summary: PhysicalAttackAttemptSummary,
    after_phrase: String,
}

#[typeshare]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AttackRequest {
    attacker: CharacterBuilder,
    target: CharacterBuilder,
}

pub(crate) async fn post_attack(
    State(state): State<AppState>,
    Json(payload): Json<AttackRequest>,
) -> (StatusCode, Json<CombatSummary>) {
    let mut attacks = Vec::new();

    let mut combatant_1_entity = character_from_builder(
        payload.attacker,
        state.weapon_model_map.as_ref(),
        state.armor_model_map.as_ref(),
    );

    let mut combatant_2_entity = character_from_builder(
        payload.target,
        state.weapon_model_map.as_ref(),
        state.armor_model_map.as_ref(),
    );

    if combatant_1_entity.cover_state == CoverState::Total
        && combatant_2_entity.cover_state == CoverState::Total
    {
        return (
            StatusCode::OK,
            axum::Json(CombatSummary {
                combatant_1_summary: None,
                combatant_2_summary: None,
                attacks: Vec::new(),
                outcome: "They both have full cover what are you trying to do".to_string(),
            }),
        );
    };

    let combatant_1_state = PhysicalAttackState {
        attacker: AttackerSnapshot::from_attacker(
            &combatant_1_entity,
            state.weapon_model_map.as_ref(),
            state.armor_model_map.as_ref(),
        ),
        defender: DefenderSnapshot::from_defender(
            &combatant_2_entity,
            state.armor_model_map.as_ref(),
        ),
    };

    let combatant_1_context = match PhysicalAttackContext::try_from(&combatant_1_state) {
        Ok(context) => context,
        Err(_cannot_be_targeted) => {
            return (
                StatusCode::OK,
                axum::Json(CombatSummary {
                    combatant_1_summary: None,
                    combatant_2_summary: None,
                    attacks: Vec::new(),
                    outcome: format!(
                        "{} is covered, cannot attempt attack.",
                        combatant_2_entity.name
                    ),
                }),
            );
        }
    };

    let combatant_2_state = PhysicalAttackState {
        attacker: AttackerSnapshot::from_attacker(
            &combatant_2_entity,
            state.weapon_model_map.as_ref(),
            state.armor_model_map.as_ref(),
        ),
        defender: DefenderSnapshot::from_defender(
            &combatant_1_entity,
            state.armor_model_map.as_ref(),
        ),
    };

    let combatant_2_context = match PhysicalAttackContext::try_from(&combatant_2_state) {
        Ok(context) => context,
        Err(_cannot_be_targeted) => {
            return (
                StatusCode::OK,
                axum::Json(CombatSummary {
                    combatant_1_summary: Some(
                        (&PhysicalAttackStateAndContext {
                            state: combatant_1_state,
                            context: combatant_1_context,
                        })
                            .into(),
                    ),
                    combatant_2_summary: None,
                    attacks: Vec::new(),
                    outcome: format!(
                        "{} is covered, cannot attempt attack.",
                        combatant_1_entity.name
                    ),
                }),
            );
        }
    };

    let mut combatant_1_state_and_context = PhysicalAttackStateAndContext {
        state: combatant_1_state,
        context: combatant_1_context,
    };

    let mut combatant_2_state_and_context = PhysicalAttackStateAndContext {
        state: combatant_2_state,
        context: combatant_2_context,
    };

    let mut loop_count = 0;
    let outcome;

    loop {
        let physical_attack_outcome = physical_attack_system(
            &combatant_1_entity,
            &mut combatant_2_entity,
            &combatant_1_state_and_context,
        );

        let physical_attack_outcome_summary =
            PhysicalAttackOutcomeSummary::from(&physical_attack_outcome);

        attacks.push(PhysicalAttackAttemptSummaryWithPhrases {
            before_phrase: format!(
                "⚔️ {} attacks {}!",
                combatant_1_entity.name, combatant_2_entity.name
            ),
            after_phrase: format!(
                "{} has {} HP remaining!",
                combatant_2_entity.name, combatant_2_entity.hp.current
            ),
            summary: PhysicalAttackAttemptSummary::Success {
                after: Box::new(physical_attack_outcome_summary),
            },
        });

        if combatant_2_entity.hp.current == 0 {
            outcome = format!("{} wins!", combatant_1_entity.name);
            break;
        }

        std::mem::swap(
            &mut combatant_1_state_and_context,
            &mut combatant_2_state_and_context,
        );
        std::mem::swap(&mut combatant_1_entity, &mut combatant_2_entity);

        loop_count += 1;

        if loop_count > 50 {
            outcome = "Combat has gone on for over 100 rounds, that's way too long, I'm ending it."
                .to_string();
            break;
        }
    }

    (
        StatusCode::OK,
        axum::Json(CombatSummary {
            combatant_1_summary: Some((&combatant_1_state_and_context).into()),
            combatant_2_summary: Some((&combatant_2_state_and_context).into()),
            attacks,
            outcome,
        }),
    )
}
