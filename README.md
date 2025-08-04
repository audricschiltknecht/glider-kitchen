# Glider Kitchen

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
- ui: Any changes related to the UI in the  glider-kitchen crate

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
