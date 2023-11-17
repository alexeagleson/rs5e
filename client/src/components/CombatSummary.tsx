import { CombatSummary } from "../utility/types";

export interface ICombatSummaryComponent extends CombatSummary {
  combatant1Name: string;
  combatant2Name: string;
}

export const CombatSummaryComponent: React.FC<ICombatSummaryComponent> = (
  summary
) => {
  const {
    combatant1Summary,
    combatant2Summary,
    attacks,
    outcome,
    combatant1Name,
    combatant2Name,
  } = summary;

  return (
    <div>
      {combatant1Summary && (
        <div>
          <h4>{combatant1Name} Summary:</h4>
          <ul>
            <li>{combatant1Summary.attackerWeaponAbilityModifier}</li>
            <li>{combatant1Summary.attackerWeaponProficiencyBonus}</li>
            <li>{combatant1Summary.attackerArmorProficiencyPenalty}</li>
            <li>{combatant1Summary.attackerProneContextBonus}</li>
            <li>{combatant1Summary.defenderBonusFomCoverState}</li>
            <li>{combatant1Summary.attackerCombinedAdvantageType}</li>
          </ul>
        </div>
      )}

      {combatant2Summary && (
        <div>
          <h4>{combatant2Name} Summary:</h4>
          <ul>
            <li>{combatant2Summary.attackerWeaponAbilityModifier}</li>
            <li>{combatant2Summary.attackerWeaponProficiencyBonus}</li>
            <li>{combatant2Summary.attackerArmorProficiencyPenalty}</li>
            <li>{combatant2Summary.attackerProneContextBonus}</li>
            <li>{combatant2Summary.defenderBonusFomCoverState}</li>
            <li>{combatant2Summary.attackerCombinedAdvantageType}</li>
          </ul>
        </div>
      )}

      <hr />

      {attacks.map((a, idx) => {
        return (
          <div>
            <h3>{a.beforePhrase}</h3>
            <>
              {a.summary.t == "failure" ? (
                <p key={idx}>{a.summary.c}</p>
              ) : (
                <ul>
                  <li>{a.summary.c.after.attack}</li>
                  <li>{a.summary.c.after.attackRoll}</li>
                  <li>{a.summary.c.after.targetAc}</li>
                  {a.summary.c.after.damageRoll && (
                    <li>{a.summary.c.after.damageRoll}</li>
                  )}
                  <li>{a.summary.c.after.hit}</li>
                  {a.summary.c.after.hpChange && (
                    <li>{a.summary.c.after.hpChange}</li>
                  )}
                </ul>
              )}
            </>
            <h5>{a.afterPhrase}</h5>
          </div>
        );
      })}

      <h1>{outcome}</h1>
    </div>
  );
};
