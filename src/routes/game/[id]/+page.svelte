<script lang="ts">
    import type { Team } from "$lib/types_rust";
    import type { PageData } from "./$types";
    import { Listgroup, ListgroupItem} from "flowbite-svelte";

    export let data: PageData;

    let teams:Array<Team> = data.data;

</script>


<Listgroup active class="w-48">
    <h3 class="text-xl p-2"> Teams </h3>
    {#each teams as team }
        <a href="/game/1/team/{team.name}">
            <ListgroupItem class="text-base font-semibold gap-2">
                    {team.name}
            </ListgroupItem>
        </a>
    {/each}
</Listgroup>


