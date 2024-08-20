mod config;
mod cli;

use config::Config;
use clap::Command;

#[tokio::main]
async fn main() {
    let config = Config::load().expect("Failed to load configuration");
    
    let cli = cli::cli();
    let matches = cli.get_matches();

    cli::handle_cli(matches, &config);
}
