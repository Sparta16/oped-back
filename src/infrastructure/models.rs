use hmac::Hmac;
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use sha2::digest::KeyInit;
use sha2::Sha256;
use std::collections::BTreeMap;

#[derive(Serialize)]
pub struct ErrorDTO {
    message: String,
}

impl ErrorDTO {
    pub fn new(message: String) -> Self {
        Self { message }
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
        let secret = std::env::var("JWT_SECRET").expect("env variable `JWT_SECRET` should be set");

        let key = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();

        key
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
