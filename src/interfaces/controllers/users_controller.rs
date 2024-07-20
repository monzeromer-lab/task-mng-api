use actix_web::{web, HttpResponse};

use crate::{domain::user::service::UserService, infrastructure::persistence::repositories::user_repository::{self, UserRepo}, interfaces::dtos::user_dto::UserLoginDto, utils::map_db_error, AppState};

pub async fn login_user(app_state: web::Data<AppState>, body: web::Json<UserLoginDto>) -> HttpResponse {
    let user_repository = UserRepo {
        db: &app_state.connection,
        cache: &app_state.app_cache
    };
    let user_service = UserService { repository: user_repository, db: &app_state.connection };
    
    match user_service.login_user(body.email.to_string(), body.password.to_string()).await {
        Ok(state) => HttpResponse::Ok().json(state),
        Err(error) => map_db_error(error),
    }
}
