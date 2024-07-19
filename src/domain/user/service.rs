use moka::future::Cache;
use sea_orm::{DatabaseConnection, DbErr};
use crate::{constns::cache::CACHE_VALUES, infrastructure::persistence::repositories::user_repository::UserRepo};
use super::{model::Model, repository::UserRepository};

#[derive(Debug, Clone)]
pub struct UserService {
    repository: UserRepo,
    db: DatabaseConnection
}

impl UserService {
    pub fn new(db: DatabaseConnection, cache: Cache<String, CACHE_VALUES>) -> Self {
        let db_clone = db.clone();
        let repository = UserRepo {
            db: db_clone,
            cache,
        };
        Self {
            repository,
            db
        }
    }
    
    pub async fn find_user_by_email(&self, email: String) -> Result<Model, DbErr> {
        self.repository.find_by_email(email).await
    }
    
    pub async fn find_user_by_username(&self, username: String) -> Result<Model, DbErr> {
        self.repository.find_by_username(username).await
    }
    
    pub async fn update_password(&self, new_password: String, verify_code: String) -> Result<Model, DbErr>{
        self.repository.update_password(new_password, verify_code).await
    }
    
    
}
