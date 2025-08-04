# Glider Kitchen

A Rust application that uses AI to predict recipes based on ingredients selection. The application has both native and web (WASM) targets.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- For native build:
  - Standard Rust toolchain
  - Development libraries for GUI (on Linux: X11 or Wayland development packages)
- For web build:
  - WASM target 
  - [Trunk](https://trunkrs.dev/#install)

## How to build and run the native app

1. Clone the repository:
   ```bash
   git clone https://github.com/audricschiltknecht/glider-kitchen.git
   cd glider-kitchen
   ```

2. Build the application:
   ```bash
   cargo build
   ```

3. Run the application:
   ```bash
   cargo run
   ```

The native application will open in a window with the Glider Kitchen interface.

## How to build and run the web app

1. Clone the repository (if not already done):
   ```bash
   git clone https://github.com/audricschiltknecht/glider-kitchen.git
   cd glider-kitchen
   ```

2. Install target and Trunk if not already installed:
   ```bash
   rustup target add wasm32-unknown-unknown
   cargo install trunk
   ```

3. Serve the web application:
   ```bash
   trunk serve
   ```

4. Open your browser and navigate to `http://127.0.0.1:8080/index.html#dev` (or the port specified by Trunk).

5. To create a release version:
   ```bash
   trunk build --release
   ```
   It will generate a `dist` repository that contains the static HTML website that can be deployed on any web hosting service.

## Project Structure

- `glider-kitchen-ui`: Main UI application crate
- `glider-kitchen-ai`: AI component for recipe prediction
- `config.toml`: Configuration for recipe generation parameters
- `table.toml`: Database of ingredients with their nutritional ratios

## How it works

The application allows users to select ingredients from two categories (fruits and vegetables) and uses an AI component to predict valid recipes based on the configured ratio constraints. The UI is built with egui/eframe and works on both desktop and web platforms.

## Continuous Integration

This project uses GitHub Actions for continuous integration to ensure code quality and functionality.

### CI Workflows

#### Semantic Pull Request

The semantic pull request workflow validates that all PR titles follow the [Conventional Commits](https://www.conventionalcommits.org/) specification. This ensures consistent and meaningful commit messages throughout the project.

PR titles must follow this format:
```
<type>([optional scope]): <description>
```

Where `type` is one of:
- feat: A new feature
- fix: A bug fix
- docs: Documentation changes
- style: Changes that don't affect code meaning (formatting, etc.)
- refactor: Code changes that neither fix bugs nor add features
- perf: Performance improvements
- test: Adding or fixing tests
- build: Changes to build system or dependencies
- ci: Changes to CI configuration
- chore: Other changes that don't modify src or test files
- revert: Reverts a previous commit

Scope can be one of:
- core: Generally, anything related to the glider-kitchen-ai crate
- ui: Any changes related to the UI in the glider-kitchen-ui crate

Example valid PR titles:
- `feat: add user authentication`
- `fix(ui): resolve button alignment issue`
- `docs: update installation instructions`

#### Pre-commit Checks

The pre-commit workflow runs on every pull request to the `main` branch and performs the following checks:

- Trailing whitespace removal
- End-of-file fixing
- YAML and TOML validation
- Large file checks
- Rust formatting (rustfmt)
- Clippy linting

To run these checks locally before committing:

1. Install pre-commit: `pip install pre-commit`
2. Install the hooks: `pre-commit install`
3. Run the hooks manually: `pre-commit run --all-files`

#### Build and Test

The build and test workflow runs on every pull request to the `main` branch and performs:

- Installation of required system dependencies
- Rust toolchain setup
- Building the Rust workspace
- Running all tests

### Local Development

To ensure your changes will pass CI before creating a pull request:

1. Run `pre-commit run --all-files` to check code style and formatting
2. Run `cargo build` to verify the build succeeds
3. Run `cargo test` to ensure all tests pass

