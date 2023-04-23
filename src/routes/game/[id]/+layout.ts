import { invoke } from '@tauri-apps/api/tauri'
import type { LayoutLoad } from '../../$types';


export const load = (() => {
    let teams = invoke('generate_teams');


    return {
        teams
    };
}) satisfies LayoutLoad;