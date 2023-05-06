<script lang="ts">
  import type { GameSaveData } from '$lib/types_rust';

  import { AccordionItem, Accordion, Button } from 'flowbite-svelte';

  let sample_save_0:GameSaveData = {
    save_name: "Philadelphia Rebuild",
    save_file: "philadelphiarebuild",
    id: 0,
    user_team: "Philadelphia 76ers",
    year:2025,
  };

  let sample_save_1:GameSaveData = {
    save_name: "Boston Rebuild",
    save_file: "bostonrebuild",
    id: 1,
    user_team: "Boston Celtics",
    year:2032,
  };

  let game_saves = [sample_save_0, sample_save_1];
  
</script>

<main class="w-[50%]">
  <Accordion>
    {#each game_saves as save}
      <AccordionItem>
        <span slot="header">{save.save_name}</span>
        <p class="mb-2 text-gray-500 dark:text-gray-400">Team: {save.user_team}</p>
        <p class="text-gray-500 dark:text-gray-400">Year: {save.year}</p>
        <Button class="mt-4"  href="/game/{save.save_file}">
          Load Save
          <svg aria-hidden="true" class="ml-2 -mr-1 w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" 
              d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clip-rule="evenodd">
            </path>
          </svg>
        </Button>
      </AccordionItem>
    {/each}
  </Accordion>
</main>
