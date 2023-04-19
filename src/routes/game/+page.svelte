<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'

    let teams = invoke('generate_teams');

    console.log(teams);
</script>

<h1>Welcome to the Danger Zone</h1>