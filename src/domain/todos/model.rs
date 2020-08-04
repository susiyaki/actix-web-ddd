use crate::schema::todos;
use serde::{Deserialize, Serialize};

impl Todo {
    fn from(todo: Todo) -> Todo {
        Todo {
            title: todo.title,
            description: todo.description,
            done: todo.done,
        }
    }
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "todos"]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "todos"]
pub struct Todos {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}

