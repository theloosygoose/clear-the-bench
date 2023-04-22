use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Owner {
    wealth: u32, 
}
