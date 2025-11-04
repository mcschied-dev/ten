# AGENTS.md

## Build/Lint/Test Commands

### Building
- `cargo build` - Build the project in debug mode
- `cargo build --release` - Build optimized release version
- `cargo run` - Build and run the game

### Testing
- `cargo test` - Run all tests
- `cargo test -- --nocapture` - Run tests with output visible
- `cargo test <test_name>` - Run a specific test (e.g., `cargo test test_player_clamping_left_boundary`)
- `cargo test <module>::<test_name>` - Run test in specific module (e.g., `cargo test entities::player::tests::test_player_clamping_left_boundary`)

### Code Quality
- `cargo clippy` - Run Clippy linter
- `cargo fmt` - Format code with rustfmt
- `cargo fmt --check` - Check if code is properly formatted

## Code Style Guidelines

### Imports
- Group imports: std, external crates, then local modules
- Use explicit imports over glob imports (`use std::collections::HashMap` not `use std::collections::*`)
- Import macroquad prelude: `use macroquad::prelude::*;`

### Formatting
- Use `cargo fmt` for consistent formatting
- 4-space indentation
- Line length: ~100 characters
- Use rustfmt defaults

### Types and Naming
- Structs: `PascalCase` (e.g., `Player`, `Bullet`)
- Functions: `snake_case` (e.g., `new()`, `update()`, `clamp_position()`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `SCREEN_WIDTH`, `PLAYER_SPEED`)
- Enums: `PascalCase` for type and variants (e.g., `GameState::Playing`)
- Use meaningful names with full words over abbreviations

### Error Handling
- Use `Result<T, E>` for fallible operations
- Prefer `unwrap()` only in tests or when certain the operation won't fail
- Use `expect("message")` with descriptive error messages
- Handle errors appropriately rather than panicking

### Documentation
- All public APIs must have `///` documentation comments
- Include parameter descriptions with `# Arguments`
- Add examples in doc tests where appropriate
- Module-level docs with `//!`

### Code Structure
- Separate concerns: entities (data), systems (logic), main game loop
- Use `#[must_use]` on constructor functions
- Implement `Default` where appropriate
- Keep functions focused on single responsibilities
- Use `const fn` where possible

### Testing
- Unit tests co-located with implementation in `#[cfg(test)]` modules
- Test public APIs thoroughly
- Use descriptive test names: `test_<function>_<condition>`
- Test edge cases and boundary conditions
- Pure functions in systems/ are easily testable without game context

### Logging
- Use `log` crate with appropriate levels:
  - `log::info!` for major events
  - `log::debug!` for detailed state changes
  - `log::warn!` for important warnings
- Include relevant context in log messages
- Log at function entry/exit for complex operations

### Constants
- All game constants in `src/constants.rs`
- Use descriptive names
- Group related constants together
- Document units and purpose</content>
<parameter name="filePath">AGENTS.md