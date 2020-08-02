use crate::error_handler::CustomError;
use crate::todos::{Todo, Todos};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

// 全件取得
#[get("/todos")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let todos = Todos::find_all()?;
    Ok(HttpResponse::Ok().json(todos))
}

// idから取得
#[get("/todos/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let todo = Todos::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(todo))
}

#[post("/todos")]
async fn create(todo: web::Json<Todo>) -> Result<HttpResponse, CustomError> {
    let todo = Todos::create(todo.into_inner())?;
    Ok(HttpResponse::Ok().json(todo))
}

#[put("/todos/{id}")]
async fn update(id: web::Path<i32>, todo: web::Json<Todo>) -> Result<HttpResponse, CustomError> {
    let todo = Todos::update(id.into_inner(), todo.into_inner())?;
    Ok(HttpResponse::Ok().json(todo))
}

#[delete("/todos/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_todo = Todos::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_todo })))
}

// 作成したエンドポイントをセットしてエクスポート
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}
