use lazy_static::lazy_static;
use std::env::var;

use super::models::EnvConfig;

lazy_static! {
    pub static ref ENV_CONFIG: EnvConfig = {
        let jwt_secret = var("JWT_SECRET").expect("ENV-variable `JWT_SECRET` must be set");
        let jwt_domain = var("JWT_DOMAIN").expect("ENV-variable `JWT_DOMAIN` must be set");
        let access_control_allow_origin = var("ACCESS_CONTROL_ALLOW_ORIGIN")
            .expect("ENV-variable `ACCESS_CONTROL_ALLOW_ORIGIN` must be set");
        let access_control_allow_methods = var("ACCESS_CONTROL_ALLOW_METHODS")
            .expect("ENV-variable `ACCESS_CONTROL_ALLOW_METHODS` must be set");
        let access_control_allow_headers = var("ACCESS_CONTROL_ALLOW_HEADERS")
            .expect("ENV-variable `ACCESS_CONTROL_ALLOW_HEADERS` must be set");
        let access_control_allow_credentials = var("ACCESS_CONTROL_ALLOW_CREDENTIALS")
            .expect("ENV-variable `ACCESS_CONTROL_ALLOW_CREDENTIALS` must be set");

        EnvConfig::new(
            jwt_secret,
            jwt_domain,
            access_control_allow_origin,
            access_control_allow_methods,
            access_control_allow_headers,
            access_control_allow_credentials,
        )
    };
}
