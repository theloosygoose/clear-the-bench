
<script lang="ts">
    import type { PlayerPerson } from '$lib/types_rust';

    import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from 'flowbite-svelte';

    export let players: Array<PlayerPerson>;


</script>

<h1 class="dark:text-white mt-24 text-2xl"> Roster </h1>
<hr class="my-5">
<Table>
  <TableHead>
    <TableHeadCell>Name</TableHeadCell>
    <TableHeadCell>Country</TableHeadCell>
    <TableHeadCell>Height</TableHeadCell>
    <TableHeadCell>Age</TableHeadCell>
  </TableHead>
  <TableBody class="divide-y">
    {#each players as player}
    <TableBodyRow>
        <TableBodyCell>
          <a href="/game/1/player/{player.name}">{player.name}</a>
        </TableBodyCell>
        <TableBodyCell>{player.country}</TableBodyCell>
        <TableBodyCell>{player.job.Player.ratings.height}</TableBodyCell>
        <TableBodyCell>{player.age}</TableBodyCell>
    </TableBodyRow>
    {/each}
  </TableBody>
</Table>