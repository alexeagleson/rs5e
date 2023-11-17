use rs5e_concepts::{cover_state::CoverState, prone_state::ProneState};

pub trait Combatant {
    fn prone_state(&self) -> &ProneState;

    fn cover_state(&self) -> &CoverState;
}
