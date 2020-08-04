use crate::routes;
use actix_web::web;

pub fn init(config: &mut web::ServiceConfig) {
    routes::todo_routes(config);
}
