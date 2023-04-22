use serde::Serialize;

// use rand::{thread_rng, Rng};
use crate::ratings::personality::Personality;
use crate::ratings::tangible;
use crate::ratings::intangible;


#[derive(Debug, Clone, Copy, Serialize)]
pub struct Player{
    ratings: intangible::IntangibleRatings,
    skills: tangible::TangibleRatings,
}

impl Player {
    pub fn gen_ratings(personality: &Personality) -> Player {
        let ratings = intangible::IntangibleRatings::gen();
        let skills = tangible::TangibleRatings::gen(&ratings, personality);

        return Player {
            ratings,
            skills,
        }
        
    }

    pub fn develop_ratings(&mut self, personality: &Personality) {
        // let upgrades: [u16; 20] = [0; 20];
        
        // let test = upgrades.map(|x| x + thread_rng().gen_range(0..5));
        
        self.ratings.off_awareness += 12;
        self.ratings.def_awareness += 12;

        self.skills.update(self.ratings, personality);
    }
}
