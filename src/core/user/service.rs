use async_trait::async_trait;

use super::model::{User, UserServiceError, Users};

#[async_trait]
pub trait UserService: Sync + Send {
    async fn get_all(&self) -> Result<Users, UserServiceError>;
    async fn get_one(&self, id: i32) -> Result<User, UserServiceError>;
    async fn register(&self, login: String, password: String) -> Result<User, UserServiceError>;
    async fn login(&self, login: String, password: String) -> Result<String, UserServiceError>;
}
