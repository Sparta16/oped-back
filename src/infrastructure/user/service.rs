use async_trait::async_trait;
use sha256::digest;
use std::sync::Arc;

use crate::core::user::{
    models::{User, UserServiceError, Users},
    repository::UserRepository,
    service::UserService,
};
use crate::infrastructure::{models::JwtData, utils::generate_salt};

pub struct UserServiceImp {
    user_repository: Arc<dyn UserRepository>,
}

impl UserServiceImp {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
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
        let user = self.user_repository.select_one_by_id(id).await;

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

        let user = self.user_repository.select_one_by_id(user_id).await;

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

        let salt = user.clone_salt();

        password.push_str(salt.as_str());

        let hash = digest(password);

        if hash != user.clone_hash() {
            return Err(UserServiceError::Message("Wrong password".to_owned()));
        }

        let jwt_data = JwtData::new(user.get_id());

        let token = jwt_data.into_token();

        Ok(token)
    }
}
