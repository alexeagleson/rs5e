import { useEffect, useState } from "react";
import "./App.css";
import {
  ArmorModel,
  Constants,
  CharacterBuilder,
  WeaponModel,
  CombatSummary,
} from "./utility/types";
import { MakeCharacter } from "./components/MakeCharacter";
import { CombatSummaryComponent } from "./components/CombatSummary";
import { getArmor, getConstants, getWeapons, postAttack } from "./api";
import Ibraham from "./assets/ibraham.png";

export const AppWithData = () => {
  const [weapons, setWeapons] = useState<WeaponModel[]>();
  const [armor, setArmor] = useState<ArmorModel[]>();
  const [constants, setConstants] = useState<Constants>();

  useEffect(() => {
    getWeapons().then((w) => {
      setWeapons(w);
    });
    getArmor().then((a) => {
      setArmor(a);
    });
    getConstants().then((c) => {
      setConstants(c);
    });
  }, []);

  if (!weapons || !armor || !constants) {
    return <p>Loading...</p>;
  }

  return <App weapons={weapons} armor={armor} constants={constants} />;
};

interface IApp {
  constants: Constants;
  weapons: WeaponModel[];
  armor: ArmorModel[];
}

const defaultUnit = (
  constants: Constants,
  defaultName: string
): CharacterBuilder => {
  return {
    abilityScores: {
      str: constants.DEFAULT_ABILITY_SCORE,
      dex: constants.DEFAULT_ABILITY_SCORE,
      con: constants.DEFAULT_ABILITY_SCORE,
      int: constants.DEFAULT_ABILITY_SCORE,
      wis: constants.DEFAULT_ABILITY_SCORE,
      cha: constants.DEFAULT_ABILITY_SCORE,
    },
    class: constants.DEFAULT_CLASS_TYPE,
    name: defaultName,
    hp: 10,
    level: constants.DEFAULT_LEVEL,
    proneState: constants.DEFAULT_PRONE_STATE,
    coverState: constants.DEFAULT_COVER_STATE,
    armorType: undefined,
    weaponType: undefined,
  };
};

const App: React.FC<IApp> = ({ constants, weapons, armor }) => {
  const [log, setLog] = useState<CombatSummary>();

  const [attacker, setAttacker] = useState<CharacterBuilder>(
    defaultUnit(constants, "Combatant 1")
  );
  const [target, setTarget] = useState<CharacterBuilder>(
    defaultUnit(constants, "Combatant 2")
  );

  const [currentUnit, setCurrentUnit] = useState<"attacker" | "defender">(
    "attacker"
  );

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        paddingBottom: "240px",
      }}
    >
      <div
        style={{
          alignSelf: "center",
          display: "flex",
          flexDirection: "row",
          columnGap: "12px",
          alignItems: "center",
        }}
      >
        <img height={96} width={96} src={Ibraham} alt="Ibraham" />
        <h1>rs5e</h1>
      </div>

      <div
        className="button-group-container"
        style={{
          alignSelf: "center",
          margin: "24px 0",
        }}
      >
        <button
          className={currentUnit === "attacker" ? "button-selected" : ""}
          onClick={() => setCurrentUnit("attacker")}
        >
          Combatant 1
        </button>
        <button
          className={currentUnit === "defender" ? "button-selected" : ""}
          onClick={() => setCurrentUnit("defender")}
        >
          Combatant 2
        </button>
      </div>
      {currentUnit === "attacker" ? (
        <MakeCharacter
          unit={attacker}
          setUnit={setAttacker}
          constants={constants}
          weapons={weapons}
          armor={armor}
          characterType={currentUnit}
        />
      ) : (
        <MakeCharacter
          unit={target}
          setUnit={setTarget}
          constants={constants}
          weapons={weapons}
          armor={armor}
          characterType={currentUnit}
        />
      )}

      <hr />
      <label>
        <button
          style={{ maxWidth: "fit-content" }}
          onClick={() => {
            postAttack({
              attacker,
              target,
            }).then((p) => {
              setLog(p);
            });
          }}
        >
          Simulate Combat
        </button>
      </label>

      {log && (
        <CombatSummaryComponent
          {...log}
          combatant1Name={attacker.name}
          combatant2Name={target.name}
        />
      )}

      <hr />
    </div>
  );
};

export default AppWithData;
