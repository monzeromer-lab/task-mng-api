use actix_web::{web, HttpResponse, Responder, Scope};
use tracing::{event, instrument, Level};

pub fn task_routes() -> Scope {
    web::scope("/task")
        .route("/create", web::post().to(create_task))
        .route("/find", web::get().to(find_task))
}

pub async fn create_task() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[instrument(level=Level::INFO, name="Task Contoller")]
pub async fn find_task() -> HttpResponse {
    event!(Level::INFO, "find task");
    HttpResponse::Ok().body("Hello, World!")
}
