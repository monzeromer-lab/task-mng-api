use sea_orm::DbErr;
use super::model::{Entity as UserEntity, Model as User};

pub trait UserRepository {
    async fn signup(&self, user: User) -> Result<User, DbErr>;
    async fn find_by_email(&self, email: String) -> Result<User, DbErr>;
    async fn find_by_username(&self, username: String) -> Result<User, DbErr>;
    async fn update_password(&self, new_password: String, verify_code: String) -> Result<User, DbErr>;
    async fn active_user(&self, user_id: i32, verify_code: String) -> Result<bool, DbErr>;
}
