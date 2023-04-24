<script lang="ts">
    import { Navbar, NavBrand, NavHamburger, NavUl, NavLi} from 'flowbite-svelte';
    export let data;

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
      <NavLi href="/">Home</NavLi>
      <NavLi href="/teams">Teams</NavLi>
      <NavLi href="/players">Players</NavLi>
    </NavUl>
</Navbar>

<slot/>
