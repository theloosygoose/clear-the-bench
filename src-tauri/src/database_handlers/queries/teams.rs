use sqlx::SqlitePool;

use sqlx::FromRow;
use serde::Serialize;

use crate::team::{Team, cities::City}; 

pub async fn insert_team(team: Team, db: &SqlitePool) {
    
    let result = sqlx::query(
        "INSERT INTO teams 
        (team_id, name,city_name, longitude, latitude, coach, owner, wins, losses, team_salary)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(team.id)
        .bind(team.name)
        .bind(team.city.city_name)
        .bind(team.city.longitude)
        .bind(team.city.latitude)
        .bind(team.coach)
        .bind(team.owner)
        .bind(team.wins)
        .bind(team.losses)
        .bind(team.team_salary)
        .execute(db)
        .await
        .unwrap();
}

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct GetTeam{
    pub team_id: String,
    pub name: String,
    
    #[sqlx(flatten)]
    pub city: City,
    
    pub coach:String,
    pub owner:String,
    pub wins: u8,
    pub losses: u8,
    pub team_salary: i16,
}

impl GetTeam{
    pub fn translate_to_team(self) -> Team {
        let id = self.team_id;
        let name = self.name;

        let coach: Option<String>;
        let owner: Option<String>;

        if &self.coach == "" {
            coach = None;
        } else {
            coach = Some(self.coach);
        }
        
        if &self.owner == "" {
            owner = None;
        } else {
            owner = Some(self.owner);
        }


        let city = self.city;
        Team {
            id,
            name,
            city,
            coach,
            owner,
            wins: self.wins,
            losses: self.losses,
            team_salary: self.team_salary,
        } 
        
    }
}


pub async fn get_teams(db: &SqlitePool) -> Vec<Team> {

        let mut teams = vec![];
    
        let teams_results = sqlx::query_as::<_,GetTeam>(
            "SELECT * FROM teams"
        )
            .fetch_all(db)
            .await
            .unwrap();
        
        for team_db in teams_results {
            let team = team_db.translate_to_team();
            teams.push(team);
        }; 

        return teams;
    
}
