<script lang="ts">
    import { page } from '$app/stores';

    import type { PlayerPerson } from '$lib/types_rust';
    import type { PageData } from './$types';

    export let data: PageData;

    console.log(data.players);

</script>

<h1>Individual Player Screen</h1>