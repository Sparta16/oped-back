use crate::contollers::{get_user, get_users, login_user, register_user};
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(scope("/api/v1").configure(|cfg| {
        cfg.service(
            scope("/users")
                .route("", get().to(get_users))
                .route("/reg", post().to(register_user))
                .route("/login", post().to(login_user))
                .route("/{id}", get().to(get_user)),
        );
    }));
}
