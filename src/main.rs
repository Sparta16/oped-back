mod configurators;
mod contollers;
mod models;
mod repositories;
mod services;
mod utils;

use crate::configurators::configure;
use crate::repositories::{UserRepository, UserRepositoryImp};
use crate::services::{UserService, UserServiceImp};
use actix_web::error::InternalError;
use actix_web::middleware::Logger;
use actix_web::web::{Data, JsonConfig};
use actix_web::{App, HttpResponse, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    let user_repository: Arc<dyn UserRepository> = Arc::new(UserRepositoryImp {
        pool: Arc::new(pool.clone()),
    });

    let user_service: Arc<dyn UserService> = Arc::new(UserServiceImp { user_repository });

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
            .wrap(Logger::default())
            .app_data(json_config)
            .app_data(Data::from(user_service.clone()))
            .configure(configure)
    })
    .bind(("127.0.0.1", 25565))?
    .run()
    .await
}
