use config::Config;

#[tokio::main]
async fn main() {
    let config = Config::new().expect("Failed to load configuration");
    // Use config...
}
