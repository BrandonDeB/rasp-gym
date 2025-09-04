use crate::models::User;
use std::{collections::HashMap, sync::RwLock};
use once_cell::sync::Lazy;

    pub struct InMemoryUserRep

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn add_user(&self, user: User) -> Result<(), String>;
    async fn get_user(&self, email: &str) -> Result<User, String>;
    async fn user_exists(&self, email: &str) -> bool;
}

pub struct InMemoryUserRepo {
    users: Lazy<RwLock<HashMap<String, User>>>,
}

impl InMemoryUserRepo {
    pub fn new() -> Self {
        Self {
            users: Lazy::new(|| RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl UserRepo for InMemoryUserRepo {
    async fn add_user(&self, user:User) -> Result<(), String> {
        let mut users = self.users.write().unwrap();
        if users.contains_key(&users.email) {
            return Err(format!("User with email {} already exists", user.email));
        }
        users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &str) -> Result<User, String> {
        let users = self.users.read().unwrap();
        match users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(format!("User with email {} not found", email)),
        }
    }

    async user_exists(&self, email: &str) -> bool {
        let users = self.users.read().unwrap();
        users.contains_key(email)
    }
}
