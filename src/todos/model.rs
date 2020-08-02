use crate::db;
use crate::error_handler::CustomError;
use crate::schema::todos;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "todos"]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub done: bool,
}

// id採番後の構造体, Queryableなのでクエリを書ける
#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "todos"]
pub struct Todos {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}

impl Todos {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let todos = todos::table.load::<Todos>(&conn)?;
        Ok(todos)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let todo = todos::table.filter(todos::id.eq(id)).first(&conn)?;
        Ok(todo)
    }

    pub fn create(todo: Todo) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let todo = Todo::from(todo);
        let todo = diesel::insert_into(todos::table)
            .values(todo)
            .get_result(&conn)?;
        Ok(todo)
    }

    pub fn update(id: i32, todo: Todo) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let todo = diesel::update(todos::table)
            .filter(todos::id.eq(id))
            .set(todo)
            .get_result(&conn)?;
        Ok(todo)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(todos::table.filter(todos::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl Todo {
    fn from(todo: Todo) -> Todo {
        Todo {
            title: todo.title,
            description: todo.description,
            done: todo.done,
        }
    }
}
