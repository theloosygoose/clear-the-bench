<script lang="ts">
    import { Navbar, NavBrand, NavHamburger, NavUl, NavLi} from 'flowbite-svelte';
    import type { LayoutData } from './$types';

    export let data:LayoutData;

    console.log(data);
</script>

<Navbar let:hidden let:toggle>
  <NavBrand href="/">
    <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
        Gameplay Screen
    </span>
  </NavBrand>

  <NavHamburger on:click={toggle} />
    <NavUl {hidden}>
      <NavLi href="/game/1/">Home</NavLi>
      <NavLi href="/game/1/team">Teams</NavLi>
      <NavLi href="/game/1/player">Players</NavLi>
    </NavUl>
</Navbar>

<slot/>
