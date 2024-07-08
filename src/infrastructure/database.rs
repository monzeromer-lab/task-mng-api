// setup
use sea_orm::{Database, DatabaseConnection};
use tracing::{event, instrument, Level};

#[instrument(level= Level::INFO, name = "DatabaseConnectionEstablishment")]
pub async fn init_db() -> DatabaseConnection {
    let db: DatabaseConnection = Database::connect("sqlite://taskmng.sqlite?mode=rwc")
        .await
        .expect("Couldn't connect to the database");
    event!(Level::INFO, "Connected to the database.");
    db
}
