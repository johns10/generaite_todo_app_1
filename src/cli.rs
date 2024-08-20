use clap::{Command, ArgMatches};
use crate::config::Config;
mod create_db;

pub fn cli() -> Command {
    Command::new("gen_todo")
        .about("A Todo application")
        .subcommand(Command::new("version")
            .about("Prints the version of the application"))
        .subcommand(create_db::create_db_command())
}

pub async fn handle_cli(matches: ArgMatches, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("version", _)) => {
            println!("gen_todo version {}", config.version);
        }
        Some(("create-db", _)) => {
            create_db::handle_create_db(config).await?;
        }
        _ => {
            println!("No command specified. Use --help for usage information.");
        }
    }
    Ok(())
}
