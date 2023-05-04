<script lang="ts">
    import { page } from '$app/stores';
</script>

<h1 class="text-bold text-3xl">
    Damn bruh code better 
    {$page.status}: {$page.error?.message}
</h1>

<a class="hover:text-lg" href="/"> Go Back Home </a>