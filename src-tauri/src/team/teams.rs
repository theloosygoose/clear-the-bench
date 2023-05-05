use std::{fs, path::Path};

use rand_derive2::RandGen;
use serde::Serialize;
use strum_macros::{EnumString, Display};
use crate::{people::Person, generators::constants::TEAMS_AMT};

use tauri::PathResolver;


#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct Team {
    pub id: String, 
    pub name: String,
    pub city: City,
    pub owner: Option<Person>,
    pub coach: Option<Person>, 
    pub wins: u8,
    pub losses: u8,
    pub team_salary: i16,
}

#[derive(Debug, Clone, Serialize)]
pub struct City {
    pub name: String,
    pub long: f32,
    pub lat: f32,
}

impl City {
    pub fn read() -> Vec<Cities> {
        
        let cities_file_path = PathResolver::resource_dir();

        let cities_file_lines = fs::read_to_string()
    }
}



impl Team {
    pub fn gen_teams() -> Vec<Team> {
        let teams = vec![];

        let mut n = 0;


        return teams;
    }
    
}
