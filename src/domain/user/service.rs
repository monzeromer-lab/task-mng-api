use super::{model::Model as User, repository::UserRepository};
use crate::{
    constns::cache::CACHE_VALUES,
    infrastructure::persistence::repositories::user_repository::UserRepo,
};
use moka::future::Cache;
use sea_orm::{DatabaseConnection, DbErr};

#[derive(Debug, Clone)]
pub struct UserService<'a> {
    repository: UserRepo<'a>,
    db: &'a DatabaseConnection,
}

impl<'a> UserService<'a> {
    pub fn new(db: &'a DatabaseConnection, cache: &'a Cache<String, CACHE_VALUES>) -> Self {
        let repository = UserRepo {
            db: db,
            cache,
        };
        Self { repository, db }
    }

    pub async fn signup(&self, user: User) -> Result<User, sea_orm::DbErr> {
        self.repository.create_user(user).await
    }

    pub async fn find_user_by_email(&self, email: String) -> Result<User, DbErr> {
        self.repository.find_by_email(email).await
    }

    pub async fn find_user_by_username(&self, username: String) -> Result<User, DbErr> {
        self.repository.find_by_username(username).await
    }

    pub async fn update_password(
        &self,
        new_password: String,
        verify_code: String,
    ) -> Result<User, DbErr> {
        self.repository
            .update_password(new_password, verify_code)
            .await
    }

    pub async fn active_user(
        &self,
        user_id: i32,
        verify_code: String,
    ) -> Result<bool, sea_orm::DbErr> {
        self.repository.active_user(user_id, verify_code).await
    }
}
