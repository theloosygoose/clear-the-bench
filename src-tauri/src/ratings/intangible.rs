use serde::Serialize;

use crate::generators::gen_ratings::generate_rating; use crate::generators::constants::{MEAN_RTG, MEAN_STD_DEV, WIDE_STD_DEV, NARROW_STD_DEV};

#[derive(Debug, Clone, Copy, Serialize)]
pub struct IntangibleRatings {
//Physical Ratings
    pub strength: u16,
    pub fluidity: u16,
    pub burst: u16,
    pub speed: u16,
    pub height: u16,
    pub wingspan: u16,
//Offense_Defense Ratings
    pub off_awareness: u16,
    pub def_awareness: u16,
    pub shot_form: u16,
    pub touch: u16,
    pub pass_accuracy: u16,
    pub ball_handling: u16,
    pub sliding: u16,
    pub hands: u16,
}

impl IntangibleRatings{
    pub fn gen() -> IntangibleRatings {
        //Generate Height and Length Derivitive
        let height = generate_rating(MEAN_RTG, MEAN_STD_DEV);
        let wingspan = generate_rating(MEAN_RTG, MEAN_STD_DEV);

        //Generate INDEPENDENT RATINGS
        let strength = generate_rating(MEAN_RTG, MEAN_STD_DEV);
        let off_awareness = generate_rating(MEAN_RTG, MEAN_STD_DEV);
        let def_awareness= generate_rating(MEAN_RTG, MEAN_STD_DEV);
        let pass_accuracy = generate_rating(MEAN_RTG, MEAN_STD_DEV);

        //GENERATE HEIGHT DEPENDENT RATINGS
        //Calculate Inverse mean from Height
        // ** For Example if a player is tall then they should be less athletic *
        let height_weight= (100 - height) as f32;

        //Generate an Athleticism Mean
        //WIDER STD_DEV so that you can have some players that are tall and athletic
        //Doesn't have like a -1 speed and 100 burst
        let athleticism_mean = generate_rating(height_weight, WIDE_STD_DEV) as f32;

        //Generate Athletic traits that are based off height
        let fluidity = generate_rating(athleticism_mean, NARROW_STD_DEV);
        let burst = generate_rating(athleticism_mean, NARROW_STD_DEV);
        let speed = generate_rating(athleticism_mean, NARROW_STD_DEV);

        //Just Regular Height Weighted Ratings
        let touch = generate_rating(height_weight, MEAN_STD_DEV);
        let ball_handling = generate_rating(height_weight, MEAN_STD_DEV);
        let shot_form = generate_rating(height_weight, MEAN_STD_DEV);
        let sliding = generate_rating(height_weight, MEAN_STD_DEV);
        let hands = generate_rating(height_weight, MEAN_STD_DEV);

        //Generate Random Personality
        return IntangibleRatings{ 
            strength, fluidity, burst, speed, height, wingspan,
        //Offense_Defense Ratings
            off_awareness, def_awareness, shot_form, touch, 
            pass_accuracy, ball_handling, sliding, hands
        };
    }
}
