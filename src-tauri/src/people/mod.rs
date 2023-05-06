#![allow(dead_code)]
pub mod player;
pub mod coach;
pub mod owner;
pub mod scout;
pub mod journalist;
pub mod gmanager;

use rand::Rng;
use serde::Serialize;
use sqlx::FromRow;
use strum::{EnumString, Display};

use crate::generators::id::generate_person_id;
use crate::ratings::intangible::IntangibleRatings;
use crate::ratings::personality::Personality;
use crate::generators::name::country::Country;

use crate::generators::name::gen_name::gen_name;
use crate::ratings::tangible::TangibleRatings;

#[derive(Debug, Clone, Serialize, EnumString, Display)]
pub enum Job {
    Coach,
    Owner,
    Player,
    Journalist,
    Scout,
    GeneralManager,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Person {
    pub name: String,
    pub person_id: String,
    pub job: Job,
    pub country: Country,
    pub age: u16,
    pub active: u8,
    pub team: Option<String>,
    
    //Ratings
    #[sqlx(flatten)]
    pub personality: Personality,
    #[sqlx(flatten)]
    pub intangibles: IntangibleRatings,
    #[sqlx(flatten)]
    pub tangibles: TangibleRatings,
}


impl Person {
    pub fn gen_person(job: Job, team: Option<String>) -> Person {
        let (name, country) = gen_name();
        let personality = Personality::gen();
        let age = rand::thread_rng().gen_range(16..35);
        let active = 1;
        
        let person_id = generate_person_id(&name, &country, &age);
        let intangibles = IntangibleRatings::gen();
        let tangibles = TangibleRatings::gen(&intangibles, &personality);

        Person { 
            name, 
            person_id,
            job, 
            active,
            country, 
            team,
            age, 
            personality,
            tangibles,
            intangibles,
        }
    }

    
    pub fn develop(&mut self) -> &mut Self {
        self.intangibles.off_awareness += 10;
        self.intangibles.def_awareness += 10;
        
        self
    }
    
}
