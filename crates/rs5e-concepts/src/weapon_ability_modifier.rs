use crate::{
    ability_modifier::AbilityModifier,
    ability_scores::{AbilityScore, AbilityScores},
    ability_type::AbilityType,
    weapon_range::WeaponRange,
};
#[cfg(feature = "serde")]
#[derive(Debug)]
pub struct WeaponAbilityModifierContext<'a> {
    pub ability_scores: &'a AbilityScores,
    pub weapon_range: Option<&'a WeaponRange>,
}

#[derive(Debug)]
pub struct AbilityModifierContext {
    pub ability_type: AbilityType,
    pub ability_score: AbilityScore,
    pub ability_modifier: AbilityModifier,
}

impl From<WeaponAbilityModifierContext<'_>> for AbilityModifierContext {
    fn from(context: WeaponAbilityModifierContext) -> Self {
        // Unarmed counts as melee (strength)
        let ability_type = context
            .weapon_range
            .map_or(AbilityType::Strength, AbilityType::from);

        let ability_score = context.ability_scores.score(&ability_type);

        Self {
            ability_type,
            ability_modifier: ability_score.modifier(),
            ability_score: ability_score.clone(),
        }
    }
}
