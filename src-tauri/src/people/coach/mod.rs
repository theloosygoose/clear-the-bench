use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct Coach{
    wins: u16,
    losses: u16,
}
