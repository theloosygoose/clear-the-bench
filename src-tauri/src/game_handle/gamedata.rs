use serde::Serialize;

use crate::people::Person;
use crate::team::Team;

#[derive(Debug, Serialize)]
pub struct GameData {
    pub save_name: String,

    pub user_name: String,

    pub people: Vec<Person>,
    pub teams: Vec<Team>,
}
