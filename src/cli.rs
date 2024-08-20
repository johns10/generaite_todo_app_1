use clap::{Command, ArgMatches};
use crate::config::Config;

pub fn cli() -> Command {
    Command::new("gen_todo")
        .about("A Todo application")
        .subcommand(Command::new("version")
            .about("Prints the version of the application"))
}

pub fn handle_cli(matches: ArgMatches, config: &Config) {
    match matches.subcommand() {
        Some(("version", _)) => {
            println!("gen_todo version {}", config.version);
        }
        _ => {
            println!("No command specified. Use --help for usage information.");
        }
    }
}
