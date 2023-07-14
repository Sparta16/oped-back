mod core;
mod infrastructure;

use actix_web::{
    error::InternalError,
    main,
    web::{Data, JsonConfig},
    App, HttpResponse, HttpServer,
};
use dotenv::dotenv;
use std::sync::{Arc, Mutex};

use crate::core::user::{models::User, repository::UserRepository, service::UserService};
use crate::infrastructure::controllers::configure;
use crate::infrastructure::user::{repository::MemoryUserRepository, service::UserServiceImp};

#[main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let shared_users: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(vec![]));
    let shared_index: Arc<Mutex<i32>> = Arc::new(Mutex::new(1));

    let user_repository: Arc<dyn UserRepository> =
        Arc::new(MemoryUserRepository::new(shared_users, shared_index));

    let user_service: Arc<dyn UserService> = Arc::new(UserServiceImp::new(user_repository));

    HttpServer::new(move || {
        let json_config = JsonConfig::default().error_handler(|err, _req| {
            InternalError::from_response(
                "",
                HttpResponse::BadRequest()
                    .content_type("application/json")
                    .body(format!(r#"{{"message":"{}"}}"#, err)),
            )
            .into()
        });

        App::new()
            .app_data(json_config)
            .app_data(Data::from(user_service.clone()))
            .configure(configure)
    })
    .bind(("127.0.0.1", 25565))?
    .run()
    .await
}
