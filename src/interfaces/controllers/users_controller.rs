use actix_web::{web, HttpResponse, Scope};
use crate::{consts::cache::CACHE_VALUES, domain::user::service::UserService, infrastructure::persistence::repositories::user_repository::{self, UserRepo}, interfaces::dtos::user_dto::{UserDto, UserLoginDto, UserResponseDto}, utils::map_db_error, AppState};
use uuid::Uuid;

pub fn user_routes() -> Scope {
    web::scope("/user")
        .route("/login", web::post().to(login_user))  
        .route("/signup", web::post().to(signup_user))
        .route("/reset", web::post().to(reset_password))
        .route("/active", web::get().to(active_user))
        .route("update_password", web::post().to(update_password))
}

pub async fn signup_user(app_state: web::Data<AppState>, body: web::Json<UserDto>) -> HttpResponse {
    let user_repository = UserRepo {
        db: &app_state.connection,
        cache: &app_state.app_cache
    };
    let user_service = UserService {
        repository: user_repository,
        db: &app_state.connection
    };
    
    match user_service.signup(body.0).await {
        Ok(user) => HttpResponse::Created().json(UserResponseDto {
            username: user.username,
            email: user.email,
            active: user.active,
            created_at: user.created_at,
        }),
        Err(error) => map_db_error(error),
    }
}

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

pub async fn reset_password(app_state: web::Data<AppState>, email: String) -> String {
    let reset_code = Uuid::new_v4().to_string();
    app_state.app_cache.insert("RESET_PASSWORD_CODE:".to_string(), CACHE_VALUES::RESET_PASSWORD_CODE(reset_code.clone(), email)).await;
    
    reset_code
}

pub async fn update_password(app_state: web::Data<AppState>, new_password: String, verify_code: String) -> HttpResponse {
    let cache_key = format!("RESET_PASSWORD_CODE:{}", verify_code);
    let user_repository = UserRepo {
        db: &app_state.connection,
        cache: &app_state.app_cache
    };
    let user_service = UserService { repository: user_repository, db: &app_state.connection };
    
    let cache_reset_value = app_state.app_cache.get(&cache_key).await;
    match cache_reset_value {
        Some(CACHE_VALUES::RESET_PASSWORD_CODE(code, email )) => {
            match user_service.update_password(new_password.to_string(), verify_code).await {
                Ok(user) => HttpResponse::Ok().finish(),
                Err(err) => map_db_error(err),
            }
        },
        None => HttpResponse::Forbidden().json("You are not allowed here!"),
        _ => HttpResponse::NoContent().finish()
    }
}

pub async fn active_user(app_state: web::Data<AppState>, verify_code: String) -> HttpResponse {
    let user_repository = UserRepo {
        db: &app_state.connection,
        cache: &app_state.app_cache
    };
    let user_service = UserService { repository: user_repository, db: &app_state.connection };
    
    let cache_key = format!("ACTIVATION_CODE:{}", verify_code);
    let verify_code = app_state.app_cache.get(&cache_key.to_string()).await;
    match verify_code {
        Some(CACHE_VALUES::ACTIVATION_CODE(verification_code, user_id)) => {
            match user_service.active_user(user_id, verification_code).await {
                Ok(state) => HttpResponse::Ok().json(state),
                Err(error) => map_db_error(error),
            }
        },
        None => HttpResponse::Forbidden().json("This activiation code is not out to date."),
        _ => HttpResponse::NoContent().finish()
    }
    
}
