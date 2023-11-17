use crate::snapshots::{attacker::AttackerSnapshot, defender::DefenderSnapshot};
use rs5e_components::{
    attacker::Attacker, combatant::Combatant, damages::Damages, destructible::Destructible,
    has_armor::HasArmor,
};
use rs5e_concepts::{
    advantage_type::AdvantageType,
    armor_class::ArmorClass,
    armor_proficiency_penalty::{ArmorProficiencyContext, ArmorProficiencyPenalty},
    attack_roll::AttackRoll,
    cover_state::CoverBonus,
    cover_state::{CanBeTargetedCoverBonus, CannotBeTargeted},
    hit::Hit,
    prone_state::ProneContext,
    roll::Roll,
    roll_type::RollType,
    weapon_ability_modifier::{AbilityModifierContext, WeaponAbilityModifierContext},
    weapon_proficiency_bonus::{WeaponProficiencyBonus, WeaponProficiencyContext},
};
use std::ops::Deref;

#[derive(Debug)]
pub struct PhysicalAttackOutcome<'a> {
    pub hit: Hit,
    pub attack_roll_type: RollType<AttackRoll>,
    pub state_and_context: &'a PhysicalAttackStateAndContext<'a>,
}

#[derive(Debug)]
pub struct PhysicalAttackStateAndContext<'a> {
    pub state: PhysicalAttackState<'a>,
    pub context: PhysicalAttackContext,
}

#[derive(Debug)]
pub struct PhysicalAttackState<'a> {
    pub attacker: AttackerSnapshot<'a>,
    pub defender: DefenderSnapshot<'a>,
}

#[derive(Debug)]
pub struct AttackerPhysicalAttackContext {
    pub prone_context_bonus: AdvantageType,
    pub weapon_proficiency_bonus: WeaponProficiencyBonus,
    pub armor_proficiency_penalty: ArmorProficiencyPenalty,
    pub ability_modifier_context: AbilityModifierContext,
    pub computed_advantage_type: AdvantageType,
}

#[derive(Debug)]
pub struct DefenderPhysicalAttackContext {
    pub cover_state_bonus: CanBeTargetedCoverBonus,
    pub cover_state_armor_class_bonus: ArmorClass,
    pub base_armor_class: ArmorClass,
    pub computed_armor_class: ArmorClass,
}

#[derive(Debug)]
pub struct PhysicalAttackContext {
    pub prone: ProneContext,
    pub attacker: AttackerPhysicalAttackContext,
    pub defender: DefenderPhysicalAttackContext,
}

impl TryFrom<&PhysicalAttackState<'_>> for PhysicalAttackContext {
    type Error = CannotBeTargeted;

    fn try_from(state: &PhysicalAttackState<'_>) -> Result<Self, Self::Error> {
        let prone_context =
            ProneContext::from((&state.attacker.prone_state, &state.defender.prone_state));
        let attacker_prone_context_bonus = AdvantageType::from(&prone_context);
        let defender_bonus_from_cover_state_maybe_cant_be_targeted =
            CoverBonus::from(&state.defender.cover_state);
        let defender_bonus_from_cover_state = CanBeTargetedCoverBonus::try_from(
            &defender_bonus_from_cover_state_maybe_cant_be_targeted,
        )?;

        let attacker_weapon_proficiency_bonus =
            WeaponProficiencyBonus::from(WeaponProficiencyContext {
                class: &state.attacker.class,
                level: &state.attacker.level,
                weapon_category: state.attacker.weapon.as_ref().map(|w| &w.weapon_category),
                weapon_type: state.attacker.weapon.as_ref().map(|w| &w.weapon_type),
            });

        let attacker_armor_proficiency_penalty =
            ArmorProficiencyPenalty::from(ArmorProficiencyContext {
                class: &state.attacker.class,
                armor_category: state.attacker.armor.as_ref().map(|a| &a.armor_category),
                armor_primary_material: state.attacker.armor.as_ref().map(|a| &a.primary_material),
            });

        let attacker_weapon_ability_modifier_context =
            AbilityModifierContext::from(WeaponAbilityModifierContext {
                weapon_range: state.attacker.weapon.as_ref().map(|w| &w.weapon_range),
                ability_scores: &state.attacker.ability_scores,
            });

        let attacker_computed_advantage_type = AdvantageType::from_all_sources(&[
            &attacker_prone_context_bonus,
            &AdvantageType::from(&attacker_armor_proficiency_penalty),
        ]);

        let defender_base_armor_class = state.defender.equipped_armor.map_or_else(
            || state.defender.armor_class.clone(),
            |armor| armor.armor_class.clone(),
        );

        let defender_cover_state_armor_class_bonus = defender_bonus_from_cover_state
            .armor_class_bonus()
            .cloned()
            .unwrap_or(ArmorClass::new(0));

        let defender_total_armor_class = ArmorClass::new(
            defender_base_armor_class.value() + defender_cover_state_armor_class_bonus.value(),
        );

        Ok(Self {
            prone: prone_context,
            attacker: AttackerPhysicalAttackContext {
                prone_context_bonus: attacker_prone_context_bonus,
                weapon_proficiency_bonus: attacker_weapon_proficiency_bonus,
                armor_proficiency_penalty: attacker_armor_proficiency_penalty,
                ability_modifier_context: attacker_weapon_ability_modifier_context,
                computed_advantage_type: attacker_computed_advantage_type,
            },
            defender: DefenderPhysicalAttackContext {
                base_armor_class: defender_base_armor_class,
                cover_state_bonus: defender_bonus_from_cover_state,
                cover_state_armor_class_bonus: defender_cover_state_armor_class_bonus,
                computed_armor_class: defender_total_armor_class,
            },
        })
    }
}

pub fn physical_attack_system<'a, A, D>(
    attacker: &A,
    defender: &mut D,
    state_and_context: &'a PhysicalAttackStateAndContext<'a>,
) -> PhysicalAttackOutcome<'a>
where
    A: Attacker,
    D: Destructible + Combatant + HasArmor,
{
    let PhysicalAttackStateAndContext { context, .. } = &state_and_context;

    let attack_roller = || {
        attacker.roll_attack(
            context
                .attacker
                .ability_modifier_context
                .ability_modifier
                .clone(),
            context.attacker.weapon_proficiency_bonus.deref().clone(),
        )
    };

    let attack_roll_type: RollType<AttackRoll> = (
        context.attacker.computed_advantage_type.clone(),
        attack_roller,
    )
        .into();

    let chosen_roll = attack_roll_type.chosen_roll();

    PhysicalAttackOutcome {
        hit: if chosen_roll.total_value() >= context.defender.computed_armor_class.value() {
            let (damage_roll, damage_type) = attacker.equipped_weapon().map_or_else(
                || {
                    (
                        attacker.roll_damage(Some(chosen_roll)),
                        attacker.damage_type(),
                    )
                },
                |weapon| (weapon.roll_damage(Some(chosen_roll)), weapon.damage_type()),
            );

            let hp_change = defender.take_damage(damage_roll.total_value());
            Hit::Success {
                damage_roll,
                hp_change,
                damage_type,
            }
        } else {
            Hit::Miss
        },
        attack_roll_type,
        state_and_context,
    }
}
