mod core;
mod infrastructure;

use actix_web::dev::{Service, ServiceResponse};
use actix_web::{
    error::InternalError,
    http::Method,
    main,
    web::{Data, JsonConfig},
    App, HttpResponse, HttpServer,
};
use dotenv::dotenv;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use crate::core::user::{models::User, repository::UserRepository, service::UserService};
use crate::infrastructure::user::{repository::MemoryUserRepository, service::UserServiceImp};
use crate::infrastructure::{
    constants::ENV_CONFIG, controllers::configure, utils::insert_access_control_allow_headers,
};

#[main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    ENV_CONFIG.check();

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
            .wrap_fn(|service_request, app_routing| -> Pin<Box<dyn Future<Output = Result<ServiceResponse, actix_web::Error>>>> {
                let http_request = service_request.request();
                let is_pre_flight = http_request.method() == Method::OPTIONS
                    && http_request.headers().contains_key("origin")
                    && http_request
                        .headers()
                        .contains_key("access-control-request-method");

                if is_pre_flight {
                    return Box::pin(async move {
                        let mut service_response = ServiceResponse::new(
                            service_request.request().clone(),
                            HttpResponse::NoContent().finish(),
                        );

                        let headers = service_response.response_mut().headers_mut();

                        insert_access_control_allow_headers(headers);

                        return Ok(service_response);
                    });
                };

                let fut = app_routing.call(service_request);

                Box::pin(async {
                    let mut service_response: ServiceResponse = fut.await?;

                    let headers = service_response.response_mut().headers_mut();

                    insert_access_control_allow_headers(headers);

                    Ok(service_response)
                })
            })
    })
    .bind(("127.0.0.1", 25565))?
    .run()
    .await
}
