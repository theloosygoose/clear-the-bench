<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import GameSavesAcordian from "../../lib/components/FrontPageComp/GameSavesAcordian.svelte";
    
    let name = 'gusti'
    async function greet() {
      await invoke('greet', {name})
    }
</script>

<h1 class="text-4xl font-bold dark:text-white mb-10">Create New Game</h1>

<h2 class="text-3xl font-bold dark:text-white mb-5 underline-offset-2">
  Load Game
</h2>
<GameSavesAcordian/>

<button on:click={greet}>Greeting </button>





