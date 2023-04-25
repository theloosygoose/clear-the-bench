
<script lang="ts">
    import type { TangibleRatings } from "$lib/types_rust";
    import { Progressbar, } from "flowbite-svelte";

    export let tangible_ratings: TangibleRatings;


</script>

<div class="my-24">
    <h1 class="dark:text-white text-xl">Skills:</h1>
    <hr>

    <div class="my-4">
        <Progressbar 
            labelOutside="On Ball Defense" 
            progress="{tangible_ratings.defense_on_ball.toString()}" 
            color="green"
        />
    </div>

    <div class="my-4">
        <Progressbar 
            labelOutside="Off Ball Defense" 
            progress="{tangible_ratings.defense_off_ball.toString()}" 
            color="green"
        />
    </div>

    <div class="my-4">
        <Progressbar 
            labelOutside="Block" 
            progress="{tangible_ratings.defense_block.toString()}" 
            color="green"
        />
    </div>

    <div class="my-4">
        <Progressbar 
            labelOutside="Steal" 
            progress="{tangible_ratings.defense_steal.toString()}" 
            color="green"
        />
    </div>
</div>
