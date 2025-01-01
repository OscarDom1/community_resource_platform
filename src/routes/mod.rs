pub mod resource_routes;
pub mod user_routes;

use actix_web::web::{self};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/resources").configure(resource_routes::init));
    cfg.service(web::scope("/users").configure(user_routes::init));
}
