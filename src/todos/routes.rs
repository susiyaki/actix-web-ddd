use super::model::Todo;
use actix_web::{get, web, HttpResponse, Responder};

// 全件取得
#[get("/todos")]
async fn find_all() -> impl Responder {
    HttpResponse::Ok().json(vec![
        Todo {
            id: 1,
            title: "title".to_string(),
            description: "description".to_string(),
            done: false,
        },
        Todo {
            id: 2,
            title: "title".to_string(),
            description: "description".to_string(),
            done: true,
        },
    ])
}

// idから取得
#[get("/todos/{id}")]
async fn find() -> impl Responder {
    HttpResponse::Ok().json(Todo {
        id: 2,
        title: "title".to_string(),
        description: "description".to_string(),
        done: true,
    })
}

// 作成したエンドポイントをセットしてエクスポート
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
}
