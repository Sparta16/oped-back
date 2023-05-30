use crate::models::{User, Users};
use crate::services::{UserService, UserServiceError};
use actix_web::web::{Data, Json};
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ErrorDTO {
    message: String,
}

impl ErrorDTO {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[derive(Serialize)]
struct GetUsersResDTO {
    id: i32,
    login: String,
}

impl Into<GetUsersResDTO> for User {
    fn into(self) -> GetUsersResDTO {
        GetUsersResDTO {
            id: self.id,
            login: self.login,
        }
    }
}

impl Into<Vec<GetUsersResDTO>> for Users {
    fn into(self) -> Vec<GetUsersResDTO> {
        self.0.into_iter().map(|item| item.into()).collect()
    }
}

#[derive(Deserialize)]
pub struct RegisterUserReqDTO {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUserReqDTO {
    pub login: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginUserResDTO {
    pub token: String,
}

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
            .json(ErrorDTO::new("User id is not a number".to_owned()));
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
        return HttpResponse::BadRequest().json(ErrorDTO::new(
            "Genius, login min 3 characters max 32".to_owned(),
        ));
    }

    if password.len() < 7 || password.len() > 32 {
        return HttpResponse::BadRequest().json(ErrorDTO::new(
            "Genius, password min 3 characters max 32".to_owned(),
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
        return HttpResponse::BadRequest().json(ErrorDTO::new(
            "Genius, login min 3 characters max 32".to_owned(),
        ));
    }

    if password.len() < 7 || password.len() > 32 {
        return HttpResponse::BadRequest().json(ErrorDTO::new(
            "Genius, password min 3 characters max 32".to_owned(),
        ));
    }

    let token = user_service.login(login, password).await;

    match token {
        Ok(token) => HttpResponse::Ok().json(LoginUserResDTO { token }),
        Err(UserServiceError::Message(message)) => {
            HttpResponse::BadRequest().json(ErrorDTO::new(message))
        }
    }
}
