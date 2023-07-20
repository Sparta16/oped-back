use async_trait::async_trait;

use super::{
    models::{User, Users},
    repository::{
        UserRepositoryInsertError, UserRepositorySelectAllError, UserRepositorySelectOneError,
    },
};

#[derive(Debug, Clone)]
pub enum UserServiceGetAllError {
    UnexpectedError,
}

impl Into<UserServiceGetAllError> for UserRepositorySelectAllError {
    fn into(self) -> UserServiceGetAllError {
        match self {
            Self::UnexpectedError => UserServiceGetAllError::UnexpectedError,
        }
    }
}

#[derive(Debug, Clone)]
pub enum UserServiceGetOneError {
    NotFound,
    UnexpectedError,
}

impl Into<UserServiceGetOneError> for UserRepositorySelectOneError {
    fn into(self) -> UserServiceGetOneError {
        match self {
            Self::NotFound => UserServiceGetOneError::NotFound,
            Self::UnexpectedError => UserServiceGetOneError::UnexpectedError,
        }
    }
}

#[derive(Debug, Clone)]
pub enum UserServiceRegisterError {
    LoginAlreadyUsed,
    UnexpectedError,
}

impl Into<UserServiceRegisterError> for UserRepositoryInsertError {
    fn into(self) -> UserServiceRegisterError {
        match self {
            Self::LoginAlreadyUsed => UserServiceRegisterError::LoginAlreadyUsed,
            Self::UnexpectedError => UserServiceRegisterError::UnexpectedError,
        }
    }
}

impl Into<UserServiceRegisterError> for UserRepositorySelectOneError {
    fn into(self) -> UserServiceRegisterError {
        match self {
            Self::NotFound | Self::UnexpectedError => UserServiceRegisterError::UnexpectedError,
        }
    }
}

#[derive(Debug, Clone)]
pub enum UserServiceLoginError {
    NotFound,
    WrongPassword,
    UnexpectedError,
}

impl Into<UserServiceLoginError> for UserRepositorySelectOneError {
    fn into(self) -> UserServiceLoginError {
        match self {
            Self::UnexpectedError => UserServiceLoginError::UnexpectedError,
            Self::NotFound => UserServiceLoginError::NotFound,
        }
    }
}

#[async_trait]
pub trait UserService: Sync + Send {
    async fn get_all(&self) -> Result<Users, UserServiceGetAllError>;
    async fn get_one_by_id(&self, id: i32) -> Result<User, UserServiceGetOneError>;
    async fn get_one_by_login(&self, login: String) -> Result<User, UserServiceGetOneError>;
    async fn register(
        &self,
        login: String,
        password: String,
    ) -> Result<User, UserServiceRegisterError>;
    async fn login(&self, login: String, password: String)
        -> Result<String, UserServiceLoginError>;
}
