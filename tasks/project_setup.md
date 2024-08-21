# To-Do App Initial Project Setup Tasks

## Environment Setup

### Install Rust
x Download and run rustup
x Verify installation with `rustc --version`

### Install Cargo
x Verify Cargo installation with `cargo --version`

### Set up version control
x Initialize git repository
x Create .gitignore file

### Install additional tools
x Install cargo-watch: `cargo install cargo-watch`
x Install Aider: `pip install aider-chat`

## Project Initialization

### Create new Rust project
x Run `cargo new todo_app`
x Navigate to project directory

### Set up initial project structure
x Create src/models directory
x Create src/services directory
x Create src/repositories directory
x Create src/handlers directory
x Create src/routes directory
x Create src/utils directory
x Create tests directory

Unix

`mkdir -p src/{models,services,repositories,handlers,routes,utils} tests`

Windows

`mkdir src\models src\services src\repositories src\handlers src\routes src\utils tests`

### Initialize README.md
- Add project description

`touch README.md`

## Dependencies Setup

### Add core dependencies to Cargo.toml
x Add axum
x Add tokio with full features
x Add sea-orm
x Add sea-orm-migration

### Add utility dependencies
x Add serde for serialization
x Add chrono for datetime handling
x Add uuid for unique identifiers

### Add development dependencies
x Add tokio-test to dev-dependencies

```
cargo install cargo-edit
cargo add axum tokio --features tokio/full
cargo add sea-orm sea-orm-migration
cargo add serde chrono uuid clap
cargo add --dev tokio-test
```

## Configuration Setup


### Set up environment variables
aider design/architecture.md design/code_standards.md design/tech_stack.md design/folder_structure.md
x Create .env.example file with placeholders for all the variables relevant to this project. Accomodate for test, dev, and production environments.

### Create configuration file

x Create a src/config.rs file and a Config struct. It should use environment variables wherever possible, and manage the dev, test, and prod environment variables. If you delegate any configuration to seperate files, create them now. Do not add anything else to the project at this time, just configuration. https://docs.rs/config/latest/config/

## CLI Setup

x Create a cli module for managing cli commands. It should be started from main. Include a version cli method that returns the version of the project. Set the version variable in the appropriate place in config.

(see bug chat thread)

## Database Setup

### Set up PostgreSQL
x Write a clap script that creates the database, based on the values from config.rs. If necessary, or beneficial, add a cfg(test) section to the config file. Make sure that you don't add any control logic to code, and only use config values. This script should go in a seperate file in the cli directory, and be called from the standard cli file.

## Testing Setup

### Set up test database
x ./target/release/gen_todo create-db

### Create test utilities file
x Write a basic hello world test and place it in an appropriate directory 
x Create shell script for running cargo-watch with appropriate flags https://crates.io/crates/cargo-watch
x Add tests/utils.rs file. This is the purpose of the tests/utils.rs file

## CI/CD Setup

### Set up GitHub Actions
- Create .github/workflows directory
- Add basic CI workflow for running tests and linting

## Documentation Setup

### Set up cargo-doc
x Write a a basic doc comment in main.rs

## Development Workflow Setup

### Set up Aider for AI-assisted development
x Configure Aider for the project

### Set up cargo-watch command
x Review the structure of the project, and configurations. Make improvements in trms of configuration/environment management, readability, and other bells and whistles you can identify.

### Set up Axum

Set up Axum. Use the included documentation. DOn't add any routes or handlers yet. Just do basic setup. Here's the documentation. Don't use Server, it doesn't exist. Use Router https://docs.rs/axum/latest/axum/index.html

