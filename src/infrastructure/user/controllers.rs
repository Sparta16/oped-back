use actix_web::cookie::time::Duration;
use actix_web::{
    cookie::Cookie,
    web::{get, post, scope, Data, Json, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};

use crate::core::user::service::{
    UserService, UserServiceGetAllError, UserServiceGetOneError, UserServiceLoginError,
    UserServiceRegisterError,
};
use crate::infrastructure::{
    constants::ENV_CONFIG,
    models::{AuthGuard, ErrorDTO, JwtData},
};

use super::models::{
    GetProfileResDTO, GetUserResDTO, LoginUserReqDTO, LoginUserResDTO, LogoutUserResDTO,
    RegisterUserReqDTO,
};

pub async fn get_users(user_service: Data<dyn UserService>) -> impl Responder {
    let users = user_service.get_all().await;

    match users {
        Ok(users) => {
            let dto: Vec<GetUserResDTO> = users.into();

            HttpResponse::Ok().json(dto)
        }
        Err(error) => match error {
            UserServiceGetAllError::UnexpectedError => {
                HttpResponse::InternalServerError().json(ErrorDTO::new("Внезапная ошибка"))
            }
        },
    }
}

pub async fn get_user(user_service: Data<dyn UserService>, req: HttpRequest) -> impl Responder {
    let login = req.match_info().query("login");

    let user = user_service.get_one_by_login(login.to_owned()).await;

    match user {
        Ok(user) => {
            let dto: GetUserResDTO = user.into();

            HttpResponse::Ok().json(dto)
        }
        Err(error) => match error {
            UserServiceGetOneError::NotFound => {
                HttpResponse::NotFound().json(ErrorDTO::new("Пользователь не найден"))
            }
            UserServiceGetOneError::UnexpectedError => {
                HttpResponse::InternalServerError().json(ErrorDTO::new("Внезапная ошибка"))
            }
        },
    }
}

pub async fn get_profile(user_service: Data<dyn UserService>, req: HttpRequest) -> impl Responder {
    let jwt_cookie = req.cookie("jwt").unwrap();

    let jwt = jwt_cookie.value();

    let jwt_data = JwtData::from_token_str(jwt).unwrap();

    let user = user_service.get_one_by_id(jwt_data.get_user_id()).await;

    match user {
        Ok(user) => {
            let dto: GetProfileResDTO = user.into();

            HttpResponse::Ok().json(dto)
        }
        Err(error) => match error {
            UserServiceGetOneError::NotFound => {
                HttpResponse::NotFound().json(ErrorDTO::new("Пользователь не найден"))
            }
            UserServiceGetOneError::UnexpectedError => {
                HttpResponse::InternalServerError().json(ErrorDTO::new("Внезапная ошибка"))
            }
        },
    }
}

pub async fn register_user(
    user_service: Data<dyn UserService>,
    dto: Json<RegisterUserReqDTO>,
) -> impl Responder {
    let dto = dto.into_inner();

    let login = dto.login;

    let password = dto.password;

    if login.len() < 3 || login.len() > 30 {
        return HttpResponse::BadRequest().json(ErrorDTO::new("Длина логина: 3-30 символов"));
    }

    if password.len() < 3 || password.len() > 30 {
        return HttpResponse::BadRequest().json(ErrorDTO::new("Длина пароля: 3-30 символов"));
    }

    let user = user_service.register(login, password).await;

    match user {
        Ok(user) => {
            let res_dto: GetUserResDTO = user.into();

            HttpResponse::Ok().json(res_dto)
        }
        Err(error) => match error {
            UserServiceRegisterError::LoginAlreadyUsed => {
                HttpResponse::BadRequest().json(ErrorDTO::new("Данный логин уже используется"))
            }
            UserServiceRegisterError::UnexpectedError => {
                HttpResponse::InternalServerError().json(ErrorDTO::new("Внезапная ошибка"))
            }
        },
    }
}

pub async fn login_user(
    user_service: Data<dyn UserService>,
    dto: Json<LoginUserReqDTO>,
) -> impl Responder {
    let dto = dto.into_inner();

    let login = dto.login;

    let password = dto.password;

    if login.len() < 3 || login.len() > 30 {
        return HttpResponse::BadRequest().json(ErrorDTO::new("Длина логина: 3-30 символов"));
    }

    if password.len() < 3 || password.len() > 30 {
        return HttpResponse::BadRequest().json(ErrorDTO::new("Длина пароля: 3-30 символов"));
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
                .json(LoginUserResDTO::default())
        }
        Err(error) => match error {
            UserServiceLoginError::NotFound | UserServiceLoginError::WrongPassword => {
                HttpResponse::BadRequest().json(ErrorDTO::new("Неверный логин или пароль"))
            }
            UserServiceLoginError::UnexpectedError => {
                HttpResponse::InternalServerError().json(ErrorDTO::new("Внезапная ошибка"))
            }
        },
    }
}

pub async fn logout_user() -> impl Responder {
    let cookie = Cookie::build("jwt", "")
        .domain(ENV_CONFIG.clone_jwt_domain())
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(Duration::ZERO)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(LogoutUserResDTO::default())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/users")
            .route("", get().to(get_users))
            .route("/profile", get().to(get_profile).wrap(AuthGuard::default()))
            .route("/registration", post().to(register_user))
            .route("/login", post().to(login_user))
            .route("/logout", post().to(logout_user).wrap(AuthGuard::default()))
            .route("/{login}", get().to(get_user)),
    );
}
