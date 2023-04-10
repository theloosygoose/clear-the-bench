<script lang="ts">
  import "../app.css";
</script>

<h1 class="text-3xl font-bold underline">Hello from the Layout</h1>

<slot />

<style lang="postcss">

  :global(html) {
    background-color: theme(colors.gray.100);
  }

</style>