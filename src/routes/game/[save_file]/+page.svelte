<script lang="ts">
  import type { GameSaveData } from "$lib/types_rust";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { currentgamedata } from "./data";

  export let data:any;
  
  onMount( async () => {
    
    currentgamedata.set(await invoke('load_game', {saveName: data.save_name}));
    
  });

  let gameData: GameSaveData;
  currentgamedata.subscribe((n) => { gameData = n; console.log(gameData); });
  
  console.log(`LOADED ${data.save_name}`)

</script>
  
  
{#if gameData.people.length < 1}
  
  <h1>Loading...</h1>
  
{:else}
  <h1>Welcome to the {data.save_name}</h1>
  
  <h1>You have Loaded {gameData.save_name}</h1>
 
  <div class="flex">
    {#each gameData.teams as team }
      <a href="$/team/{team.id}">{team.city.city_name}</a>
    {/each}
  </div>
{/if}
