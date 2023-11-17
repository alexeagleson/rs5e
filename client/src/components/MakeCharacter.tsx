import { AbilityScoresComponent } from "./AbilityScores";
import { Counter } from "./Counter";
import { camelToSentenceCase, rollHp } from "../util";
import {
  ArmorModel,
  ArmorType,
  ClassType,
  Constants,
  CoverState,
  ProneState,
  CharacterBuilder,
  WeaponModel,
  WeaponType,
} from "../utility/types";

export interface IMakeCharacter {
  unit: CharacterBuilder;
  setUnit: React.Dispatch<React.SetStateAction<CharacterBuilder>>;
  constants: Constants;
  weapons: WeaponModel[];
  armor: ArmorModel[];
  characterType: "attacker" | "defender";
}

export const MakeCharacter: React.FC<IMakeCharacter> = ({
  unit,
  setUnit,
  constants,
  weapons,
  armor,
  characterType,
}) => {
  return (
    <div key={characterType} className="make-unit">
      <div className="make-unit-column-1 flex-column">
        <label>
          Name:{" "}
          <input
            type="text"
            onChange={(e) => {
              setUnit(() => ({ ...unit, name: e.target.value }));
            }}
            value={unit.name}
          ></input>
        </label>
        <div className="button-group-container">
          <Counter
            value={`${unit.level}`}
            label="Level"
            increment={() => {
              setUnit(
                (u): CharacterBuilder => ({
                  ...u,
                  level: Math.min(u.level + 1, constants.MAX_LEVEL),
                })
              );
            }}
            decrement={() => {
              setUnit(
                (u): CharacterBuilder => ({
                  ...u,
                  level: Math.max(u.level - 1, constants.MIN_LEVEL),
                })
              );
            }}
            isLevel
          />
          <p>
            Proficiency bonus:&nbsp;
            {constants.WEAPON_PROFICIENCY_BONUS_TABLE[unit.level - 1]}
          </p>
        </div>

        <label className="button-group-container">
          Hit Points:&nbsp;
          <input
            style={{ maxWidth: "72px" }}
            type="number"
            value={unit.hp}
            onChange={(e) =>
              setUnit(
                (u): CharacterBuilder => ({
                  ...u,
                  hp: parseInt(e.target.value),
                })
              )
            }
          />
        </label>
        <label
          className="button-group-container"
          style={{
            width: "240px",
            display: "flex",
            justifyContent: "space-between",
          }}
        >
          Hit Die:&nbsp;
          {unit.level}
          {constants.HIT_DIE_BY_CLASS_MAP[unit.class]}
          <button
            onClick={() => {
              setUnit(
                (u): CharacterBuilder => ({
                  ...u,
                  hp: rollHp(
                    {
                      die: constants.HIT_DIE_BY_CLASS_MAP[u.class],
                      quantity: u.level,
                    },
                    "random"
                  ),
                })
              );
            }}
          >
            Roll
          </button>
          <button
            onClick={() => {
              setUnit(
                (u): CharacterBuilder => ({
                  ...u,
                  hp: rollHp(
                    {
                      die: constants.HIT_DIE_BY_CLASS_MAP[u.class],
                      quantity: u.level,
                    },
                    "max"
                  ),
                })
              );
            }}
          >
            Max
          </button>
        </label>

        <div>
          <label className="button-group-container">
            Class:&nbsp;
            <select
              value={unit.class}
              onChange={(e) => {
                setUnit(
                  (u): CharacterBuilder => ({
                    ...u,
                    class: ClassType[e.target.value as keyof typeof ClassType],
                  })
                );
              }}
            >
              {Object.entries(ClassType).map(([k, v], idx) => {
                return (
                  <option key={idx} value={v}>
                    {k}
                  </option>
                );
              })}
            </select>
          </label>
        </div>

        <div>
          <label className="button-group-container">
            Prone:
            {Object.values(ProneState).map((v) => {
              return (
                <button
                  className={unit.proneState === v ? "button-selected" : ""}
                  onClick={() => {
                    switch (v) {
                      case ProneState.Upright:
                        setUnit(
                          (u): CharacterBuilder => ({
                            ...u,
                            proneState: ProneState.Upright,
                          })
                        );
                        return;
                      case ProneState.Prone:
                        setUnit(
                          (u): CharacterBuilder => ({
                            ...u,
                            proneState: ProneState.Prone,
                          })
                        );
                        return;
                    }
                  }}
                >
                  {v}
                </button>
              );
            })}
          </label>
        </div>

        <div className="button-group-container">
          <label>Cover: </label>
          {Object.values(CoverState).map((v) => {
            return (
              <button
                className={unit.coverState === v ? "button-selected" : ""}
                onClick={() => {
                  switch (v) {
                    case CoverState.None:
                      setUnit(
                        (u): CharacterBuilder => ({
                          ...u,
                          coverState: CoverState.None,
                        })
                      );
                      return;
                    case CoverState.Half:
                      setUnit(
                        (u): CharacterBuilder => ({
                          ...u,
                          coverState: CoverState.Half,
                        })
                      );
                      return;
                    case CoverState.ThreeQuarters:
                      setUnit(
                        (u): CharacterBuilder => ({
                          ...u,
                          coverState: CoverState.ThreeQuarters,
                        })
                      );
                      return;
                    case CoverState.Total:
                      setUnit(
                        (u): CharacterBuilder => ({
                          ...u,
                          coverState: CoverState.Total,
                        })
                      );
                      return;
                  }
                }}
              >
                {v}
              </button>
            );
          })}
        </div>

        <div>
          <label htmlFor="dnd-weapon">Weapon</label>
          <select
            name="dnd-weapon"
            value={unit.weaponType}
            onChange={(e) => {
              setUnit(
                (u): CharacterBuilder => ({
                  ...u,
                  weaponType: (e.target.value as WeaponType) || undefined,
                })
              );
            }}
          >
            <option value="">None</option>
            {weapons.map((wp, idx) => {
              return (
                <option key={idx} value={wp.weapon_type}>
                  {`${camelToSentenceCase(wp.weapon_type)} (${
                    wp.weapon_category
                  }, ${wp.damage_dice.quantity}${wp.damage_dice.die.die_type} ${
                    wp.damage_type
                  })`}
                </option>
              );
            })}
          </select>
        </div>

        <div>
          <label htmlFor="dnd-armor">Armor</label>
          <select
            name="dnd-armor"
            value={unit.armorType}
            onChange={(e) => {
              setUnit(
                (u): CharacterBuilder => ({
                  ...u,
                  armorType: (e.target.value as ArmorType) || undefined,
                })
              );
            }}
          >
            <option value="">None</option>
            {armor.map((ar, idx) => {
              return (
                <option key={idx} value={ar.armor_type}>
                  {`${camelToSentenceCase(ar.armor_type)} (${
                    ar.armor_category
                  }) (AC ${ar.armor_class})`}
                </option>
              );
            })}
          </select>
        </div>
      </div>

      <div className="make-unit-column-2 flex-column">
        <AbilityScoresComponent
          constants={constants}
          scores={unit.abilityScores}
          increment={(key) => {
            setUnit((u): CharacterBuilder => {
              return {
                ...u,
                abilityScores: {
                  ...u.abilityScores,
                  [key]: Math.min(
                    u.abilityScores[key] + 1,
                    constants.MAX_ABILITY_SCORE
                  ),
                },
              };
            });
          }}
          decrement={(key) => {
            setUnit((u): CharacterBuilder => {
              return {
                ...u,
                abilityScores: {
                  ...u.abilityScores,
                  [key]: Math.max(
                    u.abilityScores[key] - 1,
                    constants.MIN_ABILITY_SCORE
                  ),
                },
              };
            });
          }}
        />
      </div>
    </div>
  );
};
