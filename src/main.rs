mod config;
mod cli;

use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load().expect("Failed to load configuration");
    
    let cli = cli::cli();
    let matches = cli.get_matches();

    cli::handle_cli(matches, &config).await?;

    Ok(())
}
