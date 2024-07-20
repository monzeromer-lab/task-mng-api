use actix_web::{HttpResponse, Responder};
use tracing::{event, instrument, Level};

pub async fn create_task() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[instrument(level=Level::INFO, name="Task Contoller")]
pub async fn find_task() -> HttpResponse {
    event!(Level::INFO, "find task");
    HttpResponse::Ok().body("Hello, World!")
}
