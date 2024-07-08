// application starting point

mod infrastructure;

use actix_web::web::Data;
use actix_web::{web, App, HttpServer, Responder};
use infrastructure::database::init_db;
use sea_orm::DatabaseConnection;
use tracing::level_filters::LevelFilter;
use tracing::{event, instrument, Level};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[instrument(level= Level::INFO, name= "HealthCheck")]
async fn index() -> impl Responder {
    event!(Level::INFO, "OK!");
    "Hello world!"
}

pub struct AppState {
    pub connection: DatabaseConnection
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    EnvFilter::builder()
        .with_default_directive(LevelFilter::ERROR.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let database_connection = init_db().await;

    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(AppState {
            connection: database_connection.clone()
        }))
        .service(web::scope("/app").route("/index.html", web::get().to(index)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
