#![allow(dead_code)]
use serde;

use super::player_ratings::Ratings;

#[derive(Debug, serde::Serialize)]
pub struct Skills {
    creation_off_ball: u16,
    creation_on_ball: u16,

    shot_movement: u16,

    shot_three: u16,
    shot_mid: u16,
    shot_close: u16,

    rim_finishing: u16,
    
    foul_drawing: u16,

    pass_iq: u16,
    pass_creativity: u16,
    pass_vision: u16,

    defense_on_ball: u16,
    defense_off_ball: u16,

    defense_steal: u16,
    defense_block: u16,

    hustle: u16,
}


impl Skills {
    pub fn gen (ratings: &Ratings) -> Skills {
        let creation_off_ball = (
            ratings.speed + 
            ratings.burst + 
            ratings.intelligence + 
            ratings.off_awareness + 
            ratings.fluidity)/(5); 

        let creation_on_ball = (
            ratings.speed + 
            ratings.burst + 
            ratings.strength + 
            ratings.creativity + 
            ratings.intelligence + 
            ratings.fluidity + 
            ratings.ball_handling) / (7);


        let shot_movement  = (
            ratings.fluidity + 
            ratings.shot_form + 
            ratings.touch + 
            ratings.strength) / (4);
        let shot_three = (
            ratings.shot_form + 
            ratings.touch) / (2);
        let shot_mid = (
            ratings.shot_form + 
            ratings.touch) / (2); 
        let shot_close = (
            ratings.touch + 
            ratings.strength + 
            ratings.burst + 
            ratings.height) / (4);


        let rim_finishing = (
            ratings.burst + 
            ratings.fluidity + 
            ratings.strength + 
            ratings.touch + 
            ratings.creativity) / (5);

        let foul_drawing = (
            ratings.strength + 
            ratings.creativity + 
            ratings.intelligence ) / (3);


        let pass_iq = (
            ratings.intelligence + 
            ratings.off_awareness) / (2);
        let pass_creativity = (
            ratings.creativity + 
            ratings.intelligence + 
            ratings.pass_accuracy + 
            ratings.off_awareness) / (4);
        let pass_vision = (
            ratings.height + 
            ratings.off_awareness + 
            ratings.intelligence) / (3);


        let defense_on_ball = (
            ratings.sliding + 
            ratings.strength + 
            ratings.wingspan + 
            ratings.burst + 
            ratings.intelligence + 
            ratings.hands) / (6);
        let defense_off_ball = (
            ratings.def_awareness + 
            ratings.off_awareness + 
            ratings.intelligence + 
            ratings.speed + 
            ratings.burst + 
            ratings.wingspan) / (6);

        let defense_steal = (
            ratings.hands + 
            ratings.intelligence) / (2);
        let defense_block = (
            ratings.height + 
            ratings.burst + 
            ratings.wingspan) / (3);


        let hustle = (
            ratings.dog_factor + 
            ratings.loyalty + 
            ratings.hands) / (3);

        return Skills {
            creation_off_ball,
            creation_on_ball,
            shot_movement,
            shot_three,
            shot_mid,
            shot_close,
            rim_finishing,
            foul_drawing,
            pass_iq,
            pass_creativity,
            pass_vision,
            defense_on_ball,
            defense_off_ball,
            defense_steal,
            defense_block,
            hustle,
        }
    }

    pub fn update(&mut self, ratings: Ratings){
        self.creation_off_ball = (
            ratings.speed + 
            ratings.burst + 
            ratings.intelligence + 
            ratings.off_awareness + 
            ratings.fluidity)/(5); 

        self.creation_on_ball = (
            ratings.speed + 
            ratings.burst + 
            ratings.strength + 
            ratings.creativity + 
            ratings.intelligence + 
            ratings.fluidity + 
            ratings.ball_handling) / (7);


        self.shot_movement  = (
            ratings.fluidity + 
            ratings.shot_form + 
            ratings.touch + 
            ratings.strength) / (4);
        self.shot_three = (
            ratings.shot_form + 
            ratings.touch) / (2);
        self.shot_mid = (
            ratings.shot_form + 
            ratings.touch) / (2); 
        self.shot_close = (
            ratings.touch + 
            ratings.strength + 
            ratings.burst + 
            ratings.height) / (4);


        self.rim_finishing = (
            ratings.burst + 
            ratings.fluidity + 
            ratings.strength + 
            ratings.touch + 
            ratings.creativity) / (5);

        self.foul_drawing = (
            ratings.strength + 
            ratings.creativity + 
            ratings.intelligence ) / (3);


        self.pass_iq = (
            ratings.intelligence + 
            ratings.off_awareness) / (2);
        self.pass_creativity = (
            ratings.creativity + 
            ratings.intelligence + 
            ratings.pass_accuracy + 
            ratings.off_awareness) / (4);
        self.pass_vision = (
            ratings.height + 
            ratings.off_awareness + 
            ratings.intelligence) / (3);


        self.defense_on_ball = (
            ratings.sliding + 
            ratings.strength + 
            ratings.wingspan + 
            ratings.burst + 
            ratings.intelligence + 
            ratings.hands) / (6);
        self.defense_off_ball = (
            ratings.def_awareness + 
            ratings.off_awareness + 
            ratings.intelligence + 
            ratings.speed + 
            ratings.burst + 
            ratings.wingspan) / (6);

        self.defense_steal = (
            ratings.hands + 
            ratings.intelligence) / (2);
        self.defense_block = (
            ratings.height + 
            ratings.burst + 
            ratings.wingspan) / (3);


        self.hustle = (
            ratings.dog_factor + 
            ratings.loyalty + 
            ratings.hands) / (3);

        
    }
}
