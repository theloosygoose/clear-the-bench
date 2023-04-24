<script lang="ts">
    import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from 'flowbite-svelte';

    import type { Team } from '$lib/types_rust';

    export let teams: Array<Team>;

</script>

<Table class="mt-5">
  <TableHead>
    <TableHeadCell>Name</TableHeadCell>
    <TableHeadCell>Coach</TableHeadCell>
    <TableHeadCell>Wins</TableHeadCell>
    <TableHeadCell>Losses</TableHeadCell>
  </TableHead>
  <TableBody class="divide-y">
    {#each teams as team}
    <a href="game/[id]/team/{team.name}">
        <TableBodyRow>
            <TableBodyCell>{team.name}</TableBodyCell>
            <TableBodyCell>{team.coach}</TableBodyCell>
            <TableBodyCell>{team.wins}</TableBodyCell>
            <TableBodyCell>{team.losses}</TableBodyCell>
        </TableBodyRow>
    </a>
    {/each}
  </TableBody>
</Table>