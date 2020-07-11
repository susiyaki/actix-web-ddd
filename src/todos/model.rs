use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}
