<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { currentgamedata } from "./data";

  export let data;

  
  onMount( async () => {
    currentgamedata.set(await invoke('load_game', {saveName: data.save_name}));
  })

  let gameData = {};
  
  currentgamedata.subscribe((n) => {
    gameData = n;
    console.log(gameData);
  });
  
  console.log(`LOADED ${data.save_name}`)
  
</script>
  
  
{#if Object.keys(gameData).length == 0}
  
  <h1>Loading...</h1>
  
{:else}
  <h1>Welcome to the {data.save_name}</h1>

  <h1>You have Loaded {gameData.save_name}</h1>

  <h1>Here is Player 1 {gameData.people[0].name}</h1>
{/if}
