use actix_web::web::{scope, ServiceConfig};

use super::user::controllers::configure as configure_user;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(scope("/api/v1").configure(configure_user));
}
