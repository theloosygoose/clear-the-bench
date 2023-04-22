use serde::Serialize;

use crate::team::teams::TeamName;


#[derive(Debug, Clone, Serialize)]
enum Reigon {
    National,
    TeamName(TeamName),
}

#[derive(Debug, Clone, Serialize)]
pub struct Journalist {
    reigon: Reigon,
    sub_stack: String,
    articles: Vec<String>,
}
