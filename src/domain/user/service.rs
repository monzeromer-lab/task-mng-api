
use super::{model::Model as User, repository::{LoginState, UserRepository}};
use crate::{
    consts::cache::CACHE_VALUES,
    infrastructure::{auth::{Auth, AuthintecationToken}, persistence::repositories::user_repository::UserRepo},
    interfaces::dtos::user_dto::UserDto,
};
use argon2::Argon2;
use moka::future::Cache;
use sea_orm::{DatabaseConnection, DbErr};

#[derive(Debug, Clone)]
pub struct UserService<'a> {
    pub repository: UserRepo<'a>,
    pub db: &'a DatabaseConnection,
}

impl<'a> UserService<'a> {
    pub fn new(db: &'a DatabaseConnection, cache: &'a Cache<String, CACHE_VALUES>) -> Self {
        let repository = UserRepo { db: db, cache };
        Self { repository, db }
    }

    pub async fn signup(&self, user: UserDto) -> Result<User, sea_orm::DbErr> {
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
    
    pub async fn login_user(&self, email: String, password: String) -> Result<LoginState, DbErr> {
        let user = self.repository.find_by_email(email).await.ok().ok_or(DbErr::Custom(
            "couldn't find the user by email: {email}".to_string(),
        ))?;
        
        let password_authintec = Auth {
            argon: Argon2::default()
        }.verify_password(password, user.password_hash).ok().ok_or(DbErr::Custom("".to_string()))?;
        
        let token_payload = AuthintecationToken {
            username: user.username,
            user_id: user.id,
        };

        let token = Auth {
            argon: Argon2::default(),
        }.make_token(token_payload).ok().ok_or(DbErr::Custom("couldn't create the token for user with email: {email}".to_string()))?;
        
        if password_authintec {
           Ok(LoginState::Token(token)) 
        } else {
            Err(DbErr::Custom("Couldn't login user".to_string()))
        }
    }
}
