<script lang="ts">
    import { page } from '$app/stores';
    import RatingsBars from '$lib/components/GameComp/PlayerView/RatingsBars.svelte';

    import type { PlayerPerson } from '$lib/types_rust';
    import type { PageData } from './$types';

    export let data: PageData;

    let player:PlayerPerson = data.playersData.filter((e:PlayerPerson) => {
        return e.id == $page.params.id;
    })[0];

    let intangible_ratings = player.job.Player.ratings;
    let tangible_ratings = player.job.Player.skills;


</script>

<h1 class="text-2xl">{player.name}</h1>

<h2>Place of Birth: {player.country}</h2>


<RatingsBars {intangible_ratings} {tangible_ratings}/>

