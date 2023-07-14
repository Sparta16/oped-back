use serde::{Deserialize, Serialize};

use crate::core::user::models::{User, Users};

#[derive(Serialize)]
pub struct GetUsersResDTO {
    id: i32,
    login: String,
}

impl Into<GetUsersResDTO> for User {
    fn into(self) -> GetUsersResDTO {
        GetUsersResDTO {
            id: self.get_id(),
            login: self.clone_login(),
        }
    }
}

impl Into<Vec<GetUsersResDTO>> for Users {
    fn into(self) -> Vec<GetUsersResDTO> {
        self.into_users()
            .into_iter()
            .map(|item| item.into())
            .collect()
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

#[derive(Serialize)]
pub struct LoginUserResDTO {
    token: String,
}

impl LoginUserResDTO {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
