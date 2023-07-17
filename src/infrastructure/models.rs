use hmac::Hmac;
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use sha2::{digest::KeyInit, Sha256};
use std::collections::BTreeMap;
use std::env::var;

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
    pub user_id: i32,
}

impl JwtData {
    pub fn new(user_id: i32) -> Self {
        Self { user_id }
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
}
