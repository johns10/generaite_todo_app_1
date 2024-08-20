use axum::Router;
use sea_orm::DatabaseConnection;

pub mod config;

pub fn create_app(_config: config::Config, _db: DatabaseConnection) -> Router {
    // TODO: Implement the actual app creation logic
    Router::new()
}
