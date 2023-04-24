<script lang="ts">
    import { page } from '$app/stores';

</script>


<h1 class="text-bold text-3xl">
    Uhoh Error
    {$page.status}: {$page.error?.message}
</h1>

<a class="hover:text-lg" href="/"> Go Back Home </a>