//! BumbleBees - Space Invaders-style arcade shooter
//! Macroquad edition with WASM support

use macroquad::audio::{
    load_sound, play_sound, play_sound_once, stop_sound, PlaySoundParams, Sound,
};
use macroquad::prelude::*;
use macroquad::texture::DrawTextureParams;

mod constants;
mod entities;
mod highscore;
mod systems;

use constants::*;
use entities::{Bullet, Enemy, Explosion, Player};
use highscore::HighscoreManager;
use systems::{generate_wave, process_collisions};

/// Load texture with fallback paths for bundle compatibility
async fn load_texture_fallback(path: &str) -> Result<Texture2D, macroquad::Error> {
    // For WASM builds, just try the path directly
    #[cfg(target_arch = "wasm32")]
    {
        return load_texture(path).await;
    }

    // For desktop builds, try fallback paths
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Try the path as-is first
        match load_texture(path).await {
            Ok(texture) => return Ok(texture),
            Err(_) => {
                // If we're in a bundle, try relative to executable
                if let Ok(exe_path) = std::env::current_exe() {
                    if let Some(exe_dir) = exe_path.parent() {
                        // Try relative to executable directory
                        let exe_relative = exe_dir.join(path);
                        if exe_relative.exists() {
                            if let Some(path_str) = exe_relative.to_str() {
                                return load_texture(path_str).await;
                            }
                        }

                        // Try in bundle Resources directory
                        if exe_dir.ends_with("MacOS") {
                            if let Some(contents) = exe_dir.parent() {
                                let resources_path = contents.join("Resources").join(path);
                                if resources_path.exists() {
                                    if let Some(path_str) = resources_path.to_str() {
                                        return load_texture(path_str).await;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Final fallback - return error
        load_texture(path).await
    }
}

/// Load sound with fallback paths for bundle compatibility
async fn load_sound_fallback(path: &str) -> Result<Sound, macroquad::Error> {
    // For WASM builds, just try the path directly
    #[cfg(target_arch = "wasm32")]
    {
        return load_sound(path).await;
    }

    // For desktop builds, try fallback paths
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Try the path as-is first
        match load_sound(path).await {
            Ok(sound) => return Ok(sound),
            Err(_) => {
                // If we're in a bundle, try relative to executable
                if let Ok(exe_path) = std::env::current_exe() {
                    if let Some(exe_dir) = exe_path.parent() {
                        // Try relative to executable directory
                        let exe_relative = exe_dir.join(path);
                        if exe_relative.exists() {
                            if let Some(path_str) = exe_relative.to_str() {
                                return load_sound(path_str).await;
                            }
                        }

                        // Try in bundle Resources directory
                        if exe_dir.ends_with("MacOS") {
                            if let Some(contents) = exe_dir.parent() {
                                let resources_path = contents.join("Resources").join(path);
                                if resources_path.exists() {
                                    if let Some(path_str) = resources_path.to_str() {
                                        return load_sound(path_str).await;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Final fallback - return error
        load_sound(path).await
    }
}

/// Load TTF font with fallback paths for bundle compatibility
async fn load_font_fallback(path: &str) -> Option<Font> {
    // For WASM builds, try to load directly
    #[cfg(target_arch = "wasm32")]
    {
        if let Ok(bytes) = load_file(path).await {
            if let Ok(font) = load_ttf_font_from_bytes(&bytes) {
                return Some(font);
            }
        }
        return None;
    }

    // For desktop builds, try fallback paths
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::fs;

        // Try the path as-is first
        if let Ok(bytes) = fs::read(path) {
            if let Ok(font) = load_ttf_font_from_bytes(&bytes) {
                return Some(font);
            }
        }

        // If we're in a bundle, try relative to executable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // Try relative to executable directory
                let exe_relative = exe_dir.join(path);
                if exe_relative.exists() {
                    if let Ok(bytes) = fs::read(&exe_relative) {
                        if let Ok(font) = load_ttf_font_from_bytes(&bytes) {
                            return Some(font);
                        }
                    }
                }

                // Try in bundle Resources directory (macOS)
                if exe_dir.ends_with("MacOS") {
                    if let Some(contents) = exe_dir.parent() {
                        let resources_path = contents.join("Resources").join(path);
                        if resources_path.exists() {
                            if let Ok(bytes) = fs::read(&resources_path) {
                                if let Ok(font) = load_ttf_font_from_bytes(&bytes) {
                                    return Some(font);
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    Menu,
    Playing,
    GameOver,
}

/// Represents a single parallax background layer with infinite scrolling.
///
/// Each layer maintains two texture positions to create seamless scrolling.
/// When one texture scrolls off-screen, it's repositioned behind the other
/// to create an infinite loop effect.
///
/// # Fields
///
/// - `speed`: Scroll speed in pixels/second (negative = left, positive = right, 0 = static)
/// - `parts`: Two X positions for the dual-texture infinite scroll technique
/// - `layer_type`: Identifies which texture to use for this layer
///
/// # Examples
///
/// ```
/// # use ten::*; // This would need proper module structure
/// // Create a slow-moving cloud layer scrolling left at 20 px/s
/// // let clouds = BackgroundLayer::new(-20.0, 1024.0, BackgroundLayerType::Clouds);
/// ```
#[derive(Debug, Clone)]
struct BackgroundLayer {
    /// Scroll speed in pixels per second (negative = left, positive = right)
    speed: f32,
    /// Two positions for seamless infinite scrolling
    parts: [f32; 2],
    /// Layer type for texture selection
    layer_type: BackgroundLayerType,
}

/// Enum representing the different parallax background layers.
///
/// Layers are numbered sequentially from 01-08 in the resources directory
/// using the naming convention: `bg_layer_01.png` through `bg_layer_08.png`,
/// plus `bg_main.png` for the main background field.
///
/// # Layer Mapping
///
/// - `Sky` -> `bg_layer_01.png` (static sky, no scrolling)
/// - `Clouds` -> `bg_layer_02.png` (slow-moving clouds)
/// - `FarField` -> `bg_layer_03.png` (medium-speed far field)
/// - `Layer4` -> `bg_layer_04.png` (medium-slow layer)
/// - `Layer5` -> `bg_layer_05.png` (medium-fast layer)
/// - `Layer6` -> `bg_layer_06.png` (very fast layer)
/// - `Layer7` -> `bg_layer_07.png` (fastest layer)
/// - `Layer8` -> `bg_layer_08.png` (very slow foreground layer)
/// - `NearField` -> `bg_main.png` (fast near-field main background)
///
/// Layers are rendered from back to front to create the parallax effect.
#[derive(Debug, Clone, Copy)]
enum BackgroundLayerType {
    Sky,
    Layer4,
    Layer5,
    Layer6,
    Layer7,
    Layer8,
    Clouds,
    FarField,
    NearField,
}

impl BackgroundLayer {
    /// Create a new background layer.
    ///
    /// # Arguments
    ///
    /// * `speed` - Scroll speed in pixels per second (negative scrolls left, positive scrolls right, 0 is static)
    /// * `texture_width` - Width of the texture in pixels
    /// * `layer_type` - Type of layer for texture selection
    ///
    /// # Returns
    ///
    /// A new `BackgroundLayer` with two texture positions for seamless scrolling
    #[must_use]
    fn new(speed: f32, texture_width: f32, layer_type: BackgroundLayerType) -> Self {
        Self {
            speed,
            parts: [0.0, texture_width],
            layer_type,
        }
    }

    /// Update the layer's scroll position.
    ///
    /// Moves both texture parts based on speed and delta time. When a texture
    /// scrolls completely off-screen, it's repositioned behind the other for
    /// seamless infinite scrolling.
    ///
    /// # Arguments
    ///
    /// * `dt` - Delta time in seconds since last update
    /// * `texture_width` - Width of the texture (for wraparound calculation)
    fn update(&mut self, dt: f32, texture_width: f32) {
        // Move both parts
        self.parts[0] += self.speed * dt;
        self.parts[1] += self.speed * dt;

        // If first part has moved off-screen to the left, reposition it behind the second part
        if self.parts[1] < 0.0 {
            self.parts[0] = self.parts[1] + texture_width;
            // Swap parts so the repositioned one becomes the second
            self.parts.swap(0, 1);
        }
    }

    /// Reset the layer to its initial position.
    ///
    /// # Arguments
    ///
    /// * `texture_width` - Width of the texture in pixels
    fn reset(&mut self, texture_width: f32) {
        self.parts = [0.0, texture_width];
    }
}

struct Game {
    player: Player,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    explosions: Vec<Explosion>,
    enemy_speed: f32,
    descent_speed: f32,
    descent_distance: f32, // how much enemies need to descend
    wave_number: u32,
    state: GameState,
    score: u32,

    // Player and highscore
    player_name: String,
    highscore_manager: HighscoreManager,

    // UI elements
    // scroll_text_x: Arc<Mutex<f32>>, // Commented out - removed wobbling BumbleBee text
    // scroll_direction: Arc<Mutex<f32>>, // Commented out - removed wobbling BumbleBee text
    // scroll_text_time: f32, // Time accumulator for wobble effect // Commented out - removed wobbling BumbleBee text
    highscore_scroll_offset: f32, // For scrolling highscore list animation
    background_layers: Vec<BackgroundLayer>,

    // Resources
    sky: Texture2D,
    clouds: Texture2D,
    far_field: Texture2D,
    near_field: Texture2D,
    layer_4: Texture2D,
    layer_5: Texture2D,
    layer_6: Texture2D,
    layer_7: Texture2D,
    layer_8: Texture2D,
    intro_icon: Texture2D,
    custom_font: Texture2D,
    enemy_image: Texture2D,
    explosion_frame1: Texture2D,
    explosion_frame2: Texture2D,
    explosion_frame3: Texture2D,

    // Font
    retro_font: Option<Font>,

    // Audio
    shoot_sound: Option<Sound>,
    hit_sound: Option<Sound>,
    background_music: Option<Sound>,
}

impl Game {
    async fn new() -> Self {
        log::info!("Loading game resources");

        // ========================================================================
        // RESOURCE NAMING CONVENTION
        // ========================================================================
        // All resources follow game development naming conventions:
        //
        // - Category prefixes: bg_ (backgrounds), sprite_ (sprites), vfx_ (effects),
        //   ui_ (interface), sfx_ (sound effects), music_ (music)
        // - Snake_case with zero-padded sequential numbering (01, 02, 03...)
        // - Examples:
        //     bg_layer_01.png    (parallax layer 1 - sky)
        //     bg_layer_08.png    (parallax layer 8 - foreground)
        //     sprite_enemy.png   (enemy sprite)
        //     vfx_explosion_01.png (first explosion frame)
        //     ui_logo.png        (main logo)
        //     sfx_shoot.wav      (shooting sound)
        //     music_background.wav (background music)
        //
        // This convention makes asset management clearer and follows industry
        // standards for game development asset organization.
        // ========================================================================

        let sky = load_texture_fallback("resources/bg_layer_01.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[135, 206, 235, 255])); // Sky blue fallback

        let clouds = load_texture_fallback("resources/bg_layer_02.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[255, 255, 255, 255]));

        let far_field = load_texture_fallback("resources/bg_layer_03.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[34, 139, 34, 255])); // Forest green fallback

        let near_field = load_texture_fallback("resources/bg_main.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[0, 100, 0, 255])); // Dark green fallback

        let layer_4 = load_texture_fallback("resources/bg_layer_04.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[150, 150, 150, 255]));

        let layer_5 = load_texture_fallback("resources/bg_layer_05.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[150, 150, 150, 255]));

        let layer_6 = load_texture_fallback("resources/bg_layer_06.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[150, 150, 150, 255]));

        let layer_7 = load_texture_fallback("resources/bg_layer_07.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[150, 150, 150, 255]));

        let layer_8 = load_texture_fallback("resources/bg_layer_08.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[150, 150, 150, 255]));

        let intro_icon = load_texture_fallback("resources/ui_logo.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[200, 200, 200, 255])); // Light gray fallback

        let custom_font = load_texture_fallback("resources/ui_font.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[255, 255, 255, 255])); // White fallback

        let enemy_image = load_texture_fallback("resources/sprite_enemy.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[255, 255, 255, 255]));

        // Load explosion animation frames (3 frames for stop-motion effect)
        let explosion_frame1 = load_texture_fallback("resources/vfx_explosion_01.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(20, 20, &[255, 100, 0, 255])); // Orange fallback

        let explosion_frame2 = load_texture_fallback("resources/vfx_explosion_02.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(24, 24, &[255, 150, 0, 255])); // Brighter orange fallback

        let explosion_frame3 = load_texture_fallback("resources/vfx_explosion_03.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(28, 28, &[255, 200, 100, 255])); // Yellow fallback

        // Load TTF font for retro gaming style
        let retro_font = load_font_fallback("resources/font_retro_gaming.ttf").await;
        if retro_font.is_some() {
            log::info!("Retro Gaming font loaded successfully");
        } else {
            log::warn!("Failed to load Retro Gaming font, using default font");
        }

        let shoot_sound = load_sound_fallback("resources/sfx_shoot.wav").await.ok();

        let hit_sound = load_sound_fallback("resources/sfx_hit.wav").await.ok();

        let background_music = load_sound_fallback("resources/music_background.wav")
            .await
            .ok();

        // Initialize background layers for parallax scrolling (9 layers: 8 numbered + main bg)
        // Layers are ordered from back to front for proper rendering depth
        let background_layers = vec![
            BackgroundLayer::new(0.0, sky.width(), BackgroundLayerType::Sky), // Static sky (layer 1)
            BackgroundLayer::new(-10.0, layer_8.width(), BackgroundLayerType::Layer8), // Very slow layer 8
            BackgroundLayer::new(-20.0, clouds.width(), BackgroundLayerType::Clouds), // Slow clouds (layer 2)
            BackgroundLayer::new(-50.0, layer_4.width(), BackgroundLayerType::Layer4), // Medium-slow layer 4
            BackgroundLayer::new(-100.0, far_field.width(), BackgroundLayerType::FarField), // Medium far-field (layer 3)
            BackgroundLayer::new(-200.0, layer_5.width(), BackgroundLayerType::Layer5), // Medium-fast layer 5
            BackgroundLayer::new(-300.0, near_field.width(), BackgroundLayerType::NearField), // Fast near-field (main bg)
            BackgroundLayer::new(-400.0, layer_6.width(), BackgroundLayerType::Layer6), // Very fast layer 6
            BackgroundLayer::new(-500.0, layer_7.width(), BackgroundLayerType::Layer7), // Fastest layer 7
        ];

        log::info!("Game state created successfully");

        Self {
            player: Player::new(),
            bullets: Vec::new(),
            enemies: generate_wave(1),
            explosions: Vec::new(),
            enemy_speed: INITIAL_ENEMY_SPEED,
            descent_speed: 100.0,  // pixels per second for controlled descent
            descent_distance: 0.0, // current descent progress
            wave_number: 1,
            state: GameState::Menu,
            score: 0,
            player_name: String::new(),
            highscore_manager: HighscoreManager::new("highscores.txt"),
            // scroll_text_x: Arc::new(Mutex::new(SCREEN_WIDTH)), // Commented out - removed wobbling BumbleBee text
            // scroll_direction: Arc::new(Mutex::new(-1.0)), // Commented out - removed wobbling BumbleBee text
            // scroll_text_time: 0.0, // Commented out - removed wobbling BumbleBee text
            highscore_scroll_offset: 0.0,
            background_layers,
            sky,
            clouds,
            far_field,
            near_field,
            layer_4,
            layer_5,
            layer_6,
            layer_7,
            layer_8,
            intro_icon,
            custom_font,
            enemy_image,
            explosion_frame1,
            explosion_frame2,
            explosion_frame3,
            retro_font,
            shoot_sound,
            hit_sound,
            background_music,
        }
    }

    fn reset(&mut self) {
        log::info!("Resetting game to menu");
        // Stop background music
        if let Some(ref sound) = self.background_music {
            stop_sound(sound);
        }
        self.player.reset();
        self.bullets.clear();
        self.enemies = generate_wave(1);
        self.enemy_speed = INITIAL_ENEMY_SPEED;
        self.descent_speed = 100.0;
        self.descent_distance = 0.0;
        self.wave_number = 1;
        self.score = 0;
        self.state = GameState::Menu;
        for layer in &mut self.background_layers {
            let texture_width = match layer.layer_type {
                BackgroundLayerType::Sky => self.sky.width(),
                BackgroundLayerType::Layer4 => self.layer_4.width(),
                BackgroundLayerType::Layer5 => self.layer_5.width(),
                BackgroundLayerType::Layer6 => self.layer_6.width(),
                BackgroundLayerType::Layer7 => self.layer_7.width(),
                BackgroundLayerType::Layer8 => self.layer_8.width(),
                BackgroundLayerType::Clouds => self.clouds.width(),
                BackgroundLayerType::FarField => self.far_field.width(),
                BackgroundLayerType::NearField => self.near_field.width(),
            };
            layer.reset(texture_width);
        }
        self.player_name.clear();

        // let mut text_x = self.scroll_text_x.lock().unwrap(); // Commented out - removed wobbling BumbleBee text
        // *text_x = SCREEN_WIDTH; // Commented out - removed wobbling BumbleBee text
        // self.scroll_text_time = 0.0; // Commented out - removed wobbling BumbleBee text
    }

    fn start_game(&mut self) {
        if !self.player_name.is_empty() {
            log::info!("Starting game for player: {}", self.player_name);
            self.state = GameState::Playing;
            self.score = 0;
            self.wave_number = 1;
            self.enemies = generate_wave(1);
            self.bullets.clear();
            self.player.reset();
            self.enemy_speed = INITIAL_ENEMY_SPEED;
            self.descent_speed = 100.0;
            self.descent_distance = 0.0;
            // Start background music
            if let Some(ref sound) = self.background_music {
                play_sound(
                    sound,
                    PlaySoundParams {
                        looped: true,
                        volume: 0.5,
                    },
                );
            }
        } else {
            log::warn!("Cannot start game without player name");
        }
    }

    /// Draw text with the custom retro font, or fallback to default font
    fn draw_text_retro(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        if let Some(ref font) = self.retro_font {
            draw_text_ex(
                text,
                x,
                y,
                TextParams {
                    font: Some(font),
                    font_size: font_size as u16,
                    color,
                    ..Default::default()
                },
            );
        } else {
            // Fallback to default font
            draw_text(text, x, y, font_size, color);
        }
    }

    /// Measure text dimensions with the custom retro font
    fn measure_text_retro(&self, text: &str, font_size: u16) -> TextDimensions {
        if let Some(ref font) = self.retro_font {
            measure_text(text, Some(font), font_size, 1.0)
        } else {
            measure_text(text, None, font_size, 1.0)
        }
    }

    fn shoot(&mut self) {
        if matches!(self.state, GameState::Playing) {
            let new_bullets = self.player.shoot();
            if !new_bullets.is_empty() {
                if let Some(ref sound) = self.shoot_sound {
                    play_sound_once(sound);
                }
            }
            self.bullets.extend(new_bullets);
        }
    }

    fn update_bullets(&mut self, dt: f32) {
        for bullet in &mut self.bullets {
            bullet.update(dt);
        }
        self.bullets.retain(|bullet| !bullet.is_out_of_bounds());
    }

    fn update_enemies(&mut self, dt: f32) {
        // Handle gradual descent if active
        if self.descent_distance > 0.0 {
            let descent_this_frame = self.descent_speed * dt;
            if descent_this_frame >= self.descent_distance {
                // Complete the descent
                for enemy in &mut self.enemies {
                    enemy.y += self.descent_distance;
                }
                self.descent_distance = 0.0;
            } else {
                // Continue descending
                for enemy in &mut self.enemies {
                    enemy.y += descent_this_frame;
                }
                self.descent_distance -= descent_this_frame;
            }
        } else {
            // Normal horizontal movement when not descending
            for enemy in &mut self.enemies {
                enemy.update(self.enemy_speed, dt);
            }

            // Check if any enemy has reached the edge it's moving toward
            let mut edge_reached = false;
            for enemy in &self.enemies {
                let moving_right = enemy.direction > 0.0;
                let moving_left = enemy.direction < 0.0;

                if (moving_right && enemy.x >= SCREEN_WIDTH - 20.0)
                    || (moving_left && enemy.x <= 20.0)
                {
                    edge_reached = true;
                    break;
                }
            }

            if edge_reached {
                log::info!("Enemy reached edge - reversing direction and starting descent");

                // Reverse ALL directions
                for enemy in &mut self.enemies {
                    enemy.direction *= -1.0;
                    // Move back into bounds
                    if enemy.x < 20.0 {
                        enemy.x = 20.0;
                    } else if enemy.x > SCREEN_WIDTH - 20.0 {
                        enemy.x = SCREEN_WIDTH - 20.0;
                    }
                }

                // Start controlled descent for the entire wave
                self.descent_distance = 40.0;
            }
        }

        // Check if any enemy has breached the defender line
        for enemy in &self.enemies {
            if enemy.has_breached_defender_line() {
                log::warn!("Enemy breached defender line at y={}, game over!", enemy.y);
                self.state = GameState::GameOver;
                // Save highscore immediately when game over
                if !self.player_name.is_empty() && self.score > 0 {
                    log::info!("Game over! Final score: {}", self.score);
                    self.highscore_manager
                        .save_highscore(&self.player_name, self.score);
                }
                return;
            }
        }
    }

    fn update_explosions(&mut self, dt: f32) {
        // Update all explosions
        for explosion in &mut self.explosions {
            explosion.update(dt);
        }

        // Remove finished explosions
        self.explosions.retain(|explosion| !explosion.is_finished());
    }

    fn update_collisions(&mut self) {
        let destroyed_positions = process_collisions(&mut self.enemies, &self.bullets);

        if !destroyed_positions.is_empty() {
            // Play hit sound
            if let Some(ref sound) = self.hit_sound {
                play_sound_once(sound);
            }

            // Update score
            self.score += destroyed_positions.len() as u32 * POINTS_PER_ENEMY;

            // Create explosion at each destroyed enemy position
            for (x, y) in destroyed_positions {
                self.explosions.push(Explosion::new(x, y));
                log::debug!("Created explosion at ({}, {})", x, y);
            }
        }
    }

    fn check_wave_complete(&mut self) {
        if self.enemies.is_empty() {
            self.wave_number += 1;
            self.enemy_speed += SPEED_INCREASE_PER_WAVE;
            self.player.upgrade();
            self.enemies = generate_wave(self.wave_number);
            log::info!(
                "Wave {} complete! Starting wave {} with speed {}",
                self.wave_number - 1,
                self.wave_number,
                self.enemy_speed
            );
        }
    }

    /*
    fn update_scroll_text(&mut self, dt: f32) {
        let mut position = self.scroll_text_x.lock().unwrap();
        let mut direction = self.scroll_direction.lock().unwrap();

        *position += *direction * TEXT_SCROLL_SPEED * dt;

        if *position <= 0.0 && *direction < 0.0 {
            *direction = 1.0;
        } else if *position >= SCREEN_WIDTH && *direction > 0.0 {
            *direction = -1.0;
        }

        // Update wobble time accumulator
        self.scroll_text_time += dt;
    }
    */

    fn update_highscore_scroll(&mut self, dt: f32) {
        // Scroll highscore list slowly upward (like C64 games)
        self.highscore_scroll_offset -= 20.0 * dt; // 20 pixels per second

        // Reset scroll when it goes too far (allows for many high scores)
        if self.highscore_scroll_offset < -2000.0 {
            // Enough for ~60+ high scores
            self.highscore_scroll_offset = 50.0; // Reset with some padding
        }
    }

    fn update_background_scroll(&mut self, dt: f32) {
        // Update all background layers
        for layer in &mut self.background_layers {
            let texture_width = match layer.layer_type {
                BackgroundLayerType::Sky => self.sky.width(),
                BackgroundLayerType::Layer4 => self.layer_4.width(),
                BackgroundLayerType::Layer5 => self.layer_5.width(),
                BackgroundLayerType::Layer6 => self.layer_6.width(),
                BackgroundLayerType::Layer7 => self.layer_7.width(),
                BackgroundLayerType::Layer8 => self.layer_8.width(),
                BackgroundLayerType::Clouds => self.clouds.width(),
                BackgroundLayerType::FarField => self.far_field.width(),
                BackgroundLayerType::NearField => self.near_field.width(),
            };
            layer.update(dt, texture_width);
        }
    }

    fn update(&mut self, dt: f32) {
        match self.state {
            GameState::Menu => {
                self.update_background_scroll(dt);
                self.update_highscore_scroll(dt);
            }
            GameState::Playing => {
                // Handle player input
                if is_key_down(KeyCode::Left) {
                    self.player.move_left(dt);
                }
                if is_key_down(KeyCode::Right) {
                    self.player.move_right(dt);
                }

                // Update scrolling background
                self.update_background_scroll(dt);

                // Update scrolling text
                // self.update_scroll_text(dt); // Commented out - removed wobbling BumbleBee text

                // Update bullets
                self.update_bullets(dt);

                // Update enemies
                self.update_enemies(dt);

                // Update explosions
                self.update_explosions(dt);

                // Process collisions
                self.update_collisions();

                // Check if wave is complete
                self.check_wave_complete();
            }
            GameState::GameOver => {
                self.update_background_scroll(dt);
            }
        }
    }

    fn draw(&self) {
        clear_background(BLACK);

        match self.state {
            GameState::Menu => {
                self.draw_menu();
            }
            GameState::Playing => {
                self.draw_background();
                // self.draw_scroll_text(); // Commented out - removed wobbling BumbleBee text
                self.draw_player();
                self.draw_bullets();
                self.draw_enemies();
                self.draw_explosions();
                self.draw_score();
            }
            GameState::GameOver => {
                self.draw_background();
                self.draw_game_over();
            }
        }
    }

    fn draw_background(&self) {
        // Draw all background layers (from back to front for proper layering)
        for layer in &self.background_layers {
            let texture = match layer.layer_type {
                BackgroundLayerType::Sky => &self.sky,
                BackgroundLayerType::Layer4 => &self.layer_4,
                BackgroundLayerType::Layer5 => &self.layer_5,
                BackgroundLayerType::Layer6 => &self.layer_6,
                BackgroundLayerType::Layer7 => &self.layer_7,
                BackgroundLayerType::Layer8 => &self.layer_8,
                BackgroundLayerType::Clouds => &self.clouds,
                BackgroundLayerType::FarField => &self.far_field,
                BackgroundLayerType::NearField => &self.near_field,
            };

            // Only scroll layers that have speed > 0 (non-static layers)
            if layer.speed != 0.0 {
                for &x_pos in &layer.parts {
                    draw_texture(texture, x_pos, 0.0, WHITE);
                }
            } else {
                // Static sky layer - just draw once
                draw_texture(texture, 0.0, 0.0, WHITE);
            }
        }
    }

    /*
    fn draw_scroll_text(&self) {
        let text_x = *self.scroll_text_x.lock().unwrap();

        // Add C64-style wobble effect using sine wave
        let wobble_amplitude = 8.0; // How much the text moves up and down
        let wobble_frequency = 3.0; // How fast the wobble oscillates
        let wobble_offset = (self.scroll_text_time * wobble_frequency).sin() * wobble_amplitude;

        let text_y = 50.0 + wobble_offset;

        // C64-style blinking effect (blink every 0.5 seconds)
        let blink_visible = (self.scroll_text_time * 2.0).sin() > 0.0;

        // C64-style rainbow color cycling
        let color_cycle = (self.scroll_text_time * 1.5).sin() * 0.5 + 0.5; // 0.0 to 1.0
        let color_index = (color_cycle * 7.0) as i32;

        let text_color = match color_index {
            0 => Color::from_rgba(255, 0, 0, 255),     // Red
            1 => Color::from_rgba(255, 165, 0, 255),   // Orange
            2 => Color::from_rgba(255, 255, 0, 255),   // Yellow
            3 => Color::from_rgba(0, 255, 0, 255),     // Green
            4 => Color::from_rgba(0, 0, 255, 255),     // Blue
            5 => Color::from_rgba(75, 0, 130, 255),    // Indigo
            _ => Color::from_rgba(238, 130, 238, 255), // Violet
        };

        // Large bitmap-style text (increased size and add shadow for bitmap effect)
        let font_size = 80.0;

        // Draw shadow for bitmap effect
        if blink_visible {
            draw_text("BumbleBee - The Game", text_x + 2.0, text_y + 2.0, font_size, Color::from_rgba(0, 0, 0, 128));
            draw_text("BumbleBee - The Game", text_x, text_y, font_size, text_color);
        }
    }
    */

    fn draw_player(&self) {
        let player_x = self.player.x - self.player.base_width / 2.0;
        let player_y = self.player.y();
        let player_color = Color::from_rgba(0, 128, 0, 255);

        draw_rectangle(
            player_x,
            player_y,
            self.player.base_width,
            self.player.height(),
            player_color,
        );
    }

    fn draw_bullets(&self) {
        for bullet in &self.bullets {
            draw_rectangle(bullet.x - 5.0, bullet.y - 10.0, 10.0, 20.0, WHITE);
        }
    }

    fn draw_enemies(&self) {
        for enemy in &self.enemies {
            draw_texture(&self.enemy_image, enemy.x - 20.0, enemy.y - 20.0, WHITE);
        }
    }

    fn draw_explosions(&self) {
        for explosion in &self.explosions {
            // Select the appropriate frame texture based on current frame
            let texture = match explosion.current_frame {
                0 => &self.explosion_frame1,
                1 => &self.explosion_frame2,
                2 => &self.explosion_frame3,
                _ => continue, // Skip if beyond frames (shouldn't happen)
            };

            // Draw explosion centered at the position
            let half_width = texture.width() / 2.0;
            let half_height = texture.height() / 2.0;
            draw_texture(
                texture,
                explosion.x - half_width,
                explosion.y - half_height,
                WHITE,
            );
        }
    }

    /// Draw text using the custom pixel font texture
    ///
    /// This function renders text using a custom 8x8 pixel font stored in a texture atlas.
    /// The font contains A-Z, 0-9, dash (-), and period (.) characters arranged in a 16x4 grid.
    ///
    /// # Arguments
    /// * `text` - The text string to render
    /// * `x` - X coordinate for text positioning
    /// * `y` - Y coordinate for text positioning
    /// * `scale` - Scale factor for font size (1.0 = 8x8 pixels per character)
    /// * `color` - Color to tint the font (white pixels in font become this color)
    fn draw_custom_text(&self, text: &str, x: f32, y: f32, scale: f32, color: Color) {
        let char_width = 8.0; // Assuming 8x8 pixel characters
        let char_height = 8.0;
        let chars_per_row = 16; // Assuming 16 characters per row in the font texture

        let mut current_x = x;

        for ch in text.chars() {
            if ch == ' ' {
                current_x += char_width * scale;
                continue;
            }

            // Convert character to index (assuming ASCII, A-Z, 0-9, etc.)
            let char_index = if ch.is_ascii_alphanumeric() || ch == '-' || ch == '.' {
                match ch {
                    '0'..='9' => (ch as u32 - '0' as u32) as usize,
                    'A'..='Z' => (ch as u32 - 'A' as u32 + 10) as usize,
                    'a'..='z' => (ch as u32 - 'a' as u32 + 10) as usize,
                    '-' => 36,
                    '.' => 37,
                    _ => 0, // Default to '0'
                }
            } else {
                0
            };

            // Calculate position in font texture
            let tex_x = (char_index % chars_per_row) as f32 * char_width;
            let tex_y = (char_index / chars_per_row) as f32 * char_height;

            // Draw the character with the specified color
            draw_texture_ex(
                &self.custom_font,
                current_x,
                y,
                color,
                DrawTextureParams {
                    source: Some(Rect::new(tex_x, tex_y, char_width, char_height)),
                    dest_size: Some(Vec2::new(char_width * scale, char_height * scale)),
                    ..Default::default()
                },
            );

            current_x += char_width * scale;
        }
    }

    fn draw_menu(&self) {
        // Draw parallax backgrounds
        self.draw_background();

        // Center the main menu content vertically and horizontally
        let center_x = SCREEN_WIDTH / 2.0;
        let center_y = SCREEN_HEIGHT / 2.0;

        // Debug: print screen dimensions
        #[cfg(target_arch = "wasm32")]
        {
            println!(
                "WASM Screen dimensions: {}x{} (expected: {}x{})",
                screen_width(),
                screen_height(),
                SCREEN_WIDTH,
                SCREEN_HEIGHT
            );
            println!("WASM Center coordinates: {}x{}", center_x, center_y);
        }

        // Draw hummel icon on the left side, centered vertically
        let icon_x = 50.0; // Left margin
        let icon_y = center_y - self.intro_icon.height() / 2.0; // Center vertically
        draw_texture(&self.intro_icon, icon_x, icon_y, WHITE);

        // Title at the top
        let title_text = "BUMBLEBEES";
        let title_dims = self.measure_text_retro(title_text, 60);
        self.draw_text_retro(
            title_text,
            center_x - title_dims.width / 2.0,
            100.0,
            60.0,
            BLACK,
        );

        // Subtitle
        let subtitle_text = "Space Invaders Remake";
        let subtitle_dims = self.measure_text_retro(subtitle_text, 24);
        self.draw_text_retro(
            subtitle_text,
            center_x - subtitle_dims.width / 2.0,
            170.0,
            24.0,
            Color::from_rgba(100, 100, 100, 255),
        );

        // Main menu panel - centered horizontally on screen
        let panel_width = 320.0;
        let panel_height = 200.0;
        let panel_x = center_x - panel_width / 2.0;
        let panel_y = center_y - panel_height / 2.0;

        // Debug: print panel position
        #[cfg(target_arch = "wasm32")]
        {
            println!(
                "WASM Panel position: x={}, y={}, w={}, h={}",
                panel_x, panel_y, panel_width, panel_height
            );
        }

        // Draw semi-transparent panel background
        draw_rectangle(
            panel_x,
            panel_y,
            panel_width,
            panel_height,
            Color::from_rgba(255, 255, 255, 200),
        );
        draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, 2.0, BLACK);

        // Name input section inside panel
        let label_text = "Enter Your Name:";
        let label_dims = self.measure_text_retro(label_text, 20);
        let label_x = panel_x + (panel_width - label_dims.width) / 2.0; // Center within panel
        self.draw_text_retro(label_text, label_x, panel_y + 25.0, 20.0, BLACK);

        // Name input box
        let box_width = 280.0;
        let box_height = 40.0;
        let box_x = panel_x + (panel_width - box_width) / 2.0; // Center within panel
        let box_y = panel_y + 55.0;

        // Draw input box background (light gray)
        draw_rectangle(
            box_x,
            box_y,
            box_width,
            box_height,
            Color::from_rgba(240, 240, 240, 255),
        );
        // Draw border
        draw_rectangle_lines(box_x, box_y, box_width, box_height, 2.0, BLACK);

        // Display text inside the box - properly centered
        if !self.player_name.is_empty() {
            // Center the entered name both horizontally and vertically in the box
            let font_size = 24.0;
            let name_dims = self.measure_text_retro(&self.player_name, font_size as u16);
            let name_x = box_x + (box_width - name_dims.width) / 2.0;
            let name_y = box_y + (box_height + font_size) / 2.0 - font_size * 0.25; // Better vertical centering
            self.draw_text_retro(&self.player_name, name_x, name_y, font_size, BLACK);
        } else {
            // Center placeholder text both horizontally and vertically in the box
            let placeholder = "Type your name...";
            let placeholder_font_size = 22.0;
            let placeholder_dims =
                self.measure_text_retro(placeholder, placeholder_font_size as u16);
            let placeholder_x = box_x + (box_width - placeholder_dims.width) / 2.0;
            let placeholder_y =
                box_y + (box_height + placeholder_font_size) / 2.0 - placeholder_font_size * 0.25; // Better vertical centering
            self.draw_text_retro(
                placeholder,
                placeholder_x,
                placeholder_y,
                placeholder_font_size,
                Color::from_rgba(150, 150, 150, 255),
            );
        }

        // Start button - centered below input box
        let button_width = 280.0;
        let button_height = 45.0;
        let button_x = panel_x + (panel_width - button_width) / 2.0; // Center within panel
        let button_y = panel_y + 120.0;

        let button_color = if self.player_name.is_empty() {
            Color::from_rgba(180, 180, 180, 255)
        } else {
            Color::from_rgba(0, 150, 0, 255)
        };

        draw_rectangle(
            button_x,
            button_y,
            button_width,
            button_height,
            button_color,
        );
        draw_rectangle_lines(button_x, button_y, button_width, button_height, 2.0, BLACK);

        // Button text - properly centered
        let button_text = "START GAME";
        let button_font_size = 24.0;
        let button_text_dims = self.measure_text_retro(button_text, button_font_size as u16);
        let button_text_x = button_x + (button_width - button_text_dims.width) / 2.0;
        let button_text_y =
            button_y + (button_height + button_font_size) / 2.0 - button_font_size * 0.25; // Better vertical centering
        self.draw_text_retro(
            button_text,
            button_text_x,
            button_text_y,
            button_font_size,
            WHITE,
        );

        // Highscores section - bottom right
        let highscore_x = SCREEN_WIDTH - 300.0;
        let highscore_y = SCREEN_HEIGHT - 250.0;

        // Highscores header
        self.draw_text_retro("HIGH SCORES", highscore_x + 10.0, highscore_y, 24.0, BLACK);

        // Display highscores
        let top_scores = self.highscore_manager.get_top_scores(5); // Show only top 5
        for (i, entry) in top_scores.iter().enumerate() {
            let score_text = format!("{}. {} - {}", i + 1, entry.name, entry.score);
            let y_pos = highscore_y + 35.0 + i as f32 * 25.0;

            self.draw_text_retro(&score_text, highscore_x + 10.0, y_pos, 18.0, BLACK);
        }
    }

    fn draw_game_over(&self) {
        // Center "GAME OVER" text
        let game_over_text = "GAME OVER";
        let game_over_dims = self.measure_text_retro(game_over_text, 80);
        self.draw_text_retro(
            game_over_text,
            SCREEN_WIDTH / 2.0 - game_over_dims.width / 2.0,
            200.0,
            80.0,
            RED,
        );

        // Center score text
        let score_text = format!("Final Score: {}", self.score);
        let score_dims = self.measure_text_retro(&score_text, 50);
        self.draw_text_retro(
            &score_text,
            SCREEN_WIDTH / 2.0 - score_dims.width / 2.0,
            300.0,
            50.0,
            BLACK,
        );

        // Center "Press R" text
        let press_r_text = "Press R to Return to Menu";
        let press_r_dims = self.measure_text_retro(press_r_text, 30);
        self.draw_text_retro(
            press_r_text,
            SCREEN_WIDTH / 2.0 - press_r_dims.width / 2.0,
            400.0,
            30.0,
            Color::from_rgba(0, 0, 0, 255),
        );
    }

    fn draw_score(&self) {
        let score_text = format!("Score: {}", self.score);

        // Draw shadow for bold effect
        self.draw_text_retro(
            &score_text,
            SCREEN_WIDTH - 178.0,
            42.0,
            32.0,
            Color::from_rgba(0, 0, 0, 128),
        );

        // Draw main text in red with larger font for bold effect
        self.draw_text_retro(&score_text, SCREEN_WIDTH - 180.0, 40.0, 32.0, RED);
    }

    fn handle_input(&mut self) {
        match self.state {
            GameState::Menu => {
                // Handle text input
                if let Some(character) = get_last_key_pressed() {
                    println!("Key pressed: {:?}", character);
                    match character {
                        KeyCode::Backspace => {
                            self.player_name.pop();
                            println!("Player name after backspace: {}", self.player_name);
                        }
                        KeyCode::Enter => {
                            println!("Enter pressed, starting game");
                            self.start_game();
                        }
                        _ => {}
                    }
                }

                // Handle character input
                if let Some(ch) = get_char_pressed() {
                    println!("Char pressed: {}", ch);
                    if ch.is_alphanumeric() && self.player_name.len() < 20 {
                        self.player_name.push(ch);
                        println!("Player name: {}", self.player_name);
                    }
                }

                // Handle mouse click on start button
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();
                    println!("Mouse clicked at: ({}, {})", mouse_x, mouse_y);
                    // Button position matches the centered panel layout
                    let center_x = SCREEN_WIDTH / 2.0;
                    let center_y = SCREEN_HEIGHT / 2.0;
                    let panel_x = center_x - 160.0; // panel_width / 2 = 320 / 2 = 160
                    let panel_y = center_y - 100.0; // panel_height / 2 = 200 / 2 = 100
                    let button_x = panel_x + (320.0 - 280.0) / 2.0; // Center within panel
                    let button_y = panel_y + 120.0;
                    let button_rect = Rect::new(button_x, button_y, 280.0, 45.0);
                    println!(
                        "Button rect: x={}, y={}, w={}, h={}",
                        button_rect.x, button_rect.y, button_rect.w, button_rect.h
                    );

                    if button_rect.contains(Vec2::new(mouse_x, mouse_y)) {
                        println!("Button clicked!");
                        self.start_game();
                    } else {
                        println!("Click outside button");
                    }
                }

                // Also allow pressing Space bar to start game from menu
                if is_key_pressed(KeyCode::Space) {
                    println!("Space pressed in menu, starting game");
                    self.start_game();
                }
            }
            GameState::Playing => {
                if is_key_pressed(KeyCode::Space) {
                    self.shoot();
                }
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::R) {
                    self.reset();
                }
            }
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "BumbleBees".to_owned(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[macroquad::main(window_conf)]
async fn main() {
    // Desktop version
    log::info!("Starting BumbleBees game (Desktop)");

    let mut game = Game::new().await;

    loop {
        let dt = get_frame_time();

        game.handle_input();
        game.update(dt);
        game.draw();

        next_frame().await
    }
}

#[cfg(target_arch = "wasm32")]
#[macroquad::main(window_conf)]
async fn main() {
    // WASM version
    // Note: macroquad provides its own logging to console
    println!("Starting BumbleBees game (WASM)");

    let mut game = Game::new().await;

    loop {
        let dt = get_frame_time();

        game.handle_input();
        game.update(dt);
        game.draw();

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_configuration() {
        let conf = window_conf();

        assert_eq!(conf.window_width, SCREEN_WIDTH as i32);
        assert_eq!(conf.window_height, SCREEN_HEIGHT as i32);
        assert!(!conf.window_resizable);
    }

    #[test]
    fn test_game_state_transitions() {
        // Test game state enum values
        assert_eq!(GameState::Menu as u8, 0);
        assert_eq!(GameState::Playing as u8, 1);
        assert_eq!(GameState::GameOver as u8, 2);
    }

    #[test]
    fn test_wave_enemy_counts() {
        // Test that wave generation produces correct enemy counts
        let wave1 = generate_wave(1);
        assert_eq!(wave1.len(), 3 * 10); // 3 rows × 10 columns

        let wave2 = generate_wave(2);
        assert_eq!(wave2.len(), 4 * 10); // 4 rows × 10 columns

        let wave3 = generate_wave(3);
        assert_eq!(wave3.len(), 5 * 10); // 5 rows × 10 columns
    }

    #[test]
    fn test_enemy_positions_in_wave() {
        let enemies = generate_wave(1);

        // Check first enemy position (centered at top)
        assert_eq!(enemies[0].x, 242.0);
        assert_eq!(enemies[0].y, 50.0);

        // Check enemy spacing
        assert_eq!(enemies[1].x, 242.0); // Same column
        assert_eq!(enemies[1].y, 100.0); // Next row

        assert_eq!(enemies[3].x, 302.0); // Next column
        assert_eq!(enemies[3].y, 50.0); // First row
    }

    // =======================================================================
    // BackgroundLayer Tests
    // =======================================================================

    #[test]
    fn test_background_layer_creation() {
        let layer = BackgroundLayer::new(-50.0, 1024.0, BackgroundLayerType::Clouds);

        assert_eq!(layer.speed, -50.0);
        assert_eq!(layer.parts[0], 0.0);
        assert_eq!(layer.parts[1], 1024.0);
    }

    #[test]
    fn test_background_layer_static_no_movement() {
        let mut layer = BackgroundLayer::new(0.0, 1024.0, BackgroundLayerType::Sky);
        let original_parts = layer.parts;

        // Update with 1 second delta time
        layer.update(1.0, 1024.0);

        // Static layer (speed = 0) shouldn't move
        assert_eq!(layer.parts[0], original_parts[0]);
        assert_eq!(layer.parts[1], original_parts[1]);
    }

    #[test]
    fn test_background_layer_scrolls_left() {
        let mut layer = BackgroundLayer::new(-100.0, 1024.0, BackgroundLayerType::Clouds);

        // Update with 1 second delta time
        layer.update(1.0, 1024.0);

        // Both parts should move left by 100 pixels
        assert_eq!(layer.parts[0], -100.0);
        assert_eq!(layer.parts[1], 924.0);
    }

    #[test]
    fn test_background_layer_wraparound() {
        let mut layer = BackgroundLayer::new(-100.0, 1024.0, BackgroundLayerType::FarField);

        // Scroll for enough time to move second part off-screen
        // parts[1] starts at 1024.0, needs to reach < 0.0
        // At -100 px/s, needs > 10.24 seconds
        layer.update(11.0, 1024.0);

        // After wraparound, the parts should have been swapped and repositioned
        // The key is that after swap, parts should still provide seamless coverage
        // Since we scrolled left by 1100 pixels total (-100 * 11):
        // Original: [0.0, 1024.0] -> [-1100.0, -76.0]
        // After wraparound and swap, one part should be repositioned

        // After wraparound logic triggers (when parts[1] < 0),
        // parts[0] gets repositioned to parts[1] + texture_width, then they swap
        // So we should have continuous coverage with parts spaced by texture_width
        let spacing = (layer.parts[0] - layer.parts[1]).abs();
        assert!((spacing - 1024.0).abs() < 1.0,
                "Parts should be spaced by texture width (1024), but spacing is {}", spacing);
    }

    #[test]
    fn test_background_layer_reset() {
        let mut layer = BackgroundLayer::new(-100.0, 1024.0, BackgroundLayerType::NearField);

        // Scroll the layer
        layer.update(5.0, 1024.0);
        assert_ne!(layer.parts[0], 0.0); // Should have moved

        // Reset
        layer.reset(1024.0);

        // Should be back to initial positions
        assert_eq!(layer.parts[0], 0.0);
        assert_eq!(layer.parts[1], 1024.0);
    }

    #[test]
    fn test_background_layer_scrolls_right() {
        let mut layer = BackgroundLayer::new(50.0, 800.0, BackgroundLayerType::Layer4);

        // Update with 1 second delta time (positive speed = scroll right)
        layer.update(1.0, 800.0);

        // Both parts should move right by 50 pixels
        assert_eq!(layer.parts[0], 50.0);
        assert_eq!(layer.parts[1], 850.0);
    }

    #[test]
    fn test_background_layer_small_delta_time() {
        let mut layer = BackgroundLayer::new(-100.0, 1024.0, BackgroundLayerType::Layer5);

        // Update with small delta time (typical frame at 60fps)
        layer.update(0.016, 1024.0);

        // Movement should be proportional: 100 * 0.016 = 1.6 pixels
        assert!((layer.parts[0] - (-1.6)).abs() < 0.01);
        assert!((layer.parts[1] - 1022.4).abs() < 0.01);
    }
}
