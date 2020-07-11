mod todos;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

// サーバを3000番ポートで起動
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .configure(todos::init_routes)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
