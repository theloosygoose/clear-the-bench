<script lang="ts">
  import { Navbar, NavBrand, NavLi, NavUl, NavHamburger} from 'flowbite-svelte';
  import { DarkMode } from 'flowbite-svelte';
</script>

<Navbar let:hidden let:toggle>
  <NavBrand href="/">
    <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
        Basketball Sim
    </span>
    <DarkMode class=" text-xs"/>
  </NavBrand>

  <NavHamburger on:click={toggle} />
    <NavUl {hidden}>
      <NavLi href="/">Home</NavLi>
      <NavLi href="/settings">Settings</NavLi>
      <NavLi href="/contact">Contact</NavLi>
    </NavUl>
</Navbar>

<slot/>
