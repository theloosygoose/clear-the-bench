use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};
use serde::Serialize;
use tauri::api::path::data_dir;

//std imports
use std::fs::create_dir_all;


//from my code
use crate::generators::constants::PREGENERATED_PLAYERS_COUNT;

use crate::team::teams::Team;
use crate::people::Person;

use crate::database_handlers;


//modules
#[derive(Clone, FromRow, Debug, Serialize)]
pub struct GameData{
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
    
    let save_name = "save_04.db";
    
    let save_url_bigstr = "sqlite://".to_string() + 
                    &game_data.to_str().unwrap().to_string() + 
                    "/basketball_world/saves/" + 
                    &save_name.to_string();

    let save_url = save_url_bigstr.as_str();
    
    if !Sqlite::database_exists(save_url).await.unwrap_or(false) {
        
        println!("Creating Savefile at {}", save_url);

        match Sqlite::create_database(save_url).await {
            Ok(_) => println!("Created Savefile"),
            Err(error) => panic!("Error Creating Savefile!! {}", error),
        }
        
        //CONNECT TO THE DATABASE VIA POOL
        let db = SqlitePool::connect(save_url).await.unwrap();

        //Create People Tables
        database_handlers::migrations::people_table::create_people_table(&db);
        //Create Teams Tables
        database_handlers::migrations::teams_table::create_teams_table(&db);
        
        
        let mut n = 0;
        while n < PREGENERATED_PLAYERS_COUNT {
            let person = Person::gen_player();
            
            database_handlers::queries::people::insert_person(person, &db);
            
            n += 1;
        }
        
    } else {
        println!("Savefile already exists");
        
        let db = SqlitePool::connect(save_url).await.unwrap();

        let people = database_handlers::queries::people::get_people(&db);
        
    }
}
