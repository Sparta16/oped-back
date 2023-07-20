use async_trait::async_trait;
use sha256::digest;
use std::sync::Arc;

use crate::core::user::{
    models::{User, Users},
    repository::UserRepository,
    service::{
        UserService, UserServiceGetAllError, UserServiceGetOneError, UserServiceLoginError,
        UserServiceRegisterError,
    },
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
    async fn get_all(&self) -> Result<Users, UserServiceGetAllError> {
        let result = self.user_repository.select_all().await;

        match result {
            Ok(users) => Ok(Users::new(users)),
            Err(error) => Err(error.into()),
        }
    }

    async fn get_one_by_id(&self, id: i32) -> Result<User, UserServiceGetOneError> {
        let result = self.user_repository.select_one_by_id(id).await;

        match result {
            Ok(user) => Ok(user),
            Err(error) => Err(error.into()),
        }
    }

    async fn get_one_by_login(&self, login: String) -> Result<User, UserServiceGetOneError> {
        let result = self.user_repository.select_one_by_login(login).await;

        match result {
            Ok(user) => Ok(user),
            Err(error) => Err(error.into()),
        }
    }

    async fn register(
        &self,
        login: String,
        mut password: String,
    ) -> Result<User, UserServiceRegisterError> {
        let salt = generate_salt(64);

        password.push_str(salt.as_str());

        let hash = digest(password);

        let result = self.user_repository.insert(login, hash, salt).await;

        if let Err(error) = result {
            return Err(error.into());
        }

        let user_id = result.unwrap();

        let result = self.user_repository.select_one_by_id(user_id).await;

        match result {
            Ok(user) => Ok(user),
            Err(error) => Err(error.into()),
        }
    }

    async fn login(
        &self,
        login: String,
        mut password: String,
    ) -> Result<String, UserServiceLoginError> {
        let user = self.user_repository.select_one_by_login(login).await;

        if let Err(error) = user {
            return Err(error.into());
        }

        let user = user.unwrap();

        let salt = user.clone_salt();

        password.push_str(salt.as_str());

        let hash = digest(password);

        if hash != user.clone_hash() {
            return Err(UserServiceLoginError::WrongPassword);
        }

        let jwt_data = JwtData::new(user.get_id());

        let token = jwt_data.into_token();

        Ok(token)
    }
}
