# AGENTS.md

## Project Overview

This is a Rust project for AI agent testing and experimentation. Follow the guidelines below when adding functionality.

---

## Build, Lint, and Test Commands

### Rust (Cargo)

```bash
# Build the project
cargo build

# Run the project
cargo run

# Run tests
cargo test                    # Run all tests
cargo test -- <test_name>     # Run a single test
cargo test --lib              # Run only library tests
cargo test --doc              # Run only doc tests

# Lint and format
cargo clippy                  # Run linter (install: rustup component add clippy)
cargo fmt                     # Format code
cargo fmt -- --check          # Check formatting without modifying

# Build release
cargo build --release

# Documentation
cargo doc                     # Generate docs
cargo doc --open              # Generate and open docs
```

### Additional Commands

```bash
# Run all checks before committing
cargo fmt && cargo clippy && cargo test

# Clean build artifacts
cargo clean

# Update dependencies
cargo update
```

---

## Code Style Guidelines

### General Principles

- **Clarity over cleverness**: Write code that is easy to understand and maintain
- **Explicit over implicit**: Prefer explicit types and behavior
- **Small, focused functions**: Each function should do one thing well
- **Early returns**: Use early returns to reduce nesting

### Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Modules | `snake_case` | `my_module` |
| Structs | `PascalCase` | `MyStruct` |
| Enums | `PascalCase` | `MyEnum` |
| Functions | `snake_case` | `my_function` |
| Variables | `snake_case` | `my_variable` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_SIZE` |
| Type parameters | `PascalCase` | `T` |
| Traits | `PascalCase` | `MyTrait` |

### Imports

Order imports in groups:
1. Standard library (`std`)
2. External crates
3. Local modules (`crate::`)

```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::module::MyStruct;
```

### Error Handling

- Use `Result<T, E>` for fallible operations
- Use `anyhow::Result<()>` for application-level errors
- Use `thiserror` for library errors with custom error types
- Always handle `Result` values or propagate with `?`
- Add context with `.context()` when propagating errors

```rust
fn read_config() -> anyhow::Result<Config> {
    let contents = fs::read_to_string("config.toml")
        .context("Failed to read config file")?;
    toml::from_str(&contents).context("Failed to parse config")
}
```

### Types

- Use strong typing to prevent invalid states
- Prefer `&str` over `&String` in function signatures
- Use `&[T]` over `&Vec<T>` when you don't need ownership
- Use `impl Trait` for return types when concrete type is not important

### Formatting Rules

- Line length: 100 characters maximum
- Indentation: 4 spaces
- No trailing whitespace
- Use `rustfmt.toml` for project-specific rules

---

## Testing Guidelines

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_panic_case() {
        let v = vec![1, 2, 3];
        v.get(10);
    }
}
```

### Test Naming

- Test names: `test_<what_is_being_tested>`
- Group related tests: `test_parsing_`, `test_validation_`

---

## Git Workflow

### Commit Messages

Follow conventional commits:
```
feat: add user authentication
fix: handle null response in API client
docs: update README with new commands
refactor: extract validation logic
test: add integration tests
chore: upgrade dependencies
```

### Branch Naming

- Feature branches: `feature/<description>`
- Bug fixes: `fix/<description>`
- Documentation: `docs/<description>`

---

## Working with AI Assistants

When working in this codebase:

1. **Understand before modifying**: Read relevant files and understand existing patterns
2. **Run tests after changes**: Always verify changes don't break existing functionality
3. **Check formatting**: Run `cargo fmt` before committing
4. **Run clippy**: Address all warnings from `cargo clippy`
5. **Update docs**: If adding public APIs, document them appropriately
6. **Type safety**: Leverage Rust's type system to prevent runtime errors
