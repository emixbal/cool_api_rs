use actix_web::web;
use crate::app::controllers::home::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(home))
            .route(web::post().to(home))
    );
}