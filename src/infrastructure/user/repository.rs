use async_trait::async_trait;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::core::user::model::UserRepositoryError;
use crate::core::user::{model::User, repository::UserRepository};

pub struct MemoryUserRepository {
    shared_users: Arc<Mutex<Vec<User>>>,
    shared_index: Arc<Mutex<i32>>,
}

impl MemoryUserRepository {
    pub fn new(shared_users: Arc<Mutex<Vec<User>>>, shared_index: Arc<Mutex<i32>>) -> Self {
        Self {
            shared_users,
            shared_index,
        }
    }
}

#[async_trait]
impl UserRepository for MemoryUserRepository {
    async fn select_all(&self) -> Result<Vec<User>, UserRepositoryError> {
        let users: MutexGuard<Vec<User>> = match self.shared_users.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        Ok((*users).clone())
    }

    async fn select_one_by_id(&self, id: i32) -> Result<User, UserRepositoryError> {
        let users: MutexGuard<Vec<User>> = match self.shared_users.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let user = (*users).iter().find(|user| user.get_id() == id);

        match user {
            Some(user) => Ok(user.clone()),
            None => Err(UserRepositoryError::Message(
                "User does`t exist".to_string(),
            )),
        }
    }

    async fn select_one_by_login(&self, login: String) -> Result<User, UserRepositoryError> {
        let users: MutexGuard<Vec<User>> = match self.shared_users.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let user = (*users).iter().find(|user| user.clone_login() == login);

        match user {
            Some(user) => Ok(user.clone()),
            None => Err(UserRepositoryError::Message(
                "User does`t exist".to_string(),
            )),
        }
    }

    async fn insert(
        &self,
        login: String,
        hash: String,
        salt: String,
    ) -> Result<i32, UserRepositoryError> {
        let mut users: MutexGuard<Vec<User>> = match self.shared_users.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        if (*users).iter().find(|user| user.clone_login() == login) {
            return Err(UserRepositoryError::Message(
                "This login already taken".to_string(),
            ));
        }

        let mut index: MutexGuard<i32> = match self.shared_index.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let user_id = *index;

        let user = User::new(user_id, login, hash, salt);

        (*users).push(user);

        *index += 1;

        Ok(user_id)
    }
}
