mod infrastructure;
mod domain;
mod interfaces;
mod constns;

use std::sync::Arc;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer, Responder};
use constns::cache::CACHE_VALUES;
use infrastructure::cache::create_cache_store;
use infrastructure::database::init_db;
use moka::future::Cache;
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
    pub connection: DatabaseConnection,
    pub app_cache: Arc<Cache<String, CACHE_VALUES>>
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
    let app_cache = Arc::new(create_cache_store(10000));

    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(AppState {
            connection: database_connection.clone(),
            app_cache: app_cache.clone()
        }))
        .service(web::scope("/app").route("/health", web::get().to(index)))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
