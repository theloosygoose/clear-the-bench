use serde::Serialize;

// use rand::{thread_rng, Rng};
use crate::ratings::intangible;
use crate::ratings::tangible;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Player {
    ratings: intangible::IntangibleRatings,
    skills: tangible::TangibleRatings,
}
