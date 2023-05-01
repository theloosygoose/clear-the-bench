use serde::Serialize;

// use rand::{thread_rng, Rng};
use crate::ratings::tangible;
use crate::ratings::intangible;


#[derive(Debug, Clone, Copy, Serialize)]
pub struct Player{
    ratings: intangible::IntangibleRatings,
    skills: tangible::TangibleRatings,
}
