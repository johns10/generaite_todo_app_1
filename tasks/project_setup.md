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

### Create configuration file
/web https://docs.rs/config/latest/config/
Create the src/config.rs file and add placeholder for Config struct 

### Set up environment variables
- Create .env file with a placeholder for database URL in .env and placeholder for server port in .env

## Database Setup

### Set up PostgreSQL
- Create todo_app database (write a clap script to do this)

## Testing Setup

### Set up test database
- Create test database in PostgreSQL (write a clap script to do this)

### Create test utilities file
- Add tests/utils.rs file

## CI/CD Setup

### Set up GitHub Actions
- Create .github/workflows directory
- Add basic CI workflow for running tests and linting

## Documentation Setup

### Set up cargo-doc
- Ensure main.rs has a basic doc comment

## Development Workflow Setup

### Set up Aider for AI-assisted development
- Configure Aider for the project

### Set up cargo-watch command
- Create shell script for running cargo-watch with appropriate flags