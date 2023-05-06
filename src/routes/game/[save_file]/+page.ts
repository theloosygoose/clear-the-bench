import type { PageLoad } from "./$types"; 

export const load = (({ params }) => {
  
  let save_name = params.save_file;

  return {save_name};
  
}) satisfies PageLoad;
