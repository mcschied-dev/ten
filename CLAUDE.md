# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Space Invaders clone game built in Rust using the ggez game engine. The game features dynamic difficulty scaling, background music, sound effects, and a scrolling text background. It's packaged as "Hummel" for macOS distribution.

The project follows a modular architecture with clear separation of concerns between entities, systems, game state, and rendering.

## Build and Run Commands

### Development
```bash
# Run the game
cargo run

# Run with verbose output
cargo run --verbose

# Build without running
cargo build

# Build optimized release version
cargo build --release
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Run tests in a specific module
cargo test entities::player::tests
```

### Code Quality
```bash
# Run clipper for linting
cargo clippy

# Format code
cargo fmt
```

## Architecture

### Module Structure

The codebase is organized into a clean modular structure:

```
src/
├── main.rs           # Entry point, initializes game context
├── lib.rs            # Library root, exports all public modules
├── constants.rs      # Game constants (speeds, dimensions, etc.)
├── entities/         # Game entities
│   ├── mod.rs
│   ├── bullet.rs     # Bullet entity with update logic
│   ├── enemy.rs      # Enemy entity with movement and collision
│   └── player.rs     # Player entity with input handling
├── systems/          # Game systems (pure logic)
│   ├── mod.rs
│   ├── collision.rs  # Collision detection between bullets and enemies
│   └── wave.rs       # Wave generation logic
├── game_state.rs     # MainState struct, GameState enum, EventHandler
└── rendering.rs      # All drawing logic separated from game logic
```

### Key Design Principles

1. **Separation of Concerns**: Each module has a single responsibility
   - Entities know their own state and basic behavior
   - Systems contain pure logic functions (collision detection, wave generation)
   - GameState manages overall game state and coordinates updates
   - Rendering is completely separated from game logic

2. **Testability**: All game logic is unit tested
   - Each module contains its own tests
   - Tests are co-located with implementation
   - Pure functions in systems/ are easily testable

3. **Entity Encapsulation**:
   - `Player` (src/entities/player.rs:7): Manages position, width, shots
   - `Enemy` (src/entities/enemy.rs:3): Knows position and boundary checking
   - `Bullet` (src/entities/bullet.rs:3): Handles its own movement

### Game Loop Flow

The game loop is implemented via `EventHandler` in src/game_state.rs:181:

1. **Update** (src/game_state.rs:182):
   - Handle player input (keyboard)
   - Update scroll text position
   - Update bullets (movement, out-of-bounds removal)
   - Update enemies (movement, edge detection, downward movement)
   - Process collisions (bullets vs enemies)
   - Check if wave is complete

2. **Draw** (src/game_state.rs:216):
   - Delegates to rendering module
   - Draws background, UI, entities, and game-over screen

3. **Input** (src/game_state.rs:220):
   - Space: Shoot bullets
   - R: Reset game
   - Arrow keys: Move player (handled in update)

### Key Systems

**Wave Generation** (src/systems/wave.rs:5):
- Function: `generate_wave(wave: u32) -> Vec<Enemy>`
- Rows increase with wave: `rows = 2 + wave`
- Always 10 columns
- Position calculation: `x: 50.0 + i * 60.0`, `y: 100.0 + j * 50.0`

**Collision Detection** (src/systems/collision.rs:5):
- Function: `check_collision(bullet: &Bullet, enemy: &Enemy) -> bool`
- Circle-based collision with radius from constants
- `process_collisions()` removes hit enemies and returns count

**Player Progression** (src/entities/player.rs:45):
- `upgrade()` method called each wave
- Increases shots and base width
- Constants defined in src/constants.rs

### Resource Management

Resources are loaded in `MainState::new()` (src/game_state.rs:35):
- Images: background.png, enemy.png
- Audio: shoot.wav, hit.wav, background_music.wav
- Mounted from `resources/` directory via ggez filesystem

Resource paths use leading `/` (e.g., `/background.png`) which maps to `resources/background.png`.

### Game State Management

**GameState Enum** (src/game_state.rs:12):
- `Playing`: Normal gameplay
- `GameOver`: Enemies breached defender line

**MainState Methods**:
- `new()`: Initialize game with resources
- `reset()`: Reset to wave 1
- `shoot()`: Create bullets based on player shots
- `update_bullets()`: Move bullets and remove out-of-bounds
- `update_enemies()`: Handle enemy movement and edge detection
- `update_collisions()`: Process bullet-enemy collisions
- `check_wave_complete()`: Spawn next wave if enemies cleared

### Constants

Located in src/constants.rs:
```rust
SCREEN_WIDTH: 1024.0
SCREEN_HEIGHT: 768.0
PLAYER_SPEED: 300.0
BULLET_SPEED: 700.0
INITIAL_ENEMY_SPEED: 150.0
DEFENDER_LINE: 100.0
TEXT_SCROLL_SPEED: 100.0
COLLISION_RADIUS: 20.0
POINTS_PER_ENEMY: 10
SPEED_INCREASE_PER_WAVE: 20.0
BASE_WIDTH_INCREASE: 20.0
```

## Common Development Patterns

### Adding a New Entity

1. Create new file in `src/entities/`
2. Define struct with public fields
3. Implement methods for behavior
4. Add unit tests in the same file
5. Export from `src/entities/mod.rs`

### Adding a New System

1. Create new file in `src/systems/`
2. Implement pure functions (no side effects)
3. Add comprehensive unit tests
4. Export from `src/systems/mod.rs`

### Modifying Game Logic

- Update methods are in `MainState` (src/game_state.rs)
- Keep update logic in MainState methods
- Extract complex logic to systems/ if reusable

## Testing Strategy

- **Unit tests**: Each module tests its own functionality
- **Location**: Tests are in `#[cfg(test)]` modules at the bottom of each file
- **Coverage**: 19 tests covering entities, systems, and game logic
- **No integration tests**: Game logic doesn't require ggez Context for testing

## Common Gotchas

1. **Resource loading**: Resources must exist in `resources/` directory relative to `CARGO_MANIFEST_DIR`
2. **Audio format**: Sound files must be WAV format for ggez compatibility
3. **Coordinate system**: Origin (0,0) is top-left; Y increases downward
4. **Thread-safe scrolling**: Text position uses `Arc<Mutex<f32>>` for thread safety
5. **Enemy movement flag**: The `moved_down` flag (src/game_state.rs:26) prevents repeated downward movement at edges—critical for correct behavior
6. **Orphan rules**: EventHandler must be implemented in the same crate as MainState (src/game_state.rs), not in main.rs

## Bundling for macOS

The `Cargo.toml` includes bundle metadata:
- App name: "Hummel"
- Bundle identifier: `com.mcschied.hummel`
- Icon: `assets/icon.icns`
- Resources directory bundled automatically
