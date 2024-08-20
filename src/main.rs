mod config;

use config::Config;

#[tokio::main]
async fn main() {
    let config = Config::load().expect("Failed to load configuration");
    println!("Loaded configuration: {:?}", config);
    // Use config...
}
