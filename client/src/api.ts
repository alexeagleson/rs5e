import {
  ArmorModel,
  AttackRequest,
  CombatSummary,
  Constants,
  WeaponModel,
} from "./utility/types";

export const postAttack = async (
  attackRequest: AttackRequest
): Promise<CombatSummary> => {
  const log = fetch("/attack", {
    method: "POST", // *GET, POST, PUT, DELETE, etc.
    //   mode: "cors", // no-cors, *cors, same-origin
    cache: "no-cache",
    body: JSON.stringify(attackRequest),
    headers: {
      "Content-Type": "application/json",
    },
  }).then((response) =>
    response.json().then((r: CombatSummary) => {
      console.log(r);
      return r;
    })
  );

  return log;
};

export const getWeapons = async (): Promise<WeaponModel[]> => {
  return fetch("/get-weapons", {
    method: "GET",
    mode: "cors",
    cache: "no-cache",
    headers: {
      Accept: "application/json",
    },
  }).then((response) =>
    response.json().then((weaponsResponse): WeaponModel[] => {
      const weapons: WeaponModel[] = JSON.parse(weaponsResponse);
      return weapons;
    })
  );
};

export const getArmor = async (): Promise<ArmorModel[]> => {
  return fetch("/get-armor", {
    method: "GET",
    mode: "cors",
    cache: "no-cache",
    headers: {
      Accept: "application/json",
    },
  }).then((response) =>
    response.json().then((armorResponse): ArmorModel[] => {
      const armor: ArmorModel[] = JSON.parse(armorResponse);
      return armor;
    })
  );
};

export const getConstants = async (): Promise<Constants> => {
  return fetch("/get-constants", {
    method: "GET",
    mode: "cors",
    cache: "no-cache",
    headers: {
      Accept: "application/json",
    },
  }).then((response) =>
    response.json().then((constantsResponse): Constants => {
      const constants: Constants = JSON.parse(constantsResponse);
      return constants;
    })
  );
};
