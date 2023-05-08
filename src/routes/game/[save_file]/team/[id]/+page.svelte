<script lang="ts">
  import { currentgamedata } from "../../data";
  
  export let data: any;

  let team_id = data.id;
  console.log(team_id);
  
  
  let team_people = $currentgamedata.people.filter((x) => {
      return x.team == team_id;   
  });
  
  let team_data = $currentgamedata.teams.filter((x) => {
      return x.id == team_id;   
  });
  
</script>

<h1 class="text-4xl">{team_data[0].city.city_name} {team_data[0].name} </h1>
<h2></h2>

{#each team_people as person}
  <h1>{person.name}</h1>
{/each}
