# 🐝 BumbleBees - The Game

A retro-styled Space Invaders arcade shooter built in Rust with macroquad. Features authentic Space Invaders movement patterns, C64-inspired visual effects, and comprehensive gameplay with progressive difficulty. Supports both desktop and web (WASM) platforms.

![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows%20%7C%20Web-lightgrey)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Game Engine](https://img.shields.io/badge/engine-macroquad%200.4-blue)

## ✨ Features

### 🎮 Core Gameplay
- **Authentic Space Invaders Movement**: Enemies move as a formation, reverse direction when any enemy hits edge, and descend together
- **Centered Enemy Formations**: Enemy waves appear perfectly centered at the top of the screen
- **Progressive Difficulty**: Each wave adds more enemy rows and increases speed
- **Player Upgrades**: Wider ship and more simultaneous shots per wave completed

### 🎨 Retro Visual Effects
- **Custom Pixel Font**: Authentic 8x8 pixel font for highscore display (A-Z, 0-9, symbols)
- **C64-Style Scrolling Highscores**: Top highscores scroll upward like classic Commodore 64 games
- **9-Layer Parallax Background**: Multi-depth scrolling background with sky, clouds, and terrain layers
- **Explosion Animations**: 3-frame stop-motion explosion effects when enemies are destroyed
- **Red Bold Score Display**: Prominent red score text with shadow effect during gameplay
- **Enhanced Intro Screen**: Game icon, repositioned highscore display, and improved layout
- **Classic Arcade Aesthetics**: Retro-styled graphics and animations

### 🏆 Game Systems
- **Highscore System**:
  - Persistent storage across game sessions
  - Unlimited leaderboard with C64-style upward scrolling animation
  - Custom pixel font rendering for authentic retro appearance
  - Automatic score saving on game over
- **Interactive Menu**:
  - Enter your name before playing
  - View highscores from previous sessions
  - Click or press Enter to start
- **Audio**: Background music and sound effects for shooting and hits
- **Comprehensive Logging**: Debug logging system for troubleshooting

### 🌐 Cross-Platform Support
- **Desktop**: macOS, Linux, Windows
- **Web**: Full WASM support for browser play

## 🎮 How to Play

### Starting the Game

1. Launch the game
2. You'll see the **BumbleBees** main menu with:
    - **Parallax scrolling background** with 9 animated layers
    - **Game icon** prominently displayed
    - **"HIGH SCORES"** header in custom pixel font
    - **Scrolling highscore list** with C64-style upward animation
    - Name input field
    - Start button

3. **Enter your name** in the input field (alphanumeric characters, max 20 chars)
4. Press **Enter** or click the **START GAME** button

### Controls

#### Main Menu
- **Type**: Enter your player name (desktop only)
- **Backspace**: Delete characters
- **Enter**: Start the game
- **Space**: Start the game (recommended for WASM)
- **Mouse Click**: Click the START GAME button

#### During Gameplay
- **Left Arrow** (←): Move player left
- **Right Arrow** (→): Move player right
- **Space**: Fire bullets
- **ESC**: Quit game (desktop only)

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
- **Authentic Space Invaders Movement**: Enemies move as a unified formation
- **Formation Movement**: When ANY enemy reaches a screen edge, the entire wave:
  - Reverses direction
  - Moves down 40 pixels smoothly
- **Centered Formations**: Enemy waves start perfectly centered at the top
- **Alternating Directions**: Row 0 moves right, Row 1 moves left, Row 2 moves right, etc.
- **Game Over**: If any enemy crosses the defender line → **GAME OVER**

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

### Web (WASM) Deployment

The game fully supports WebAssembly for browser-based play with all features intact!

#### Prerequisites
```bash
# Ensure you're using rustup (not Homebrew) for proper WASM support
rustup --version

# Add wasm32 target
rustup target add wasm32-unknown-unknown
```

#### Building for Web
```bash
# Build optimized WASM binary
export PATH="$HOME/.cargo/bin:$PATH"  # Ensure rustup's cargo is used
cargo build --release --target wasm32-unknown-unknown

# Copy WASM file to web root
cp target/wasm32-unknown-unknown/release/ten.wasm .

# The resources directory must be accessible from web root
# (Already included in the repository)
```

#### Serving the Game
```bash
# Start a local HTTP server
python3 -m http.server 8000

# Open in browser
# Visit: http://localhost:8000/game.html
```

#### Web-Specific Features
- **Demo Highscores**: Since LocalStorage requires wasm-bindgen (adds complexity), the WASM version shows 10 demo highscores
- **In-Memory Scores**: Your score during a session is tracked but not persisted across browser reloads
- **Canvas Focus**: The canvas automatically focuses on page load to capture keyboard/mouse input
- **Keyboard Controls**: Press **Space** to start the game from the menu
- **Full Gameplay**: All game mechanics work identically to the desktop version

#### WASM Files Included
- **game.html**: Main game page with miniquad loader
- **ten.wasm**: Compiled WebAssembly binary (~850KB optimized)
- **resources/**: All game assets (textures, sounds, fonts)

#### Deploying to a Web Server
To deploy the game to your web server, copy these files:

```
your-webserver/
├── game.html              # Main game page
├── ten.wasm              # Build this locally (see above)
└── resources/            # Copy entire directory
    ├── 1.png - 10.png        (background layers)
    ├── background.png        (main background)
    ├── custom_font.png       (pixel font)
    ├── enemy.png            (enemy sprite)
    ├── explosion1-3.png     (explosion frames)
    ├── hummel_icns_temp.png (game icon)
    ├── shoot.wav, hit.wav   (sound effects)
    └── background_music.wav (optional - 32MB)
```

**Note:** The WASM file and security reports are not in the repo. Build them locally with the commands above.

#### Technical Notes
- Uses **macroquad 0.4** with built-in WASM support (no wasm-bindgen needed)
- **miniquad** provides the underlying WebGL rendering
- Canvas size: 1024x575 (optimized for web)
- **getrandom** with "js" feature for WASM-compatible random numbers
- Fallback textures for missing resources
- No passwords or API keys in codebase

## 📁 Project Structure

```
ten/
├── src/
│   ├── main.rs          # Entry point and game loop
│   ├── lib.rs           # Library exports
│   ├── constants.rs     # Game constants and configuration
│   ├── entities/        # Game entities
│   │   ├── mod.rs       # Entity module exports
│   │   ├── player.rs    # Player entity and logic
│   │   ├── enemy.rs     # Enemy entity and logic
│   │   └── bullet.rs    # Bullet entity and logic
│   ├── systems/         # Game systems
│   │   ├── mod.rs       # System module exports
│   │   ├── collision.rs # Collision detection
│   │   └── wave.rs      # Enemy wave generation
│   ├── highscore.rs     # Highscore persistence system
│   └── entities.rs      # Entity re-exports (legacy)
├── resources/           # Game assets
│   ├── 1.png through 10.png  # Parallax background layers
│   ├── custom_font.png       # Custom pixel font texture
│   ├── enemy.png              # Enemy sprite
│   ├── hummel_icns_temp.png   # Game icon
│   ├── shoot.wav              # Shooting sound effect
│   ├── hit.wav                # Hit sound effect
│   ├── background_music.wav
│   └── bg.png                 # Additional background assets
├── assets/              # Additional assets
│   └── icon.icns        # macOS application icon
├── Cargo.toml           # Rust dependencies and metadata
├── CLAUDE.md            # Developer documentation
├── AGENTS.md            # Build/lint/test commands reference
├── index.html           # Web deployment HTML
├── wasm-status.html     # WASM status page
└── README.md            # This file
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
- **Game Engine**: macroquad 0.4 (WASM-compatible 2D graphics)
- **Audio**: macroquad audio system (WAV format)
- **Graphics**: OpenGL/Metal/Vulkan via wgpu
- **Cross-Platform**: Desktop (macOS, Linux, Windows) + Web (WASM)
- **Build System**: Cargo with conditional compilation for WASM
- **Testing**: Comprehensive unit tests (39 tests passing)
- **Code Quality**: Clippy-clean with generated documentation

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
- C64-style visual effects inspired by Commodore 64 demos
- Audio and graphics assets created for this project
- Space Invaders movement patterns based on original arcade behavior

## 📚 Additional Resources

- [CLAUDE.md](CLAUDE.md) - Comprehensive developer documentation
- [Cargo.toml](Cargo.toml) - Rust dependencies and project metadata
- `debug.log` - Detailed runtime logs for debugging
- `index.html` - Web deployment entry point
- `wasm-status.html` - WASM build status page

---

**Enjoy playing BumbleBees - The Game!** 🐝 🎮

Experience authentic Space Invaders gameplay with modern C64-inspired visual effects. Battle enemy formations with progressive difficulty, earn high scores, and enjoy smooth cross-platform gameplay!

For bugs, features, or questions, please open an issue on GitHub.
