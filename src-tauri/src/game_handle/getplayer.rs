use std::str::FromStr;

use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};
use serde::{Serialize, Deserialize};

use crate::generators::name::country::Country;
use crate::ratings::tangible::TangibleRatings;
use crate::team::teams::{Team, TeamName};
use crate::people::{Person, Job};
use crate::ratings::*;


#[derive(Clone, FromRow, Debug, Serialize)]
pub struct GetPlayer{
    pub name: String,
    pub player_id: String,
    pub job: String,
    pub country: String,
    pub age: u16,
    pub active: u8,
    pub team: String,
    
    //Ratings
    #[sqlx(flatten)]
    pub personality: personality::Personality,
    #[sqlx(flatten)]
    pub intangibles: intangible::IntangibleRatings,
}

impl GetPlayer {
    pub fn translate_to_person(&self) -> Person{
        let name = self.name;
        let player_id = self.player_id;
        let job = Job::from_str(&self.job).unwrap();
        let country = Country::from_str(&self.country).unwrap();
        let age = self.age;
        let active = self.active;
        let team = TeamName::from_str(&self.team).unwrap();
        let personality = self.personality;
        let intangibles = self.intangibles;
        let tangibles = TangibleRatings::gen(&intangibles, &personality);

        Person { 
            name, 
            player_id, 
            job, 
            country, 
            age, 
            active, 
            team, 
            personality, 
            intangibles, 
            tangibles 
        }
    }
}
