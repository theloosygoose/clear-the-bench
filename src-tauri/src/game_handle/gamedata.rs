use serde::Serialize;

use crate::{team::teams::{Team, }, people::Person};

#[derive(Debug, Serialize)]
pub struct GameData {
    save_name: String,
    save_id: u16,

    user_name: String,
    user_team_id: String,

    people: Vec<Person>,
    teams: Vec<Team>,
}
