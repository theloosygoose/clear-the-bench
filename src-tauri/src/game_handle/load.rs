use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tauri::api::path::data_dir;

//std imports
use std::fs::create_dir_all;

//from my code
use crate::generators::constants::ROSTER_SIZE;
use crate::people::Person;
use crate::database_handlers;
use crate::team::Team;

#[tauri::command]
pub async fn load_game(save_name: String) {
    
    //Get route where data is stored
    let game_data = data_dir().unwrap();

    //create file for basketball_world database
    let save_data = game_data.join("basketball_world/saves/");

    //if the directory doesn't exist create it and the saves directory
    if !save_data.exists() {
        match create_dir_all(save_data) {
            Ok(_) => println!("Created Saves Directory"),
            Err(error) => println!("Error in Creating saves directory:: {}", error)
        }
    } 
    
    //Some weird ass string creation to create the sqlite database in the right place 
    let save_url_bigstr = "sqlite://".to_string() + 
                    &game_data.to_str().unwrap().to_string() + 
                    "/basketball_world/saves/" + 
                    save_name.as_str() + 
                    ".db";

    //then back to str
    let save_url = save_url_bigstr.as_str();
    
    //if database exists then read from it...else create new tables and add neccessary amount of
    //players 
    if !Sqlite::database_exists(save_url).await.unwrap_or(false) {

        match Sqlite::create_database(save_url).await {
            Ok(_) => println!("Created Savefile"),
            Err(error) => panic!("Error Creating Savefile!! {}", error),
        }
        
        //CONNECT TO THE DATABASE VIA POOL
        let db = SqlitePool::connect(save_url).await.unwrap();

        //Create People Tables
        database_handlers::migrations::people_table::create_people_table(&db)
            .await;
        
        //Create Teams Tables
        database_handlers::migrations::teams_table::create_teams_table(&db)
            .await;
        

        let teams = Team::gen_teams();
        
    } else {
        //Looks like the database file already exists so now read from it 
        println!("Savefile already exists");
        
        //Connect to the Sqlite DB that already exists
        let db = SqlitePool::connect(save_url).await.unwrap();
        let people = database_handlers::queries::people::get_people(&db)
            .await;
        

    }
}
