use actix_web::HttpResponse;
use sea_orm::DbErr;

pub fn map_db_error(err: DbErr) -> HttpResponse {
    match err {
        DbErr::RecordNotFound(_) => HttpResponse::NotFound().json("Record not found"),
        _ => HttpResponse::InternalServerError().json("Internal server error"),
    }
}