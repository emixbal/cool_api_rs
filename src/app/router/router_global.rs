use actix_web::web;
use crate::app::controllers::home::*;

pub fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(home))
    );
}