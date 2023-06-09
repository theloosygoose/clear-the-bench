#![allow(dead_code)]
pub mod coach;
pub mod gmanager;
pub mod journalist;
pub mod owner;
pub mod player;
pub mod scout;

use rand::Rng;
use serde::Serialize;
use sqlx::FromRow;
use strum::{Display, EnumString};

use crate::generators::id::generate_person_id;
use crate::generators::name::country::Country;
use crate::ratings::intangible::IntangibleRatings;
use crate::ratings::personality::Personality;

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

        let active = 1;

        let age: u16;
        match job {
            Job::Coach => age = rand::thread_rng().gen_range(32..70),
            Job::Owner => age = rand::thread_rng().gen_range(50..70),
            Job::Player => age = rand::thread_rng().gen_range(16..35),
            Job::Journalist => age = rand::thread_rng().gen_range(16..50),
            Job::Scout => age = rand::thread_rng().gen_range(25..60),
            Job::GeneralManager => age = rand::thread_rng().gen_range(27..70),
        }

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
