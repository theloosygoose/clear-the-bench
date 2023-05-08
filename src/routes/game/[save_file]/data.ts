import type { GameSaveData } from "$lib/types_rust";
import { writable, type Writable } from "svelte/store";


let test:GameSaveData = {
  id: 0,
  user_team: "",
  save_file: "",
  save_name: "",
  year: 0, 
  teams: [],
  people: [],
}

export let currentgamedata:Writable<GameSaveData> = writable(test);

