import { Counter } from "./Counter";
import { AbilityScores, Constants } from "../utility/types";

export interface IAbilityScores {
  scores: AbilityScores;
  increment: (key: keyof AbilityScores) => void;
  decrement: (key: keyof AbilityScores) => void;
  constants: Constants;
}

export const AbilityScoresComponent: React.FC<IAbilityScores> = ({
  scores,
  increment,
  decrement,
  constants,
}) => {
  return (
    <div>
      {Object.entries(scores).map(([key, value]) => {
        const typedKey = key as keyof typeof scores;
        const typedValue = value as (typeof scores)[typeof typedKey];

        return (
          <Counter
            value={`${value} (${
              constants.ABILITY_MODIFIER_TABLE[typedValue - 1]
            })`}
            label={key.toUpperCase()}
            increment={() => increment(typedKey)}
            decrement={() => decrement(typedKey)}
            isLevel={false}
          />
        );
      })}
    </div>
  );
};
