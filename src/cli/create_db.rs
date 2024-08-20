use clap::{Command, ArgMatches};
use crate::config::Config;
use sea_orm::{Database, DatabaseConnection};

pub fn create_db_command() -> Command {
    Command::new("create-db")
        .about("Creates the database based on configuration")
}

pub async fn handle_create_db(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let db_url = format!(
        "postgres://{}:{}@{}:{}/postgres",
        config.database.user,
        config.database.password,
        config.database.host,
        config.database.port
    );

    let db: DatabaseConnection = Database::connect(&db_url).await?;

    let create_db_query = format!("CREATE DATABASE \"{}\"", config.database.name);
    db.execute_unprepared(&create_db_query).await?;

    println!("Database '{}' created successfully.", config.database.name);

    Ok(())
}
