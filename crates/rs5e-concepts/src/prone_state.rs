use crate::advantage_type::{Advantage, AdvantageType, Disadvantage};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

pub const DEFAULT_PRONE_STATE: ProneState = ProneState::Upright;

#[typeshare]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ProneState {
    /// No impact on attacker or defender advantage state
    Upright,
    /// Prone attackers have disadvantage
    ///
    /// Attackers against prone targets have **advantage**
    /// if the target is within 5 feet
    ///
    /// Attackers against prone targets have **disadvantage**
    /// if the target is greater than 5 feet away
    Prone,
}

impl Default for ProneState {
    fn default() -> Self {
        DEFAULT_PRONE_STATE
    }
}

// Attacker vs. Defender
impl From<(&ProneState, &ProneState)> for ProneContext {
    fn from(attacker_vs_target: (&ProneState, &ProneState)) -> Self {
        match attacker_vs_target {
            (ProneState::Upright, ProneState::Upright) => Self::UprightVsUpright,
            (ProneState::Upright, ProneState::Prone) => Self::UprightVsProne,
            (ProneState::Prone, ProneState::Upright) => Self::ProneVsUpright,
            (ProneState::Prone, ProneState::Prone) => Self::ProneVsProne,
        }
    }
}

#[derive(Debug)]
pub enum ProneContext {
    UprightVsUpright,
    UprightVsProne,
    ProneVsProne,
    ProneVsUpright,
}

impl From<&ProneContext> for AdvantageType {
    fn from(context: &ProneContext) -> Self {
        match context {
            ProneContext::UprightVsUpright | ProneContext::ProneVsProne => Self::Normal,
            ProneContext::UprightVsProne => Self::Advantage(Advantage),
            ProneContext::ProneVsUpright => Self::Disadvantage(Disadvantage),
        }
    }
}
