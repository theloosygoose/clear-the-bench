#![allow(unused_imports)]
use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};
use serde::{Serialize, Deserialize};

use crate::team::teams::Team;
use crate::people::Person;

use tauri::api::path::data_dir;

//std imports
use std::fs::create_dir_all;
use std::path::{PathBuf, Path};


#[derive(Clone, FromRow, Debug, Serialize)]
pub struct SaveData{
    pub save_name: String,
    
    pub user_firstname: String,
    pub user_lastname: String,
    pub user_age: u16,
    
    //GameData
    pub teams: Vec<Team>,
    pub people: Vec<Person>,
}


#[tauri::command]
pub async fn load_game() {
    
    let game_data = data_dir().unwrap();

    println!("DATA DIR::{:#?}", &game_data);

    let save_data = game_data.join("basketball_world/saves/");

    println!("SAVE DIR::{:#?}", &save_data);

    if !save_data.exists() {
        
        match create_dir_all(save_data) {
            Ok(_) => println!("Created Saves Directory"),
            Err(error) => println!("Error in Creating saves directory:: {}", error)
        }
    } 
    
    let save_name = "save_01.db";
    
    let save_url = "sqlite://".to_string() + 
                    &game_data.to_str().unwrap().to_string() + 
                    "/basketball_world/saves/" + 
                    &save_name.to_string();
    
    if !Sqlite::database_exists(&save_url.as_str()).await.unwrap_or(false) {
        println!("Creating Savefile at {}", save_url);

        match Sqlite::create_database(save_url.as_str()).await {
            Ok(_) => println!("Created Savefile"),
            Err(error) => panic!("Error Creating Savefile!! {}", error),
        }
            
    } else {
        println!("Savefile already exists");
    }

}
