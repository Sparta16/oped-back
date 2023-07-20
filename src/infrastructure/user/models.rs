use serde::{Deserialize, Serialize};

use crate::core::user::models::{User, Users};

#[derive(Serialize)]
pub struct GetUserResDTO {
    id: i32,
    login: String,
}

impl Into<GetUserResDTO> for User {
    fn into(self) -> GetUserResDTO {
        GetUserResDTO {
            id: self.get_id(),
            login: self.clone_login(),
        }
    }
}

impl Into<Vec<GetUserResDTO>> for Users {
    fn into(self) -> Vec<GetUserResDTO> {
        self.into_users()
            .into_iter()
            .map(|item| item.into())
            .collect()
    }
}

#[derive(Serialize)]
pub struct GetProfileResDTO {
    id: i32,
    login: String,
}

impl Into<GetProfileResDTO> for User {
    fn into(self) -> GetProfileResDTO {
        GetProfileResDTO {
            id: self.get_id(),
            login: self.clone_login(),
        }
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

#[derive(Serialize, Default)]
pub struct LoginUserResDTO {}

#[derive(Serialize, Default)]
pub struct LogoutUserResDTO {}
