# 🐝 BumbleBees - The Game

A retro-styled Space Invaders arcade shooter built in Rust with macroquad. Features authentic Space Invaders movement patterns, C64-inspired visual effects, and comprehensive gameplay with progressive difficulty. Supports both desktop and web (WASM) platforms.

![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows%20%7C%20Web-lightgrey)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Game Engine](https://img.shields.io/badge/engine-macroquad%200.4-blue)

![Screenshot](https://www.marcschieder.de/game/resources/screenshot.png)

## ✨ Features

### 🎮 Core Gameplay
- **4 Enemy Types**:
  - Standard (White): 1 hit, 10 points
  - Fast (Yellow): 1 hit, 1.5x speed, 20 points
  - Tank (Red): 3 hits with health bar, 50 points
  - Swooper (Cyan): 1 hit, 30 points
- **4 Formation Patterns**: Grid, V-shape, Diamond, and Scattered formations that cycle every 4 waves
- **Authentic Space Invaders Movement**: Enemies move as a formation, reverse direction when any enemy hits edge, and descend together
- **Progressive Difficulty**: Enemy types unlock progressively, speed increases per wave
- **Player Upgrades**: Wider ship and more simultaneous shots per wave completed

### 🎨 Retro Visual Effects
- **Custom Pixel Font**: Authentic 8x8 pixel font for highscore display (A-Z, 0-9, symbols)
- **C64-Style Rainbow Effects**: CPU-based per-letter rainbow color cycling with sine wave wobble on menu and game over screens
- **C64-Style Scrolling Highscores**: Top highscores scroll upward like classic Commodore 64 games
- **9-Layer Parallax Background**: Multi-depth scrolling background with sky, clouds, and terrain layers
- **Explosion Animations**: 3-frame stop-motion explosion effects when enemies are destroyed
- **Color-Coded Enemies**: Each enemy type has distinct coloring (White/Yellow/Red/Cyan)
- **Health Bars**: Damaged Tank enemies display health bars
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
- **Desktop**: macOS (with app bundle), Linux, Windows
- **Web**: Full WASM support for browser play
- **Mobile**: iOS touch controls (left/right zones, tap to shoot)

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

#### During Gameplay (Desktop)
- **Left Arrow** (←): Move player left
- **Right Arrow** (→): Move player right
- **Space**: Fire bullets
- **ESC**: Quit game

#### During Gameplay (Touch/Mobile)
- **Touch left side of screen**: Move player left
- **Touch right side of screen**: Move player right
- **Tap anywhere**: Fire bullets

#### Game Over Screen
- **R**: Return to main menu

### Objective

Destroy all enemies before they reach the **defender line** at the bottom of the screen!

- **Standard enemies**: 10 points
- **Fast enemies**: 20 points
- **Tank enemies**: 50 points (requires 3 hits)
- **Swooper enemies**: 30 points
- Complete a wave to advance to the next level
- Formation patterns cycle every 4 waves
- Survive as long as possible to achieve a high score

### Game Mechanics

#### Wave System
- **Wave 1**: Classic grid formation (50 Standard enemies)
- **Wave 2**: V-shape formation (56 enemies, introduces Fast enemies)
- **Wave 3**: Diamond formation (25 enemies, introduces Tank enemies)
- **Wave 4**: Scattered formation (35 enemies, introduces Swooper enemies)
- **Wave 5+**: Patterns repeat with progressively harder enemy mixes

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
- **Enemy Types**:
  - Standard: Basic movement, 1 hit
  - Fast: 1.5x movement speed, 1 hit
  - Tank: Slower (0.7x speed), 3 hits with visible health bar
  - Swooper: Normal speed, 1 hit
- **Progressive Unlocks**: New enemy types appear in later waves
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
cd bumblebees

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
cp target/wasm32-unknown-unknown/release/bumblebees.wasm .

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
- **bumblebees.wasm**: Compiled WebAssembly binary (~850KB optimized)
- **resources/**: All game assets (textures, sounds, fonts)

#### Deploying to a Web Server
To deploy the game to your web server, copy these files:

```
your-webserver/
├── game.html                 # Main game page
├── bumblebees.wasm                  # Build this locally (see above)
└── resources/                # Copy entire directory
    ├── bg_layer_01.png       (parallax layer 1 - sky)
    ├── bg_layer_02.png       (parallax layer 2 - clouds)
    ├── bg_layer_03.png       (parallax layer 3 - far field)
    ├── bg_layer_04.png       (parallax layer 4)
    ├── bg_layer_05.png       (parallax layer 5)
    ├── bg_layer_06.png       (parallax layer 6)
    ├── bg_layer_07.png       (parallax layer 7)
    ├── bg_layer_08.png       (parallax layer 8 - foreground)
    ├── bg_main.png           (main background)
    ├── ui_font.png           (custom pixel font)
    ├── ui_logo.png           (game icon)
    ├── sprite_enemy.png      (enemy sprite)
    ├── vfx_explosion_01.png  (explosion frame 1)
    ├── vfx_explosion_02.png  (explosion frame 2)
    ├── vfx_explosion_03.png  (explosion frame 3)
    ├── sfx_shoot.wav         (shooting sound effect)
    ├── sfx_hit.wav           (hit sound effect)
    ├── intro.ogg             (intro music - OGG Vorbis, 1.8MB)
    └── music_background.ogg  (background music - OGG Vorbis, 2.5MB)
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
bumblebees/
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
├── resources/           # Game assets (follows game dev naming conventions)
│   ├── bg_layer_01-08.png     # 8 parallax background layers (sequential)
│   ├── bg_main.png            # Main background
│   ├── ui_font.png            # Custom pixel font texture
│   ├── ui_logo.png            # Game icon
│   ├── sprite_enemy.png       # Enemy sprite
│   ├── vfx_explosion_01-03.png # Explosion animation frames
│   ├── sfx_shoot.wav          # Shooting sound effect
│   ├── sfx_hit.wav            # Hit sound effect
│   ├── intro.ogg              # Intro music (OGG Vorbis)
│   └── music_background.ogg   # Background music (OGG Vorbis)
├── assets/              # Additional assets
│   ├── icon.icns        # macOS application icon (1024x1024)
│   ├── icon_16x16.png   # Window icon (16x16)
│   ├── icon_32x32.png   # Window icon (32x32)
│   └── icon_64x64.png   # Window icon (64x64)
├── fuzz/                # Fuzzing targets
│   ├── Cargo.toml       # Fuzzing dependencies
│   └── fuzz_targets/    # Fuzz target implementations
│       ├── fuzz_highscore.rs    # CSV parser fuzzer
│       ├── fuzz_asset_paths.rs  # Path resolution fuzzer
│       └── fuzz_icon_decode.rs  # PNG decoder fuzzer
├── Cargo.toml           # Rust dependencies and metadata
├── CLAUDE.md            # Developer documentation
├── game.html            # Web deployment HTML
└── README.md            # This file
```

## 🛠️ Development

### Testing

```bash
# Run all tests (73 unit tests + 2 doc tests)
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test module
cargo test entities::player::tests
```

### Fuzzing

The project includes fuzzing support for security testing:

```bash
# Install cargo-fuzz (requires nightly Rust)
cargo install cargo-fuzz
rustup toolchain install nightly

# List available fuzz targets
cargo fuzz list

# Run highscore CSV parser fuzzer
cargo +nightly fuzz run fuzz_highscore -- -max_total_time=60

# Run asset path resolution fuzzer
cargo +nightly fuzz run fuzz_asset_paths -- -max_total_time=60

# Run icon PNG decoder fuzzer
cargo +nightly fuzz run fuzz_icon_decode -- -max_total_time=60
```

**Fuzz Targets:**
- `fuzz_highscore`: Tests CSV parsing with malformed input
- `fuzz_asset_paths`: Tests path resolution for security vulnerabilities
- `fuzz_icon_decode`: Tests PNG decoding with corrupted data

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
pub const SCREEN_HEIGHT: f32 = 575.0;

// Speed settings
pub const PLAYER_SPEED: f32 = 300.0;
pub const BULLET_SPEED: f32 = 700.0;
pub const INITIAL_ENEMY_SPEED: f32 = 150.0;
pub const BACKGROUND_SCROLL_SPEED: f32 = 50.0;

// Difficulty scaling
pub const SPEED_INCREASE_PER_WAVE: f32 = 20.0;

// Enemy types have individual point values:
// Standard: 10, Fast: 20, Tank: 50, Swooper: 30
```

### Replacing Assets

All assets follow game development naming conventions (category_description_variant.ext).

Place your own assets in the `resources/` directory:
- **Background layers**: `bg_layer_01.png` through `bg_layer_08.png` (8 sequential parallax layers)
- **Main background**: `bg_main.png` (1024x575 PNG)
- **Sprites**: `sprite_enemy.png` (40x40 PNG with transparency)
- **VFX**: `vfx_explosion_01.png` through `vfx_explosion_03.png` (animation frames)
- **UI**: `ui_font.png` (pixel font), `ui_logo.png` (game icon)
- **Audio**: `sfx_*.wav` (sound effects in WAV format), `intro.ogg` and `music_background.ogg` (music in OGG Vorbis format)

## 🐛 Troubleshooting

### Game won't start
- Ensure Rust 1.70+ is installed: `rustc --version`
- Check that `resources/` directory exists and contains all assets
- Run `cargo clean` then `cargo build` to rebuild from scratch

### No sound
- Verify audio files are in `resources/` directory
- Ensure sound effects are in WAV format and music files are in OGG Vorbis format
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
- **Cross-Platform**: Desktop (macOS, Linux, Windows) + Web (WASM) + iOS touch support
- **Build System**: Cargo with conditional compilation for WASM
- **Testing**: Comprehensive unit tests (73 tests passing + 2 doc tests)
- **Fuzzing**: 3 fuzz targets for security testing (requires nightly Rust)
- **Code Quality**: Clippy-clean with full Rustdoc documentation
- **Icons**: Multi-resolution window icons (16x16, 32x32, 64x64) + macOS app icon

## 🔧 Advanced Configuration

### macOS Bundle

The game can be bundled as a native macOS .app with full icon support:

```bash
# Install cargo-bundle
cargo install cargo-bundle

# Create .app bundle (automatically includes resources)
cargo bundle --release

# Bundle will be created at:
# target/release/bundle/osx/BumbleBees.app
```

The bundle configuration is in `Cargo.toml`:
```toml
[package.metadata.bundle]
name = "BumbleBees"
identifier = "com.mcschied.bumblebees"
icon = ["assets/icon.icns"]
resources = ["resources"]
```

**Features:**
- Proper icon display in macOS dock and title bar
- Resources loaded from bundle Contents/Resources directory
- Fallback to Contents/MacOS/resources for compatibility
- Window icons (16x16, 32x32, 64x64) embedded in executable

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
- `game.html` - Web deployment entry point

---

**Enjoy playing BumbleBees - The Game!** 🐝 🎮

Experience authentic Space Invaders gameplay with modern C64-inspired visual effects. Battle enemy formations with progressive difficulty, earn high scores, and enjoy smooth cross-platform gameplay!

For bugs, features, or questions, please open an issue on GitHub.
