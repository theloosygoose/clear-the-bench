#![allow(dead_code)]
pub mod player;
pub mod coach;
pub mod owner;
pub mod scout;
pub mod journalist;

use rand::Rng;
use serde::Serialize;

use crate::generators::id::generate_person_id;
use crate::ratings::intangible::IntangibleRatings;
use crate::ratings::personality::Personality;
use crate::generators::name::country::Country;

use crate::generators::name::gen_name::gen_name;
use crate::ratings::tangible::TangibleRatings;


#[derive(Debug, Clone, Serialize)]
pub enum Job {
    Coach,
    Owner,
    Player,
    Journalist,
    Scout,
}

#[derive(Debug, Clone, Serialize)]
pub struct Person {
    pub name: String,
    pub player_id: String,
    pub job: Job,
    pub country: Country,
    pub age: u16,
    
    //Ratings
    pub personality: Personality,
    pub intangibles: IntangibleRatings,
    pub tangibles: TangibleRatings,
}


impl Person {
    pub fn gen_player() -> Person {
        let (name, country) = gen_name();
        let personality = Personality::gen();
        let age = rand::thread_rng().gen_range(16..35);
        
        let job = Job::Player;
        
        let player_id = generate_person_id(&name, &country, &age);
        let intangibles = IntangibleRatings::gen();
        let tangibles = TangibleRatings::gen(&intangibles, &personality);
        

        Person { 
            name, 
            player_id,
            job, 
            country, 
            age, 
            personality,
            tangibles,
            intangibles,
        }
    }

    pub fn gen_coach() -> Person {
        todo!();
    }

    pub fn gen_journalist() -> Person {
        todo!();
    }

    pub fn gen_owner() -> Person{
        todo!();
    }

    pub fn gen_scout() -> Person {
        todo!();
    }

    pub fn develop(&mut self) {
        let mut intangibles = self.intangibles;
        let mut tangibles = self.tangibles;
        
        let work_ethic = self.personality.work_ethic;
        let intelligence = self.personality.intelligence;

        intangibles.off_awareness += 10;
        intangibles.def_awareness += 10;
    }
    
}

