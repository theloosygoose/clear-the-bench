#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use team::teams::Team;
pub mod ratings;
pub mod generators;
pub mod player;
pub mod team;

// remember to call `.manage(MyState::default())`
#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
fn generate_teams() -> Vec<Team>{
  Team::gen_teams()
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, generate_teams])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
