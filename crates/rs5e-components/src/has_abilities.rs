use rs5e_concepts::{
    ability_scores::{AbilityScore, AbilityScores},
    ability_type::AbilityType,
};

pub trait HasAbilities {
    fn ability_scores(&self) -> &AbilityScores;

    fn ability_scores_mut(&mut self) -> &mut AbilityScores;

    fn ability_score(&self, ability: &AbilityType) -> &AbilityScore {
        self.ability_scores().score(ability)
    }

    fn ability_score_mut(&mut self, ability: &AbilityType) -> &mut AbilityScore {
        self.ability_scores_mut().score_mut(ability)
    }
}
