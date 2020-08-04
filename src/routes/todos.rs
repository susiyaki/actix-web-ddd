use crate::domain::todos::{usecase, Todo};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

// 全件取得
#[get("/todos")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let todos = usecase::find_all().await?;
    Ok(HttpResponse::Ok().json(todos))
}

// idから取得
#[get("/todos/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let todo = usecase::find_by_id(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(todo))
}

#[post("/todos")]
async fn create(todo: web::Json<Todo>) -> Result<HttpResponse, CustomError> {
    let todo = usecase::create(todo.into_inner()).await?;
    Ok(HttpResponse::Ok().json(todo))
}

#[put("/todos/{id}")]
async fn update(id: web::Path<i32>, todo: web::Json<Todo>) -> Result<HttpResponse, CustomError> {
    let todo = usecase::update(id.into_inner(), todo.into_inner()).await?;
    Ok(HttpResponse::Ok().json(todo))
}

#[delete("/todos/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_todo_id = usecase::delete(id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_todo_id })))
}

// 作成したエンドポイントをセットしてエクスポート
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}
