use hmac::Hmac;
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use sha2::{digest::KeyInit, Sha256};
use std::collections::BTreeMap;

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
}

impl EnvConfig {
    pub fn new(jwt_secret: String, jwt_domain: String) -> Self {
        Self {
            jwt_secret,
            jwt_domain,
        }
    }

    pub fn check(&self) {}

    pub fn clone_jwt_secret(&self) -> String {
        self.jwt_secret.clone()
    }

    pub fn clone_jwt_domain(&self) -> String {
        self.jwt_domain.clone()
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
