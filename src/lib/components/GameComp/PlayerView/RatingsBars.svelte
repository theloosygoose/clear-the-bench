<script lang="ts">
    import type { IntangibleRatings, TangibleRatings } from "$lib/types_rust";
import { ScoreRating } from "flowbite-svelte";

    export let intangible_ratings: IntangibleRatings;
    export let tangible_ratings: TangibleRatings;


    let burst = intangible_ratings.burst;
    let speed = intangible_ratings.speed;

    let headerLabel = {
        desc1: '8.7',
        desc2: 'Mashallah',
        desc3: '0 reviews',
        link: {
            label: 'Read all reviews',
            url: '/'
        }
    }



</script>

<ScoreRating
    { headerLabel }
    ratings={[
        { label: 'Burst', rating: burst},
        { label: 'Speed', rating: speed},

    ]}

/>

