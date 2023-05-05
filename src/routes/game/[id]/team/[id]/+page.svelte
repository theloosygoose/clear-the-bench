<script lang="ts">
    import { page } from '$app/stores';
    import PlayersTable from '$lib/components/GameComp/TeamView/PlayersTable.svelte';

    import type { Team } from '$lib/types_rust';
    import type { PageData } from './$types';

    export let data: PageData;

    let index = data.teams.filter( (e:any) => {
        return e.name == $page.params.id;
    });

    let team:Team = index[0];
</script>


<main class="mt-5">
<h1 class="text-white text-4xl font-bold">{team.name}</h1>
<h2 class="dark:text-white text-2xl p-2">Coach: {team.coach}</h2>
<h2 class="dark:text-white text-xl p-2">Owner: {team.owner}</h2>
<h2 class="dark:text-white text-xl p-2">Payroll: {team.team_salary}</h2>
<h2 class="dark:text-white p-2">Wins: {team.wins} Losses: {team.losses}</h2>
<PlayersTable players = {team.players}/>

</main>
