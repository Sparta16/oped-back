use actix_web::{
    cookie::Cookie,
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};

use crate::core::user::{models::UserServiceError, service::UserService};
use crate::infrastructure::{constants::ENV_CONFIG, models::ErrorDTO};

use super::models::{GetUsersResDTO, LoginUserReqDTO, LoginUserResDTO, RegisterUserReqDTO};

pub async fn get_users(user_service: Data<dyn UserService>) -> impl Responder {
    let users = user_service.get_all().await;

    match users {
        Ok(users) => {
            let users: Vec<GetUsersResDTO> = users.into();

            HttpResponse::Ok().json(users)
        }
        Err(UserServiceError::Message(message)) => {
            HttpResponse::InternalServerError().json(ErrorDTO::new(message))
        }
    }
}

pub async fn get_user(user_service: Data<dyn UserService>, req: HttpRequest) -> impl Responder {
    let id = req.match_info().query("id");

    let id = id.parse::<i32>();

    if let Err(_) = id {
        return HttpResponse::BadRequest()
            .json(ErrorDTO::new("User id is not a number".to_string()));
    }

    let id = id.unwrap();

    let user = user_service.get_one(id).await;

    match user {
        Ok(user) => {
            let user: GetUsersResDTO = user.into();

            HttpResponse::Ok().json(user)
        }
        Err(UserServiceError::Message(message)) => {
            HttpResponse::InternalServerError().json(ErrorDTO::new(message))
        }
    }
}

pub async fn register_user(
    user_service: Data<dyn UserService>,
    dto: Json<RegisterUserReqDTO>,
) -> impl Responder {
    let dto = dto.into_inner();

    let login = dto.login;

    let password = dto.password;

    if login.len() < 3 || login.len() > 32 {
        return HttpResponse::BadRequest()
            .json(ErrorDTO::new("Login must be 3-32 characters".to_string()));
    }

    if password.len() < 3 || password.len() > 32 {
        return HttpResponse::BadRequest().json(ErrorDTO::new(
            "Password must be 3-32 characters".to_string(),
        ));
    }

    let user = user_service.register(login, password).await;

    match user {
        Ok(user) => {
            let user: GetUsersResDTO = user.into();

            HttpResponse::Ok().json(user)
        }
        Err(UserServiceError::Message(message)) => {
            HttpResponse::InternalServerError().json(ErrorDTO::new(message))
        }
    }
}

pub async fn login_user(
    user_service: Data<dyn UserService>,
    dto: Json<LoginUserReqDTO>,
) -> impl Responder {
    let dto = dto.into_inner();

    let login = dto.login;

    let password = dto.password;

    if login.len() < 3 || login.len() > 32 {
        return HttpResponse::BadRequest()
            .json(ErrorDTO::new("Login must be 3-32 characters".to_string()));
    }

    if password.len() < 3 || password.len() > 32 {
        return HttpResponse::BadRequest().json(ErrorDTO::new(
            "Password must be 3-32 characters".to_string(),
        ));
    }

    let token = user_service.login(login, password).await;

    match token {
        Ok(token) => {
            let cookie = Cookie::build("jwt", token)
                .domain(ENV_CONFIG.clone_jwt_domain())
                .path("/")
                .secure(true)
                .http_only(true)
                .finish();

            HttpResponse::Ok()
                .cookie(cookie)
                .json(LoginUserResDTO::new())
        }
        Err(UserServiceError::Message(message)) => {
            HttpResponse::BadRequest().json(ErrorDTO::new(message))
        }
    }
}

use actix_web::web::{get, post, scope, ServiceConfig};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/users")
            .route("", get().to(get_users))
            .route("/registration", post().to(register_user))
            .route("/login", post().to(login_user))
            .route("/{id}", get().to(get_user)),
    );
}
