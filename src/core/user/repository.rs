use async_trait::async_trait;

use super::models::User;

#[derive(Debug, Clone)]
pub enum UserRepositorySelectAllError {
    UnexpectedError,
}

#[derive(Debug, Clone)]
pub enum UserRepositorySelectOneError {
    NotFound,
    UnexpectedError,
}

#[derive(Debug, Clone)]
pub enum UserRepositoryInsertError {
    LoginAlreadyUsed,
    UnexpectedError,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn select_all(&self) -> Result<Vec<User>, UserRepositorySelectAllError>;
    async fn select_one_by_id(&self, id: i32) -> Result<User, UserRepositorySelectOneError>;
    async fn select_one_by_login(
        &self,
        login: String,
    ) -> Result<User, UserRepositorySelectOneError>;
    async fn insert(
        &self,
        login: String,
        hash: String,
        salt: String,
    ) -> Result<i32, UserRepositoryInsertError>;
}
