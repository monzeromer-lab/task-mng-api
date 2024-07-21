use std::error::Error;

use actix_web::HttpResponse;
use sea_orm::DbErr;

pub fn map_db_error(err: DbErr) -> HttpResponse {
    match err {
        DbErr::RecordNotFound(_) => HttpResponse::NotFound().json("Not Found"),
        error => {
            let couse = error.to_string();
            let err_msg = format!("Internal Server Error:
                {couse}");
            HttpResponse::InternalServerError().json(err_msg)
        },
    }
}
