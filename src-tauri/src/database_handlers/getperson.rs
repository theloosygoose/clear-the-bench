use std::str::FromStr;

use serde::Serialize;
use sqlx::FromRow;

use crate::generators::name::country::Country;
use crate::people::{Job, Person};
use crate::ratings::tangible::TangibleRatings;
use crate::ratings::*;

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct GetPerson {
    pub name: String,
    pub person_id: String,
    pub job: String,
    pub country: String,
    pub age: u16,
    pub active: u8,
    pub team: Option<String>,

    //Ratings
    #[sqlx(flatten)]
    pub personality: personality::Personality,
    #[sqlx(flatten)]
    pub intangibles: intangible::IntangibleRatings,
}

impl GetPerson {
    pub fn translate_to_person(self) -> Person {
        let name = self.name;
        let person_id = self.person_id;
        let job = Job::from_str(&self.job).unwrap();
        let country = Country::from_str(&self.country).unwrap();
        let age = self.age;
        let active = self.active;

        let team = self.team;

        let personality = self.personality;
        let intangibles = self.intangibles;
        let tangibles = TangibleRatings::gen(&intangibles, &personality);

        Person {
            name,
            person_id,
            job,
            country,
            age,
            active,
            team,
            personality,
            intangibles,
            tangibles,
        }
    }
}
