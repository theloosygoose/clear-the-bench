/* eslint-disable prefer-const */
import { invoke } from "@tauri-apps/api/tauri";
import type { LayoutLoad } from "./$types";
import type { PlayerPerson, Team } from "$lib/types_rust";


export const load = (async () => {

    let teams: Array<Team> = await invoke('generate_teams');

    let playersData: PlayerPerson[] = [];

    let players = teams.forEach((team:Team) => {
        team.players.forEach((player:PlayerPerson) => playersData.push(player));
    })

    return { teams, playersData };
}) satisfies LayoutLoad;

