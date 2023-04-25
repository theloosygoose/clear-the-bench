<script lang="ts">
    import type { IntangibleRatings, TangibleRatings } from "$lib/types_rust";
import { Progressbar, ScoreRating } from "flowbite-svelte";

    export let intangible_ratings: IntangibleRatings;

    let burst = intangible_ratings.burst;
    let speed = intangible_ratings.speed;


</script>

<div class="my-24">
    <h1 class="dark:text-white text-xl">Athleticism:</h1>
    <hr>

    <div class="my-4">
        <Progressbar 
            labelOutside="Burst" 
            progress="{burst.toString()}" 
            color="green"
        />
    </div>

    <div class="my-4">
        <Progressbar 
            labelOutside="Speed" 
            progress="{speed.toString()}" 
            color="green"
        />
    </div>

    <div class="my-4">
        <Progressbar 
            labelOutside="Fluidity" 
            progress="{intangible_ratings.fluidity.toString()}" 
            color="green"
        />
    </div>

    <div class="my-4">
        <Progressbar 
            labelOutside="Strength" 
            progress="{intangible_ratings.strength.toString()}" 
            color="green"
        />
    </div>

    <div class="my-4">
        <Progressbar 
            labelOutside="Lateral Speed" 
            progress="{intangible_ratings.sliding.toString()}" 
            color="green"
        />
    </div>

</div>
