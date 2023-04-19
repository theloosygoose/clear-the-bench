#![allow(dead_code)]
use serde;
use crate::generators::name::{country::Country, gen_name::gen_name};
use crate::ratings;
use rand::{thread_rng, Rng};


#[derive(Debug, serde::Serialize)]
pub struct Player {
    name: String,
    country: Country,
    age: u16,
    ratings: ratings::player_ratings::Ratings,
    skills: ratings::skill_ratings::Skills,
}

impl Player {
    pub fn gen() -> Player {
        let ratings = ratings::player_ratings::Ratings::gen();
        let skills = ratings::skill_ratings::Skills::gen(&ratings);


        let name_country = gen_name();
        let name = name_country.0;
        let country = name_country.1;

        return Player {
            name,
            country,
            age: thread_rng().gen_range(15..40),
            ratings,
            skills,
        }
    }

    pub fn develop(&mut self) {

        let upgrades: [u16; 20] = [0; 20];

        let test = upgrades.map(|x| x + thread_rng().gen_range(0..5));

        self.ratings.off_awareness += test[0] as u16;
        self.ratings.def_awareness += test[0] as u16;

        self.skills.update(self.ratings);
    }
}
