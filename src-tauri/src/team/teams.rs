use rand_derive2::RandGen;
use serde::Serialize;
use strum_macros::{EnumString, Display};

use crate::generators::constants::{ROSTER_SIZE, TEAMS_AMT};
use crate::generators::name::names::FirstNameEnglish;

use crate::people::{Person, Job};

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct Team {
    pub name: TeamName,
    pub owner: String,
    pub coach: String, 
    pub wins: u8,
    pub losses: u8,
    pub team_salary: i16,
    pub players: Vec<Person>,
}

#[derive(Debug, Display, RandGen, EnumString, Clone, Copy, Serialize)]
pub enum TeamName{
    Sixers, Bucks, Celtics,
    Cavaliers, Knicks, Nets,
    Hawks, Heat, Bulls,
    Raptors, Pacers, Wizards,
    Magic, Hornets, Pistons,
    Grizzles, Nuggets, Kings,
    Suns, Clippers, Warriors,
    Lakers, Timberwolves, Thunder,
    Pelicans, Mavericks, Jazz,
    TrailBlazers, Rockets, Spurs,
}


impl Team {
    
    pub fn gen_teams() -> Vec<Team> {
        let team_names = [
            TeamName::Sixers, TeamName::Bulls, TeamName::Bucks, TeamName::Celtics, TeamName::Cavaliers, TeamName::Knicks,
            TeamName::Nets, TeamName::Hawks, TeamName::Heat, TeamName::Raptors, TeamName::Pacers, TeamName::Wizards,
            TeamName::Magic, TeamName::Hornets, TeamName::Pistons, TeamName::Grizzles, TeamName::Nuggets, TeamName::Kings,
            TeamName::Suns, TeamName::Clippers, TeamName::Warriors, TeamName::Lakers, TeamName::Timberwolves, TeamName::Thunder,
            TeamName::Pelicans, TeamName::Mavericks, TeamName::Jazz, TeamName::TrailBlazers, TeamName::Rockets, TeamName::Spurs,
        ];
        
        let mut teams = vec![];
        let mut n = 0;
        let mut p: usize = 0;

        while n < TEAMS_AMT {
            
            let mut team_players = vec![];
            let mut k = 0;

            while k < ROSTER_SIZE {
                team_players.push(Person::gen_player());

                k += 1;
            }
                
            teams.push(Team{
                name: team_names[p],
                coach: FirstNameEnglish::generate_random().to_string(), 
                owner: FirstNameEnglish::generate_random().to_string(),
                wins: 0,
                losses: 0,
                team_salary: 0,
                players: team_players,
            });
            p += 1;
            n += 1;
        };
        return teams;
    }

    pub fn develop_team(&mut self){
        let players: &mut Vec<Person> = &mut self.players; 
        
        for player in players.iter_mut() {
            match player.job {
                Job::Player(ref mut z) => z.develop_ratings(&player.personality),
                _ => println!("This is not a Player"),
            }
            
        }
        

    }
}
