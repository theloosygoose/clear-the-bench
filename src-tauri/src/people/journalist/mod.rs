use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
enum Reigon {
    National,
    TeamName(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct Journalist {
    reigon: Reigon,
    sub_stack: String,
    articles: Vec<String>,
}
