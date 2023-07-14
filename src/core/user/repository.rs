use async_trait::async_trait;

use super::models::{User, UserRepositoryError};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn select_all(&self) -> Result<Vec<User>, UserRepositoryError>;
    async fn select_one_by_id(&self, id: i32) -> Result<User, UserRepositoryError>;
    async fn select_one_by_login(&self, login: String) -> Result<User, UserRepositoryError>;
    async fn insert(
        &self,
        login: String,
        hash: String,
        salt: String,
    ) -> Result<i32, UserRepositoryError>;
}
