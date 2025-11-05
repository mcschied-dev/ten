# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is **BumbleBees**, a Space Invaders clone game built in Rust using the ggez game engine. The game features:
- Progressive difficulty with wave-based gameplay
- Parallax scrolling background
- Comprehensive highscore tracking system with persistent storage
- Interactive menu with player name input
- Sound effects and background music
- Comprehensive logging system for debugging

The project follows a clean modular architecture with clear separation of concerns between entities, systems, game state, highscore management, logging, and rendering.

## Build and Run Commands

### Development
```bash
# Run the game (logging to console and debug.log)
cargo run

# Build without running
cargo build

# Build optimized release version
cargo build --release
```

### Testing
```bash
# Run all tests (includes 22 unit tests + doc tests)
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Run tests in a specific module
cargo test entities::player::tests

# Run doc tests only
cargo test --doc
```

### Code Quality
```bash
# Run clippy for linting
cargo clippy

# Format code
cargo fmt

# Generate and view documentation
cargo doc --open
```

### Debugging
```bash
# View real-time logs
tail -f debug.log

# Search logs for errors
grep -i "error\|warn" debug.log

# View game startup sequence
head -50 debug.log
```

## Architecture

### Module Structure

The codebase is organized into a clean modular structure with full Rustdoc documentation:

```
src/
├── main.rs           # Entry point, initializes logging and game context
├── lib.rs            # Library root, exports all public modules
├── constants.rs      # Game constants (speeds, dimensions, etc.)
├── logger.rs         # Logging configuration (console + file output)
├── entities/         # Game entities
│   ├── mod.rs
│   ├── bullet.rs     # Bullet entity with update logic
│   ├── enemy.rs      # Enemy entity with movement and collision
│   └── player.rs     # Player entity with input handling
├── systems/          # Game systems (pure logic)
│   ├── mod.rs
│   ├── collision.rs  # Collision detection between bullets and enemies
│   └── wave.rs       # Wave generation logic
├── highscore.rs      # Highscore management with file I/O
├── game_state.rs     # MainState struct, GameState enum, EventHandler
└── rendering.rs      # All drawing logic separated from game logic
```

### Key Design Principles

1. **Separation of Concerns**: Each module has a single responsibility
   - Entities know their own state and basic behavior
   - Systems contain pure logic functions (collision detection, wave generation)
   - GameState manages overall game state and coordinates updates
   - Rendering is completely separated from game logic
   - Logging is centralized and configurable

2. **Documentation**: All public APIs have Rustdoc comments with:
   - Module-level documentation (`//!`)
   - Function/struct documentation (`///`)
   - Examples where appropriate
   - Error documentation

3. **Logging**: Comprehensive logging throughout the codebase:
   - INFO level: Major game events (wave complete, game start/end)
   - DEBUG level: Detailed state changes (bullet creation, enemy movement)
   - WARN level: Important warnings (game over conditions)
   - Logs written to both console (INFO+) and debug.log (DEBUG+)

4. **Testability**: All game logic is unit tested
   - Each module contains its own tests
   - Tests are co-located with implementation
   - Pure functions in systems/ are easily testable
   - 22 unit tests + doc tests

5. **Rust Best Practices**:
   - `#[must_use]` on constructor functions
   - `const fn` where possible
   - Proper error handling with `Result` types
   - Clear ownership and borrowing patterns
   - `Default` trait implementations

### Game Loop Flow

The game loop is implemented via `EventHandler` in src/game_state.rs:231:

1. **Update** (src/game_state.rs:232):
   - State-based logic (Menu/Playing/GameOver)
   - Handle player input (keyboard/mouse)
   - Update scroll text and background
   - Update bullets (movement, out-of-bounds removal)
   - Update enemies (movement, edge detection, downward movement)
   - Process collisions (bullets vs enemies)
   - Check if wave is complete

2. **Draw** (src/game_state.rs:277):
   - Delegates to rendering module
   - State-specific rendering (menu/game/game-over)

3. **Input** (src/game_state.rs:281):
   - Menu: Text input for player name, Enter/Click to start
   - Playing: Space to shoot, Arrows to move
   - Game Over: 'R' to return to menu

### Logging System

**Location**: src/logger.rs

The logging system uses `fern` and `log` crates for flexible, multi-output logging:

- **Console Output**: INFO level and above, formatted with timestamp and level
- **File Output**: DEBUG level and above to `debug.log`, includes module and line number
- **Format**: `[timestamp][level][module:line] message`

**Key logged events**:
- Game initialization and resource loading
- Wave generation with enemy counts
- Player actions (shooting, upgrades)
- Collision detection results
- Game state transitions
- Error conditions

**Usage in code**:
```rust
log::info!("Major game event");
log::debug!("Detailed state information");
log::warn!("Important warning");
```

### Highscore System

**Location**: src/highscore.rs

- **File**: `highscores.txt` (format: `name, score`)
- **Features**:
  - Persistent storage across sessions
  - Automatic sorting (highest to lowest)
  - Top 10 display on menu
  - Save on game over
  - Thread-safe file I/O

### Key Systems

**Wave Generation** (src/systems/wave.rs:26):
- Function: `generate_wave(wave: u32) -> Vec<Enemy>`
- Rows increase with wave: `rows = 2 + wave`
- Always 10 columns
- Pre-allocates vector with capacity
- Logs wave information

**Collision Detection** (src/systems/collision.rs:17):
- Function: `check_collision(bullet: &Bullet, enemy: &Enemy) -> bool`
- Circle-based collision with radius from constants
- `process_collisions()` removes hit enemies and returns count
- Logs collision results

**Player Progression** (src/entities/player.rs:78):
- `upgrade()` method called each wave
- Increases shots and base width
- Logs upgrade information

### Resource Management

Resources are loaded in `MainState::new()` (src/game_state.rs:65):
- Images: background.png (with logged dimensions), enemy.png
- Audio: shoot.wav, hit.wav, background_music.wav (looped)
- Mounted from `resources/` directory via ggez filesystem
- All resource loading is logged

Resource paths use leading `/` (e.g., `/background.png`) which maps to `resources/background.png`.

### Game State Management

**GameState Enum** (src/game_state.rs:20):
- `Menu`: Main menu with highscores and name input
- `Playing`: Normal gameplay
- `GameOver`: Game over screen with final score

**MainState Methods** (documented in src/game_state.rs):
- `new()`: Initialize game with resources (logged)
- `reset()`: Reset to menu (logged)
- `start_game()`: Transition from menu to playing (logged)
- `shoot()`: Create bullets based on player shots
- `update_bullets()`: Move bullets and remove out-of-bounds
- `update_enemies()`: Handle enemy movement and edge detection (logs breaches, saves highscore on game over)
- `update_collisions()`: Process bullet-enemy collisions
- `check_wave_complete()`: Spawn next wave if enemies cleared (logged)
- `return_to_menu()`: Return to menu (highscore already saved on game over)

### Constants

Located in src/constants.rs (all documented):
```rust
SCREEN_WIDTH: 1024.0
SCREEN_HEIGHT: 768.0
PLAYER_SPEED: 300.0
BULLET_SPEED: 700.0
INITIAL_ENEMY_SPEED: 150.0
DEFENDER_LINE: 100.0
TEXT_SCROLL_SPEED: 100.0
BACKGROUND_SCROLL_SPEED: 50.0
COLLISION_RADIUS: 20.0
POINTS_PER_ENEMY: 10
SPEED_INCREASE_PER_WAVE: 20.0
BASE_WIDTH_INCREASE: 20.0
```

## Common Development Patterns

### Adding a New Entity

1. Create new file in `src/entities/`
2. Add module documentation (`//!`)
3. Define struct with doc comments (`///`)
4. Implement methods with documentation
5. Add unit tests in the same file
6. Export from `src/entities/mod.rs`
7. Add appropriate logging

### Adding a New System

1. Create new file in `src/systems/`
2. Add module documentation
3. Implement pure functions (no side effects) with doc comments
4. Add `#[must_use]` where appropriate
5. Add comprehensive unit tests
6. Export from `src/systems/mod.rs`
7. Add logging for important events

### Adding Logging

```rust
// At module top
use log::{debug, info, warn, error};

// In code
log::info!("User started game: {}", player_name);
log::debug!("Created {} enemies at wave {}", count, wave);
log::warn!("Player health low: {}", health);
```

### Modifying Game Logic

- Update methods are in `MainState` (src/game_state.rs)
- Keep update logic in MainState methods
- Extract complex logic to systems/ if reusable
- Add appropriate logging
- Update tests if behavior changes

## Testing Strategy

- **Unit tests**: Each module tests its own functionality
- **Location**: Tests are in `#[cfg(test)]` modules at the bottom of each file
- **Coverage**: 22 unit tests + doc tests covering entities, systems, highscore, and game logic
- **Doc tests**: Example code in documentation is tested
- **No integration tests**: Game logic doesn't require ggez Context for testing

## Common Gotchas

1. **Resource loading**: Resources must exist in `resources/` directory relative to `CARGO_MANIFEST_DIR`
2. **Audio format**: Sound files must be WAV format for ggez compatibility
3. **Coordinate system**: Origin (0,0) is top-left; Y increases downward
4. **Thread-safe scrolling**: Text position uses `Arc<Mutex<f32>>` for thread safety
5. **Enemy movement flag**: The `moved_down` flag (src/game_state.rs:29) prevents repeated downward movement at edges—critical for correct behavior
6. **Orphan rules**: EventHandler must be implemented in the same crate as MainState (src/game_state.rs), not in main.rs
7. **Logging initialization**: Must call `init_logger()` before any logging (done in main.rs:16-18)
8. **Debug log size**: debug.log can grow large; consider rotating or clearing periodically

## File Management

### Generated Files
- `debug.log`: Debug logging output (DEBUG level and above)
- `highscores.txt`: Persistent highscore storage
- `test_*.txt`: Created by unit tests, automatically cleaned up
- `target/`: Build artifacts (ignored by git)

These files are ignored in `.gitignore` except for the highscores.

## Documentation

All modules have comprehensive Rustdoc documentation:

```bash
# Generate and view HTML documentation
cargo doc --open

# Check documentation coverage
cargo doc --document-private-items
```

Documentation includes:
- Module-level overview (`//!`)
- Struct/enum/function documentation (`///`)
- Parameter and return value descriptions
- Examples with doc tests
- Error conditions

## Bundling for macOS

The `Cargo.toml` includes bundle metadata:
- App name: "Hummel" (legacy name, game is "BumbleBees")
- Bundle identifier: `com.mcschied.hummel`
- Icon: `assets/icon.icns`
- Resources directory bundled automatically

## GitHub Repository Management

### Branch Protection

The master branch should be protected to prevent accidental force pushes or deletion:

1. Go to repository **Settings** → **Branches**
2. Click **Add rule** under "Branch protection rules"
3. Set "Branch name pattern" to: `master`
4. Recommended settings for solo development:
   - ☑️ **Lock branch** - Prevents force pushes and deletion
   - ☑️ **Do not allow bypassing the above settings**
5. Optional settings for collaborative development:
   - ☑️ **Require a pull request before merging**
   - ☑️ **Require status checks to pass before merging** (if CI/CD is configured)
   - ☑️ **Require conversation resolution before merging**
   - ☑️ **Include administrators**

**Note**: Branch protection rules must be configured through GitHub's web interface, not via git commands.

### Repository Best Practices

The repository follows these best practices:
- **No binary files**: Users build WASM locally (ten.wasm in .gitignore)
- **No user data**: highscores.txt excluded from version control
- **No security reports**: sbom.json and related files generated locally
- **No IDE files**: .vscode, .idea, .claude excluded
- **No system files**: .DS_Store excluded
- **Comprehensive .gitignore**: Organized by category (build artifacts, IDE files, runtime files, etc.)
- **Clean commit history**: Descriptive commit messages with co-author attribution
