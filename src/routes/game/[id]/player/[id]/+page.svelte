<script lang="ts">
    import { page } from '$app/stores';
    import AthleticismBars from '$lib/components/GameComp/PlayerView/AthleticismBars.svelte';
    import DefenseBars from '$lib/components/GameComp/PlayerView/DefenseBars.svelte';
    import SkillsBars from '$lib/components/GameComp/PlayerView/SkillsBars.svelte';

    import type { PlayerPerson } from '$lib/types_rust';
    import type { PageData } from './$types';

    export let data: PageData;

    let player:PlayerPerson = data.playersData.filter((e:PlayerPerson) => {
        return e.id == $page.params.id;
    })[0];

    let intangible_ratings = player.job.Player.ratings;
    let tangible_ratings = player.job.Player.skills;


</script>

<h1 class="dark:text-white text-2xl">{player.name}</h1>
<h2 class="dark:text-white">Place of Birth: {player.country}</h2>
<h3 class="dark:text-white">Height: {player.job.Player.ratings.height}</h3>
<h3 class="dark:text-white">Age: {player.age}</h3>


<div class="w-[80%] grid grid-cols-3 gap-4">
    <AthleticismBars {intangible_ratings}/>
    <SkillsBars {tangible_ratings} />
    <DefenseBars {tangible_ratings} />
</div>

