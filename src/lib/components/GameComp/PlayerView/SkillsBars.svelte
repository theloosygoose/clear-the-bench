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
            labelOutside="On Ball Creation" 
            progress="{tangible_ratings.creation_on_ball.toString()}" 
            color="green"
        />
    </div>

    <div class="my-4">
        <Progressbar 
            labelOutside="Off Ball Creation" 
            progress="{tangible_ratings.creation_off_ball.toString()}" 
            color="green"
        />
    </div>

    <div class="my-4">
        <Progressbar 
            labelOutside="Shot Close" 
            progress="{tangible_ratings.shot_close.toString()}" 
            color="green"
        />
    </div>

    <div class="my-4">
        <Progressbar 
            labelOutside="Shot Mid" 
            progress="{tangible_ratings.shot_mid.toString()}" 
            color="green"
        />
    </div>

    <div class="my-4">
        <Progressbar 
            labelOutside="Three Pointer" 
            progress="{tangible_ratings.shot_three.toString()}" 
            color="green"
        />
    </div>

    <div class="my-4">
        <Progressbar 
            labelOutside="Movement Shot" 
            progress="{tangible_ratings.shot_movement.toString()}" 
            color="green"
        />
    </div>

    <div class="my-4">
        <Progressbar 
            labelOutside="Rim Finishing" 
            progress="{tangible_ratings.rim_finishing.toString()}" 
            color="green"
        />
    </div>



</div>
