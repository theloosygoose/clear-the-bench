use serde::Serialize;

use self::cities::City;

pub mod cities;

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub city: City,
    pub owner: Option<String>,
    pub coach: Option<String>,
    pub wins: u8,
    pub losses: u8,
    pub team_salary: i16,
}

impl Team {
    pub fn gen_teams() -> Vec<Team> {
        let cities = City::gen();

        let mut teams = vec![];

        for city in cities {
            let id = (city.city_name.to_string() + &city.longitude.to_string())
                .to_lowercase()
                .trim()
                .to_string();

            let team = Team {
                id,
                name: "BasketballTeam".to_string(),
                city,
                owner: None,
                coach: None,
                wins: 0,
                losses: 0,
                team_salary: 0,
            };
            teams.push(team);
        }

        teams
    }
}
