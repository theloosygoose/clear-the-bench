<script lang="ts">
  import { Navbar, NavLi, NavUl} from 'flowbite-svelte';
  
</script>
  
<Navbar let:hidden>
    <NavUl {hidden}>
      <NavLi href="/">Home</NavLi>
      <NavLi href="./teams">Teams</NavLi>
      <NavLi href="./players">Players</NavLi>
    </NavUl>
</Navbar>

<slot/>
