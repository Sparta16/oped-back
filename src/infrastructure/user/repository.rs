use async_trait::async_trait;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::core::user::{
    models::User,
    repository::{
        UserRepository, UserRepositoryInsertError, UserRepositorySelectAllError,
        UserRepositorySelectOneError,
    },
};

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

    fn lock_users(&self) -> MutexGuard<Vec<User>> {
        match self.shared_users.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    fn lock_index(&self) -> MutexGuard<i32> {
        match self.shared_index.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }
}

#[async_trait]
impl UserRepository for MemoryUserRepository {
    async fn select_all(&self) -> Result<Vec<User>, UserRepositorySelectAllError> {
        let users = self.lock_users();

        Ok((*users).clone())
    }

    async fn select_one_by_id(&self, id: i32) -> Result<User, UserRepositorySelectOneError> {
        let users = self.lock_users();

        let user = (*users).iter().find(|user| user.get_id() == id);

        match user {
            Some(user) => Ok(user.clone()),
            None => Err(UserRepositorySelectOneError::NotFound),
        }
    }

    async fn select_one_by_login(
        &self,
        login: String,
    ) -> Result<User, UserRepositorySelectOneError> {
        let users = self.lock_users();

        let user = (*users).iter().find(|user| user.clone_login() == login);

        match user {
            Some(user) => Ok(user.clone()),
            None => Err(UserRepositorySelectOneError::NotFound),
        }
    }

    async fn insert(
        &self,
        login: String,
        hash: String,
        salt: String,
    ) -> Result<i32, UserRepositoryInsertError> {
        let mut users = self.lock_users();

        if (*users)
            .iter()
            .find(|user| user.clone_login() == login)
            .is_some()
        {
            return Err(UserRepositoryInsertError::LoginAlreadyUsed);
        }

        let mut index = self.lock_index();

        let user_id = *index;

        let user = User::new(user_id, login, hash, salt);

        (*users).push(user);

        *index += 1;

        Ok(user_id)
    }
}
