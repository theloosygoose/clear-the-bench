use serde::Serialize;
use sqlx::FromRow;

use crate::generators::{
    constants::{MEAN_RTG, MEAN_STD_DEV},
    gen_ratings::generate_rating,
};

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Personality {
    pub work_ethic: u16,
    pub intelligence: u16,
    pub creativity: u16,
    pub adaptability: u16,
    pub loyalty: u16,
    pub dog_factor: u16,
}

impl Personality {
    pub fn gen() -> Personality {
        Personality {
            work_ethic: generate_rating(MEAN_RTG, MEAN_STD_DEV),
            intelligence: generate_rating(MEAN_RTG, MEAN_STD_DEV),
            creativity: generate_rating(MEAN_RTG, MEAN_STD_DEV),
            adaptability: generate_rating(MEAN_RTG, MEAN_STD_DEV),
            loyalty: generate_rating(MEAN_RTG, MEAN_STD_DEV),
            dog_factor: generate_rating(MEAN_RTG, MEAN_STD_DEV),
        }
    }
}
