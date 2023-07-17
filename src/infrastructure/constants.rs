use lazy_static::lazy_static;

use super::models::EnvConfig;

lazy_static! {
    pub static ref ENV_CONFIG: EnvConfig = EnvConfig::new();
}
