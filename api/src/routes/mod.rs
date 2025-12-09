use crate::middleware::auth::Auth;
use actix_web::web;

pub mod auth;
pub mod order;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(web::scope("/auth").configure(auth::init))
            .service(web::scope("/orders").wrap(Auth).configure(order::init)),
    );
}
