import type { PageLoad } from "./$types"; 

export const load = (({ params }) => {
  
  let id = params.id;

  return {id};
  }) satisfies PageLoad;
