use actix_web::web;
use crate::app::router::*;

pub fn app_config(cfg: &mut web::ServiceConfig) {
    users::config(cfg);
}