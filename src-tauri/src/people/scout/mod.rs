use serde::Serialize;

use crate::people::player::Player;

#[derive(Debug, Clone, Serialize)]
pub struct Scout {
    bias: String,
    players_scouted: Vec<Player>,
}
