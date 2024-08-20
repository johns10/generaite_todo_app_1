use gen_todo::{config::Config, create_app};
use axum::Router;
use sea_orm::{Database, DatabaseConnection};
use uuid::Uuid;

pub async fn setup_test_db() -> DatabaseConnection {
    let db_url = format!("postgres://postgres:password@localhost/test_db_{}", Uuid::new_v4());
    // TODO: Create database and run migrations
    Database::connect(&db_url).await.expect("Failed to connect to test database")
}

pub async fn teardown_test_db(_conn: DatabaseConnection) {
    // TODO: Implement database teardown logic
}

// TODO: Implement create_test_task() when Task model is available
// pub fn create_test_task() -> Task {
//     Task {
//         id: Uuid::new_v4(),
//         title: "Test Task".to_string(),
//         description: "This is a test task".to_string(),
//         // ... other fields ...
//     }
// }

pub async fn create_test_app() -> Router {
    let config = Config::load().expect("Failed to load test configuration");
    let db = setup_test_db().await;
    create_app(config, db)
}

// TODO: Implement authenticated_request when authentication is set up
// pub async fn authenticated_request(app: Router, path: &str) -> Response {
//     // Create an authenticated request to the app
// }

// Add more utility functions as needed
