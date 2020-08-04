use crate::domain::todos::{Todo, Todos};
use crate::error_handler::CustomError;

pub async fn find_all() -> Result<Vec<Todos>, CustomError> {
    let todos = Todos::find_all()?;
    Ok(todos)
}

pub async fn find_by_id(id: i32) -> Result<Todos, CustomError> {
    let todo = Todos::find(id)?;
    Ok(todo)
}

pub async fn create(todo: Todo) -> Result<Todos, CustomError> {
    let todo = Todos::create(todo)?;
    Ok(todo)
}

pub async fn update(id: i32, todo: Todo) -> Result<Todos, CustomError> {
    let todo = Todos::update(id, todo)?;
    Ok(todo)
}

pub async fn delete(id: i32) -> Result<usize, CustomError> {
    let deleted_todo_id = Todos::delete(id)?;
    Ok(deleted_todo_id)
}
