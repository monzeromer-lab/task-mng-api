mod configs;
mod consts;
mod domain;
mod infrastructure;
mod interfaces;
mod utils;

use actix_web::{web, web::Data, App, HttpServer, Responder};
use consts::cache::CACHE_VALUES;
use infrastructure::{cache::create_cache_store, database::init_db};
use interfaces::controllers::{
    task_contoller::{create_task, find_task, task_routes},
    users_controller::{login_user, signup_user, user_routes},
};
use moka::future::Cache;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tracing::{event, instrument, level_filters::LevelFilter, Level};
use tracing_subscriber::{filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[instrument(level= Level::INFO, name= "HealthCheck")]
async fn index() -> impl Responder {
    event!(Level::INFO, "OK!");
    "Hello world!"
}

pub struct AppState {
    pub connection: DatabaseConnection,
    pub app_cache: Arc<Cache<String, CACHE_VALUES>>,
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
                app_cache: app_cache.clone(),
            }))
            .service(
                web::scope("/app")
                    .service(user_routes())
                    .service(task_routes()),
            )
            .service(web::scope("/").route("/health", web::get().to(index)))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
