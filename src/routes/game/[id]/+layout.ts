import { invoke } from "@tauri-apps/api/tauri";
import type { LayoutLoad } from "./$types";
import type { Team } from "$lib/types_rust";


export const load = (() => {
    let data:Array<Team> = invoke('generate_teams');
    console.log("generated Data");

    return { data };
}) satisfies LayoutLoad;

