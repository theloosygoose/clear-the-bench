#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub mod ratings;
pub mod generators;
pub mod people;
pub mod team;

// remember to call `.manage(MyState::default())`
#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
fn generate_teams() -> Vec<team::teams::Team>{
  team::teams::Team::gen_teams()
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, generate_teams])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
