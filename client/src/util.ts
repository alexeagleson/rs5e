import { HitDice, HitDie } from "./utility/types";

export const camelToSentenceCase = (str: string): string => {
  return str
    .split(/([A-Z]|\d)/)
    .map((v, i, arr) => {
      // If first block then capitalise 1st letter regardless
      if (!i) return v.charAt(0).toUpperCase() + v.slice(1);
      // Skip empty blocks
      if (!v) return v;
      // Underscore substitution
      if (v === "_") return " ";
      // We have a capital or number
      if (v.length === 1 && v === v.toUpperCase()) {
        const previousCapital = !arr[i - 1] || arr[i - 1] === "_";
        const nextWord = i + 1 < arr.length && arr[i + 1] && arr[i + 1] !== "_";
        const nextTwoCapitalsOrEndOfString =
          i + 3 > arr.length || (!arr[i + 1] && !arr[i + 3]);
        // Insert space
        if (!previousCapital || nextWord) v = " " + v;
        // Start of word or single letter word
        if (nextWord || (!previousCapital && !nextTwoCapitalsOrEndOfString))
          v = v.toLowerCase();
      }
      return v;
    })
    .join("");
};

export const neverEver = (shouldBeNever: never) => {
  throw new Error("Was not never: " + shouldBeNever);
};

export const rollHp = (
  hitDice: HitDice,
  rollType: "random" | "max"
): number => {
  let dieSize: number = -1;
  switch (hitDice.die) {
    case HitDie.D6:
      dieSize = 6;
      break;
    case HitDie.D8:
      dieSize = 8;
      break;
    case HitDie.D10:
      dieSize = 10;
      break;
    case HitDie.D12:
      dieSize = 12;
      break;
    default:
      neverEver(hitDice.die);
  }

  if (dieSize === -1) {
    throw Error("Unrecognized hit dice");
  }

  let total = 0;
  switch (rollType) {
    case "random":
      for (let i = 0; i < hitDice.quantity; i++) {
        total += Math.floor(Math.random() * dieSize) + 1;
      }
      return total;
    case "max":
      return dieSize * hitDice.quantity;
    default:
      neverEver(rollType);
  }

  throw Error("unreachable in hitDice");
};
