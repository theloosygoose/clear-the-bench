#![allow(unused_imports)]
use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};
use serde::{Serialize, Deserialize};
use tauri::api::path::data_dir;

use crate::team::teams::Team;
use crate::people::Person;
use crate::{ratings::*, database_handlers};

//std imports
use std::fs::create_dir_all;
use std::path::{PathBuf, Path};

//crate GetPlayer
use crate::database_handlers::getplayer::GetPlayer;

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
    
    let save_name = "save_03.db";
    
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
        
        let db = SqlitePool::connect(save_url).await.unwrap();
        
        let create_people_table = match sqlx::query("
            CREATE TABLE IF NOT EXISTS people
            (
              player_id   VARCHAR(250) PRIMARY KEY    NOT NULL,
              name        VARCHAR(100)                NOT NULL,
              country     VARCHAR(100)                NOT NULL,
              age         INTEGER                     NOT NULL,
              active      INTEGER                     NOT NULL DEFAULT 1,
              job         VARCHAR(50)                 NOT NULL,
              team        VARCHAR(100),
              work_ethic    INTEGER,
              intelligence  INTEGER, 
              creativity    INTEGER, 
              adaptability  INTEGER, 
              loyalty       INTEGER, 
              dog_factor    INTEGER, 
              strength      INTEGER,
              fluidity      INTEGER,
              burst         INTEGER,
              speed         INTEGER,
              height        INTEGER,
              wingspan      INTEGER,
              off_awareness INTEGER,
              def_awareness INTEGER,
              shot_form     INTEGER,
              touch         INTEGER,
              pass_accuracy INTEGER,
              ball_handling INTEGER,
              sliding       INTEGER,
              hands         INTEGER
            )")
            .execute(&db)
            .await {
                Ok(val) => val,
                Err(error) => panic!("Could not create people table:: {}", error)
            };

        let create_teams = match sqlx::query(
            "CREATE TABLE IF NOT EXISTS teams
            (
                team_id     VARCHAR(250) PRIMARY KEY NOT NULL,
                name        VARCHAR(100)             NOT NULL,
                owner       VARCHAR(100)             NOT NULL,
                coach       VARCHAR(100)             NOT NULL,
                wins        INTEGER                  NOT NULL DEFAULT 0,
                losses      INTEGER                  NOT NULL DEFAULT 0,
                team_salary INTEGER                  NOT NULL DEFAULT 0,
            )
            ")
            .execute(&db)
            .await {
                Ok(val) => val,
                Err(error) => panic!("Could not create teams table:: {}", error)
            };
            
        
        println!("Created People Table Result: {:?}", create_people_table);
        
        let pregenerated_players_count = 20;
        let mut n = 0;

        while n < pregenerated_players_count {
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
