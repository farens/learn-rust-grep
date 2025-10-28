# Agent Guidelines for learn-rust-grep

## Build, Lint, and Test Commands

- Build: `cargo build` or `cargo b`
- Check (fast compile check): `cargo check` or `cargo c`
- Run: `cargo run -- <args>`
- Test all: `cargo test` or `cargo t`
- Test single: `cargo test test_name`
- Lint: `cargo clippy`
- Format: `cargo fmt`
- Format check: `cargo fmt -- --check`

## Code Style

- Edition: Rust 2024
- Imports: Use `use` statements at top of file, group std, external crates, then local modules
- Formatting: Follow `rustfmt` defaults (run `cargo fmt` before committing)
- Naming: snake_case for functions/variables, CamelCase for types, SCREAMING_SNAKE_CASE for constants
- Error handling: Use `Result<T, E>` for fallible operations, `?` operator for propagation
- Types: Prefer explicit type annotations for function signatures, let inference handle locals
- Documentation: Use `///` for public items, `//` for internal comments
- Testing: Place unit tests in `#[cfg(test)]` modules within source files, integration tests in `tests/` directory
