import { invoke } from "@tauri-apps/api/tauri";
import type { LayoutData } from "../../../$types";


export const load = (() => {
    let data = invoke('generate_teams');

    return { data };

}) satisfies LayoutData;

