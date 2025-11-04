# 🐝 BumbleBees

A classic Space Invaders-style arcade shooter built in Rust with the macroquad game engine. Battle against increasingly difficult waves of enemies with a dynamic parallax scrolling background and comprehensive highscore tracking. Supports both desktop and web (WASM) platforms.

![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

## ✨ Features

- **Progressive Difficulty**: Each wave brings more enemies and faster movement
- **Dynamic Gameplay**:
  - Player's firepower increases with each wave cleared
  - Enemy speed increases progressively
  - Wider player base to accommodate more simultaneous shots
- **Parallax Scrolling Background**: Smooth right-to-left scrolling creates depth
- **Highscore System**:
  - Persistent storage across game sessions
  - Top 10 leaderboard displayed on main menu
  - Automatic score saving on game over
- **Interactive Menu**:
  - Enter your name before playing
  - View highscores from previous sessions
  - Click or press Enter to start
- **Audio**: Background music and sound effects for shooting and hits
- **Comprehensive Logging**: Debug logging system for troubleshooting

## 🎮 How to Play

### Starting the Game

1. Launch the game
2. You'll see the **BumbleBees** main menu with:
   - Game title in golden text
   - Top 10 highscores (if any exist)
   - Name input field
   - Start button

3. **Enter your name** in the input field (alphanumeric characters, max 20 chars)
4. Press **Enter** or click the **START GAME** button

### Controls

#### Main Menu
- **Type**: Enter your player name
- **Backspace**: Delete characters
- **Enter**: Start the game
- **Mouse Click**: Click the START GAME button

#### During Gameplay
- **Left Arrow** (←): Move player left
- **Right Arrow** (→): Move player right
- **Space**: Fire bullets
- **ESC**: Quit game

#### Game Over Screen
- **R**: Return to main menu

### Objective

Destroy all enemies before they reach the **defender line** at the bottom of the screen!

- Each enemy destroyed = **10 points**
- Complete a wave to advance to the next level
- Each wave adds more enemy rows
- Survive as long as possible to achieve a high score

### Game Mechanics

#### Wave System
- **Wave 1**: 3 rows of 10 enemies (30 total)
- **Wave 2**: 4 rows of 10 enemies (40 total)
- **Wave 3**: 5 rows of 10 enemies (50 total)
- And so on...

#### Player Upgrades
After completing each wave, you gain:
- **+1 simultaneous shot** (Wave 1: 1 shot, Wave 2: 2 shots, etc.)
- **+20 pixels** wider base
- Bullets are evenly spaced across your base width

#### Enemy Behavior
- Enemies move horizontally across the screen
- When reaching a screen edge, they:
  - Reverse direction
  - Move down 40 pixels
- If any enemy crosses the defender line → **GAME OVER**

#### Difficulty Scaling
- Enemy speed increases by **20 pixels/second** per wave
- Wave 1: 150 px/s
- Wave 2: 170 px/s
- Wave 3: 190 px/s
- And so on...

## 🚀 Installation & Setup

### Prerequisites

- **Rust** 1.70 or higher ([Install Rust](https://www.rust-lang.org/tools/install))
- **Git** (to clone the repository)

### Quick Start

```bash
# Clone the repository
git clone <repository-url>
cd ten

# Run the game
cargo run --release
```

### Building from Source

```bash
# Development build (faster compile, slower runtime)
cargo build

# Release build (slower compile, optimized runtime)
cargo build --release

# Run directly
cargo run          # Development
cargo run --release # Release (recommended for gameplay)
```

## 📁 Project Structure

```
ten/
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Library exports
│   ├── constants.rs     # Game constants
│   ├── logger.rs        # Logging system
│   ├── entities/        # Game entities (Player, Enemy, Bullet)
│   ├── systems/         # Game systems (collision, wave generation)
│   ├── game_state.rs    # Core game loop and state management
│   ├── highscore.rs     # Highscore persistence
│   └── rendering.rs     # All rendering logic
├── resources/           # Game assets
│   ├── background.png   # Parallax background
│   ├── enemy.png        # Enemy sprite
│   ├── shoot.wav        # Shooting sound effect
│   ├── hit.wav          # Hit sound effect
│   └── background_music.wav
├── Cargo.toml          # Rust dependencies
├── CLAUDE.md           # Developer documentation
└── README.md           # This file
```

## 🛠️ Development

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test module
cargo test entities::player::tests
```

### Code Quality

```bash
# Run linter
cargo clippy

# Format code
cargo fmt

# Generate documentation
cargo doc --open
```

### Debugging

The game writes detailed logs to `debug.log`:

```bash
# View real-time logs
tail -f debug.log

# Search for errors
grep -i "error\|warn" debug.log
```

Log levels:
- **Console**: INFO and above
- **debug.log**: DEBUG and above (includes module and line numbers)

## 🎯 Game Tips

1. **Stay Mobile**: Keep moving to avoid enemy formations
2. **Manage Firepower**: More shots per wave means more chances to hit
3. **Watch the Edges**: Enemies drop down when they hit screen edges
4. **Defender Line**: Keep enemies away from the bottom 100 pixels
5. **Early Waves**: Take your time in early waves to practice
6. **Later Waves**: Focus on enemies closest to the defender line

## 📊 Highscores

Highscores are automatically saved to `highscores.txt` in the game directory.

File format:
```
PlayerName, Score
Alice, 1500
Bob, 1200
Charlie, 900
```

The file is automatically created on first game over and updated with each new score.

## 🎨 Customization

### Adjusting Game Balance

Edit constants in `src/constants.rs`:

```rust
// Screen dimensions
pub const SCREEN_WIDTH: f32 = 1024.0;
pub const SCREEN_HEIGHT: f32 = 768.0;

// Speed settings
pub const PLAYER_SPEED: f32 = 300.0;
pub const BULLET_SPEED: f32 = 700.0;
pub const INITIAL_ENEMY_SPEED: f32 = 150.0;
pub const BACKGROUND_SCROLL_SPEED: f32 = 50.0;

// Difficulty scaling
pub const SPEED_INCREASE_PER_WAVE: f32 = 20.0;
pub const POINTS_PER_ENEMY: u32 = 10;
```

### Replacing Assets

Place your own assets in the `resources/` directory:
- **background.png**: 1024x768 PNG
- **enemy.png**: 40x40 PNG with transparency
- **Audio files**: WAV format (supported by macroquad)

## 🐛 Troubleshooting

### Game won't start
- Ensure Rust 1.70+ is installed: `rustc --version`
- Check that `resources/` directory exists and contains all assets
- Run `cargo clean` then `cargo build` to rebuild from scratch

### No sound
- Verify audio files are in `resources/` directory
- Ensure files are WAV format (not MP3 or other formats)
- Check system audio settings

### Low frame rate
- Build with `--release` flag: `cargo run --release`
- Close other applications
- Check `debug.log` for performance warnings

### Highscores not saving
- Ensure write permissions in game directory
- Check `debug.log` for file I/O errors
- Verify `highscores.txt` is not read-only

## 📝 Technical Details

- **Language**: Rust (Edition 2021)
- **Game Engine**: macroquad 0.4
- **Audio**: macroquad audio system
- **Logging**: log + fern + chrono
- **Graphics**: OpenGL/Metal via wgpu
- **Supported Platforms**: macOS, Linux, Windows

## 🔧 Advanced Configuration

### macOS Bundle

The game can be bundled as a macOS .app:

```bash
# Install cargo-bundle
cargo install cargo-bundle

# Create .app bundle
cargo bundle --release
```

The bundle configuration is in `Cargo.toml`:
```toml
[package.metadata.bundle]
name = "Hummel"
identifier = "com.mcschied.hummel"
icon = ["assets/icon.icns"]
resources = ["resources"]
```

## 📄 License

This project is available under the MIT License.

## 👤 Author

**@mcschied**

## 🙏 Acknowledgments

- Built with [macroquad](https://macroquad.rs/) - A Rust library for making 2D games with WASM support
- Inspired by the classic Space Invaders arcade game
- Audio and graphics assets created for this project

## 📚 Additional Resources

- [CLAUDE.md](CLAUDE.md) - Comprehensive developer documentation
- [Cargo.toml](Cargo.toml) - Rust dependencies and project metadata
- `debug.log` - Detailed runtime logs for debugging

---

**Enjoy playing BumbleBees!** 🐝 🎮

For bugs, features, or questions, please open an issue or contact @mcschied.
