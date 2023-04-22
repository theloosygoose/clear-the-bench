use serde::Serialize;
use strum_macros::{EnumString, Display};
use rand::{distributions::{Distribution, Standard}, Rng};

#[derive(Debug, Display, EnumString, Clone, Serialize)]
pub enum Country {
    UnitedStates, Canada, Cameroon,
    France, Australia, Germany,
    Serbia, Croatia, Spain,
    Brazil, Argentina, Lithuania,
    Nigeria, UnitedKingdom, Senegal,
    Turkey, Jamacia, PuertoRico,
    China, Latvia, SouthSudan,
}

impl Distribution<Country> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Country {
        match rng.gen_range(0..=22){
            0 => Country::UnitedStates,
            1 => Country::UnitedStates,
            2 => Country::UnitedStates,
            3 => Country::Canada,
            4 => Country::Cameroon,
            5 => Country::France,
            6 => Country::Australia,
            7 => Country::Germany,
            8 => Country::Serbia,
            9 => Country::Croatia,
            10 => Country::Spain,
            11 => Country::Brazil,
            12 => Country::Argentina,
            13 => Country::Lithuania,
            14 => Country::Nigeria,
            15 => Country::UnitedKingdom,
            16 => Country::Senegal,
            17 => Country::Turkey,
            18 => Country::Jamacia,
            19 => Country::PuertoRico,
            20 => Country::China,
            21 => Country::Latvia,
            22 => Country::SouthSudan,
            _ => Country::UnitedStates,
        }
    }
}
