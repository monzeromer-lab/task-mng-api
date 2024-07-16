use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use crate::domain::user::model::{ Entity as UserEntity, Model as User, Column, ActiveModel};
use crate::domain::user::repository::UserRepository;

#[derive(Debug, Default, Clone)]
pub struct UserRepo {
    pub db: DatabaseConnection
}

impl UserRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl UserRepository for UserRepo {
    async fn signup(&self, user: User) -> Result<User, sea_orm::DbErr> {
        let new_user = ActiveModel {
            username: Set(user.username),
            email: Set(user.email),
            password_hash: Set(user.password_hash),
            ..Default::default()
        };
        
        let record_id = UserEntity::insert(new_user).exec(&self.db).await?.last_insert_id;
        UserEntity::find_by_id(record_id).one(&self.db).await?.ok_or(DbErr::RecordNotFound("Couldn't find this user".to_string()))
    }

    async fn find_by_email(&self, email: String) -> Result<User, sea_orm::DbErr> {
        UserEntity::find()
            .filter(Column::Email.eq(email))
            .one(&self.db)
            .await?.ok_or(DbErr::RecordNotFound("Couldn't find User with email: {email}".to_string()))
    }

    async fn find_by_username(&self, username: String) -> Result<User, sea_orm::DbErr> {
        UserEntity::find()
            .filter(Column::Username.eq(username))
            .one(&self.db)
            .await?.ok_or(DbErr::RecordNotFound("Couldn't find User with username: {username}".to_string()))
    }

    async fn update_password(&self, new_password: String, verify_code: String) -> Result<User, sea_orm::DbErr> {
        let user_record = UserEntity::find_by_id(23).one(&self.db).await?;
        let mut user: ActiveModel = user_record.ok_or(DbErr::RecordNotFound("User not found!".to_string()))?.into();
        user.password_hash = Set(new_password);
        
        user.update(&self.db).await
    }

    async fn active_user(&self, user_id: i32, verify_code: String) -> Result<bool, sea_orm::DbErr> {
        let user_record = UserEntity::find_by_id(user_id).one(&self.db).await?;
        let mut user: ActiveModel = user_record.ok_or(DbErr::RecordNotFound("User not found!".to_string()))?.into();
        user.active = Set(true);
        
        Ok(user.update(&self.db).await?.active)
    }
}