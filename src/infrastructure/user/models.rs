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

#[derive(Serialize)]
pub struct LoginUserResDTO {}

impl LoginUserResDTO {
    pub fn new() -> Self {
        Self {}
    }
}
