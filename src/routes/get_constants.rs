use axum::{http::StatusCode, Json};
use rs5e_concepts::{
    ability_modifier::ABILITY_MODIFIER_TABLE,
    ability_scores::{DEFAULT_ABILITY_SCORE, MAX_ABILITY_SCORE, MIN_ABILITY_SCORE},
    class_type::{ClassType, DEFAULT_CLASS_TYPE},
    cover_state::CoverState,
    hit_dice::{hit_die_by_class_map, HitDieByClassMap},
    level::{DEFAULT_LEVEL, MAX_LEVEL, MIN_LEVEL},
    proficiency_type::{proficiency_type_by_class_map, ProficiencyTypeByClassMap},
    prone_state::ProneState,
    weapon_proficiency_bonus::WEAPON_PROFICIENCY_BONUS_TABLE,
};
use serde::Serialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
struct Constants {
    MIN_LEVEL: u32,
    MAX_LEVEL: u32,
    MIN_ABILITY_SCORE: u32,
    MAX_ABILITY_SCORE: u32,
    DEFAULT_ABILITY_SCORE: u32,
    DEFAULT_LEVEL: u32,
    DEFAULT_CLASS_TYPE: ClassType,
    DEFAULT_PRONE_STATE: ProneState,
    DEFAULT_COVER_STATE: CoverState,
    WEAPON_PROFICIENCY_BONUS_TABLE: [u32; 20],
    ABILITY_MODIFIER_TABLE: [i32; 30],
    PROFICIENCY_TYPE_BY_CLASS_MAP: &'static ProficiencyTypeByClassMap,
    HIT_DIE_BY_CLASS_MAP: &'static HitDieByClassMap,
}

pub(crate) async fn get_constants() -> (StatusCode, Json<String>) {
    let constants = Constants {
        MIN_LEVEL,
        MAX_LEVEL,
        MIN_ABILITY_SCORE,
        MAX_ABILITY_SCORE,
        DEFAULT_ABILITY_SCORE,
        DEFAULT_LEVEL,
        DEFAULT_CLASS_TYPE,
        DEFAULT_PRONE_STATE: ProneState::default(),
        DEFAULT_COVER_STATE: CoverState::default(),
        WEAPON_PROFICIENCY_BONUS_TABLE,
        ABILITY_MODIFIER_TABLE,
        PROFICIENCY_TYPE_BY_CLASS_MAP: proficiency_type_by_class_map(),
        HIT_DIE_BY_CLASS_MAP: hit_die_by_class_map(),
    };

    let serialized_constants = serde_json::to_string(&constants).unwrap();

    (StatusCode::OK, axum::Json(serialized_constants))
}
