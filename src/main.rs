// main.rs

use actix_web::{App, HttpServer};
use crate::app::routers::router_global::app_config;

pub mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
            .configure(app_config)
    })
    .bind("127.0.0.1:8889")?
    .run()
    .await
}
