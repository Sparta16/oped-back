use lazy_static::lazy_static;
use std::env::var;

use super::models::EnvConfig;

lazy_static! {
    pub static ref ENV_CONFIG: EnvConfig = {
        let jwt_secret = var("JWT_SECRET").expect("ENV-variable `JWT_SECRET` must be set");
        let jwt_domain = var("JWT_DOMAIN").expect("ENV-variable `JWT_DOMAIN` must be set");

        EnvConfig::new(jwt_secret, jwt_domain)
    };
}
