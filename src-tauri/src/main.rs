#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub mod ratings;
pub mod generators;
pub mod people;
pub mod team;
pub mod game_handle;
pub mod database_handlers;

use crate::game_handle::load::load_game;

// remember to call `.manage(MyState::default())`
#[tauri::command]
fn greet(name: &str){
  println!("Hello, {}!", name)
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, load_game])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
