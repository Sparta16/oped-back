#[derive(Debug, Clone)]
pub struct User {
    id: i32,
    login: String,
    hash: String,
    salt: String,
}

impl User {
    pub fn new(id: i32, login: String, hash: String, salt: String) -> Self {
        Self {
            id,
            login,
            hash,
            salt,
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn clone_login(&self) -> String {
        self.login.clone()
    }

    pub fn clone_salt(&self) -> String {
        self.salt.clone()
    }

    pub fn clone_hash(&self) -> String {
        self.hash.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Users(Vec<User>);

impl Users {
    pub fn new(users: Vec<User>) -> Self {
        Self(users)
    }

    pub fn into_users(self) -> Vec<User> {
        self.0
    }
}
