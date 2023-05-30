use crate::models::{JwtData, User, Users};
use crate::repositories::{UserRepository, UserRepositoryError};
use crate::utils::generate_salt;
use async_trait::async_trait;
use sha256::digest;
use std::sync::Arc;

#[derive(Debug)]
pub enum UserServiceError {
    Message(String),
}

impl Into<UserServiceError> for UserRepositoryError {
    fn into(self) -> UserServiceError {
        match self {
            UserRepositoryError::Message(message) => UserServiceError::Message(message),
        }
    }
}

#[async_trait]
pub trait UserService: Sync + Send {
    async fn get_all(&self) -> Result<Users, UserServiceError>;
    async fn get_one(&self, id: i32) -> Result<User, UserServiceError>;
    async fn register(&self, login: String, password: String) -> Result<User, UserServiceError>;
    async fn login(&self, login: String, password: String) -> Result<String, UserServiceError>;
}

pub struct UserServiceImp {
    pub user_repository: Arc<dyn UserRepository>,
}

#[async_trait]
impl UserService for UserServiceImp {
    async fn get_all(&self) -> Result<Users, UserServiceError> {
        let users = self.user_repository.select_all().await;

        match users {
            Ok(users) => Ok(Users::new(users)),
            Err(error) => Err(error.into()),
        }
    }

    async fn get_one(&self, id: i32) -> Result<User, UserServiceError> {
        let user = self.user_repository.select_one(id).await;

        match user {
            Ok(user) => Ok(user),
            Err(error) => Err(error.into()),
        }
    }

    async fn register(
        &self,
        login: String,
        mut password: String,
    ) -> Result<User, UserServiceError> {
        let salt = generate_salt(64);

        password.push_str(salt.as_str());

        let hash = digest(password);

        let user_id = self.user_repository.insert(login, hash, salt).await;

        if let Err(error) = user_id {
            return Err(error.into());
        }

        let user_id = user_id.unwrap();

        let user = self.user_repository.select_one(user_id).await;

        match user {
            Ok(user) => Ok(user),
            Err(error) => Err(error.into()),
        }
    }

    async fn login(&self, login: String, mut password: String) -> Result<String, UserServiceError> {
        let user = self.user_repository.select_one_by_login(login).await;

        if let Err(error) = user {
            return Err(error.into());
        }

        let user = user.unwrap();

        let salt = user.salt;

        password.push_str(salt.as_str());

        let hash = digest(password);

        if hash != user.hash {
            return Err(UserServiceError::Message(
                "Пароль неверный! Как и ты (такой же невверный ежжи)".to_owned(),
            ));
        }

        let jwt_data = JwtData::new(user.id);

        let token = jwt_data.into_token();

        Ok(token)
    }
}
