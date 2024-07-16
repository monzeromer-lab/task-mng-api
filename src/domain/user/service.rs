use sea_orm::{DatabaseConnection, DbErr};
use tracing::{info, instrument, level_filters::LevelFilter, Level};
use crate::infrastructure::{database::init_db, persistence::repositories::user_repository::{self, UserRepo}};
use crate::domain::user::model::{Model as User};

use super::repository::UserRepository;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Dn {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub active: bool,
    pub password_hash: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct UserService {
    repository: UserRepo,
    db: DatabaseConnection
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> Self {
        let repository = UserRepo {
            db
        };
        Self {
            repository,
            ..Default::default()
        }
    }
}

#[instrument(level= Level::INFO, name="Demo Function")]
pub async fn demo() {
    let db = init_db().await;
    let service = UserService::new(db);
    
    let new_user = User {
        username: "monzeromer-lab".to_string(),
        email: "monzer.a.omer@gmail.com".to_string(),
        password_hash: "aStrongPass".to_string(),
        ..Default::default()
    };
    service.repository.signup(new_user).await.expect("User is not saved");
    let the_user = service.repository.find_by_email("monzer.a.omer@gmail.com".to_string()).await;
    let value: Dn = the_user.expect("l");
    info!("the user is: {:#?}", value);

}
