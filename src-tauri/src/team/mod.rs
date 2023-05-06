use serde::Serialize;
use crate::people::{Job, Person};

use self::cities::City;

pub mod cities;

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct Team {
    pub id: String, 
    pub name: String,
    pub city: City,
    pub owner: Option<Person>,
    pub coach: Option<Person>,
    pub wins: u8,
    pub losses: u8,
    pub team_salary: i16,
}




impl Team {
    pub fn gen_teams() -> Vec<Team> {
       let cities = City::gen();

       let mut teams = vec![];

       for city in cities {
           let id = city.name.to_string() + &city.longitude.to_string();

           let team = Team {
               id,
               name: "BasketballTeam".to_string(),
               city,
               owner: Some(Person::gen_person(Job::Owner, Some(id))),
               coach: Some(Person::gen_person(Job::Coach, Some(id))),
               wins: 0,
               losses: 0,
               team_salary: 0,
               
           };
          
           teams.push(team);
       }


       teams
    }
    
}
