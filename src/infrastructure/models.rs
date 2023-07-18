use crate::core::user::service::UserService;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{web::Data, Error as WebActixError, HttpResponse};
use hmac::Hmac;
use jwt::{Error as JwtError, Header, SignWithKey, Token, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::{digest::KeyInit, Sha256};
use std::collections::BTreeMap;
use std::env::var;
use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::rc::Rc;

use crate::infrastructure::constants::ENV_CONFIG;

#[derive(Serialize)]
pub struct ErrorDTO {
    message: String,
}

impl ErrorDTO {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

pub struct EnvConfig {
    jwt_secret: String,
    jwt_domain: String,
    access_control_allow_origin: String,
    access_control_allow_methods: String,
    access_control_allow_headers: String,
    access_control_allow_credentials: String,
}

impl EnvConfig {
    pub fn new() -> Self {
        Self {
            jwt_secret: var("JWT_SECRET").expect("ENV-variable `JWT_SECRET` must be set"),
            jwt_domain: var("JWT_DOMAIN").expect("ENV-variable `JWT_DOMAIN` must be set"),
            access_control_allow_origin: var("ACCESS_CONTROL_ALLOW_ORIGIN")
                .expect("ENV-variable `ACCESS_CONTROL_ALLOW_ORIGIN` must be set"),
            access_control_allow_methods: var("ACCESS_CONTROL_ALLOW_METHODS")
                .expect("ENV-variable `ACCESS_CONTROL_ALLOW_METHODS` must be set"),
            access_control_allow_headers: var("ACCESS_CONTROL_ALLOW_HEADERS")
                .expect("ENV-variable `ACCESS_CONTROL_ALLOW_HEADERS` must be set"),
            access_control_allow_credentials: var("ACCESS_CONTROL_ALLOW_CREDENTIALS")
                .expect("ENV-variable `ACCESS_CONTROL_ALLOW_CREDENTIALS` must be set"),
        }
    }

    pub fn check(&self) {}

    pub fn clone_jwt_secret(&self) -> String {
        self.jwt_secret.clone()
    }

    pub fn clone_jwt_domain(&self) -> String {
        self.jwt_domain.clone()
    }

    pub fn clone_access_control_allow_origin(&self) -> String {
        self.access_control_allow_origin.clone()
    }

    pub fn clone_access_control_allow_methods(&self) -> String {
        self.access_control_allow_methods.clone()
    }

    pub fn clone_access_control_allow_headers(&self) -> String {
        self.access_control_allow_headers.clone()
    }

    pub fn clone_access_control_allow_credentials(&self) -> String {
        self.access_control_allow_credentials.clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct JwtData {
    user_id: i32,
}

impl JwtData {
    pub fn new(user_id: i32) -> Self {
        Self { user_id }
    }

    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    fn get_key() -> Hmac<Sha256> {
        Hmac::<Sha256>::new_from_slice(ENV_CONFIG.clone_jwt_secret().as_bytes()).unwrap()
    }

    pub fn into_token(self) -> String {
        let key = JwtData::get_key();

        let mut claims = BTreeMap::new();

        claims.insert("user_id", self.user_id);

        let token_str = claims
            .sign_with_key(&key)
            .expect("Unexpected error while signing with key");

        token_str
    }

    pub fn from_token_str(token_str: &str) -> Result<JwtData, ()> {
        let key = JwtData::get_key();

        let token: Result<Token<Header, BTreeMap<String, i32>, _>, JwtError> =
            token_str.verify_with_key(&key);

        let token = token.unwrap();

        let claims = token.claims();

        let user_id = claims.get("user_id");

        if let None = user_id {
            return Err(());
        }

        let user_id = user_id.unwrap();

        Ok(JwtData::new(*user_id))
    }
}

#[derive(Default)]
pub struct AuthGuard {}

pub struct AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = WebActixError> + 'static,
{
    service: Rc<S>,
}

impl<S> Transform<S, ServiceRequest> for AuthGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = WebActixError> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = WebActixError;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = WebActixError> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = WebActixError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        let user_service = req
            .app_data::<Data<dyn UserService>>()
            .expect("user_service is missing")
            .clone();

        Box::pin(async move {
            let jwt_cookie = req.cookie("jwt");

            if jwt_cookie.is_none() {
                return Ok(ServiceResponse::new(
                    req.request().clone(),
                    HttpResponse::Unauthorized()
                        .json(ErrorDTO::new("You are unauthorized".to_string())),
                ));
            }

            let jwt_cookie = jwt_cookie.unwrap();

            let jwt = jwt_cookie.value();

            let jwt_data = JwtData::from_token_str(jwt);

            if jwt_data.is_err() {
                return Ok(ServiceResponse::new(
                    req.request().clone(),
                    HttpResponse::Unauthorized().json(ErrorDTO::new("Jwt is invalid".to_string())),
                ));
            }

            let jwt_data = jwt_data.unwrap();

            let user = user_service.get_one(jwt_data.get_user_id()).await;

            if user.is_err() {
                return Ok(ServiceResponse::new(
                    req.request().clone(),
                    HttpResponse::Unauthorized().json(ErrorDTO::new("Jwt is invalid".to_string())),
                ));
            }

            let res = service.call(req).await?;

            Ok(res)
        })
    }
}
