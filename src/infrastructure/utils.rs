use crate::infrastructure::constants::ENV_CONFIG;
use actix_web::http::header::{self, HeaderMap, HeaderValue};
use rand::Rng;
use std::iter;

pub const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

pub fn generate_salt(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(length).collect()
}

pub fn insert_access_control_allow_headers(headers: &mut HeaderMap) -> () {
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_str(ENV_CONFIG.clone_access_control_allow_origin().as_str()).unwrap(),
    );

    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_str(ENV_CONFIG.clone_access_control_allow_methods().as_str()).unwrap(),
    );

    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_str(ENV_CONFIG.clone_access_control_allow_headers().as_str()).unwrap(),
    );

    headers.insert(
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_str(ENV_CONFIG.clone_access_control_allow_credentials().as_str())
            .unwrap(),
    );
}
