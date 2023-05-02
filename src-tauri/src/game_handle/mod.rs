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
    
    let save_name = "monkeytest7.db";
    
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
              personality_work_ethic    INTEGER,
              personality_intelligence  INTEGER, 
              personality_creativity    INTEGER, 
              personality_adaptability  INTEGER, 
              personality_loyalty       INTEGER, 
              personality_dog_factor    INTEGER, 
              intangibles_strength      INTEGER,
              intangibles_fluidity      INTEGER,
              intangibles_burst         INTEGER,
              intangibles_speed         INTEGER,
              intangibles_height        INTEGER,
              intangibles_wingspan      INTEGER,
              intangibles_off_awareness INTEGER,
              intangibles_def_awareness INTEGER,
              intangibles_shot_form     INTEGER,
              intangibles_touch         INTEGER,
              intangibles_pass_accuracy INTEGER,
              intangibles_ball_handle   INTEGER,
              intangibles_sliding       INTEGER,
              intangibles_hands         INTEGER
            )")
            .execute(&db)
            .await {
                Ok(val) => val,
                Err(error) => panic!("Could not create Database:: {}", error)
            };
        
        println!("Created People Table Result: {:?}", create_people_table);
        
        let pregenerated_players_count = 400;
        let mut n = 0;

        while n < pregenerated_players_count {
            let player = Person::gen_player();
            
            let result = sqlx::query(
                "INSERT INTO people 
                (player_id, name, country, age, active, job, team, 
                personality_work_ethic, personality_intelligence, personality_creativity, 
                personality_adaptability, personality_loyalty, personality_dog_factor,   
                
                intangibles_strength, intangibles_fluidity, intangibles_burst, intangibles_speed, 
                intangibles_height, intangibles_wingspan, intangibles_off_awareness, intangibles_def_awareness,
                intangibles_shot_form, intangibles_touch, intangibles_pass_accuracy, intangibles_ball_handle, 
                intangibles_sliding, intangibles_hands)
                VALUES (?,?,?,?,?,?,?,
                        ?,?,?,
                        ?,?,?,
                        ?,?,?,?,
                        ?,?,?,?,
                        ?,?,?,?,
                        ?,?)"
                )
                .bind(player.player_id)
                .bind(player.name)
                .bind(player.country.to_string())
                .bind(player.age)
                .bind(player.active)
                .bind(player.job.to_string())
                .bind(player.team.to_string())
                .bind(player.personality.work_ethic)
                .bind(player.personality.intelligence)
                .bind(player.personality.creativity)
                .bind(player.personality.adaptability)
                .bind(player.personality.loyalty)
                .bind(player.personality.dog_factor)
                .bind(player.intangibles.strength)
                .bind(player.intangibles.fluidity)
                .bind(player.intangibles.burst)
                .bind(player.intangibles.speed)
                .bind(player.intangibles.height)
                .bind(player.intangibles.wingspan)
                .bind(player.intangibles.off_awareness)
                .bind(player.intangibles.def_awareness)
                .bind(player.intangibles.shot_form)
                .bind(player.intangibles.touch)
                .bind(player.intangibles.pass_accuracy)
                .bind(player.intangibles.ball_handling)
                .bind(player.intangibles.sliding)
                .bind(player.intangibles.hands)
                .execute(&db)
                .await
                .unwrap();

            println!("Added Player into DB");
            n += 1;
        }
    } else {
        println!("Savefile already exists");
    }
}
