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
use entities::{Bullet, Enemy, Player};
use highscore::HighscoreManager;
use systems::{generate_wave, process_collisions};

/// Load texture with fallback paths for bundle compatibility
async fn load_texture_fallback(path: &str) -> Result<Texture2D, macroquad::Error> {
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

/// Load sound with fallback paths for bundle compatibility
async fn load_sound_fallback(path: &str) -> Result<Sound, macroquad::Error> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    Menu,
    Playing,
    GameOver,
}

/// Represents a single parallax background layer with infinite scrolling
#[derive(Debug, Clone)]
struct BackgroundLayer {
    /// Scroll speed in pixels per second (negative = left, positive = right)
    speed: f32,
    /// Two positions for seamless infinite scrolling
    parts: [f32; 2],
    /// Layer type for texture selection
    layer_type: BackgroundLayerType,
}

#[derive(Debug, Clone, Copy)]
enum BackgroundLayerType {
    Sky,
    Layer10,
    Clouds,
    Layer5,
    FarField,
    Layer6,
    NearField,
    Layer7,
    Layer8,
}

impl BackgroundLayer {
    fn new(speed: f32, texture_width: f32, layer_type: BackgroundLayerType) -> Self {
        Self {
            speed,
            parts: [0.0, texture_width],
            layer_type,
        }
    }

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

    fn reset(&mut self, texture_width: f32) {
        self.parts = [0.0, texture_width];
    }
}

struct Game {
    player: Player,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
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
    layer_5: Texture2D,
    layer_6: Texture2D,
    layer_7: Texture2D,
    layer_8: Texture2D,
    layer_10: Texture2D,
    intro_icon: Texture2D,
    custom_font: Texture2D,
    enemy_image: Texture2D,

    // Audio
    shoot_sound: Option<Sound>,
    hit_sound: Option<Sound>,
    background_music: Option<Sound>,
}

impl Game {
    async fn new() -> Self {
        log::info!("Loading game resources");

        let sky = load_texture_fallback("resources/1.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[135, 206, 235, 255])); // Sky blue fallback

        let clouds = load_texture_fallback("resources/2.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[255, 255, 255, 255]));

        let far_field = load_texture_fallback("resources/3.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[34, 139, 34, 255])); // Forest green fallback

        let near_field = load_texture_fallback("resources/background.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[0, 100, 0, 255])); // Dark green fallback

        let layer_5 = load_texture_fallback("resources/5.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[150, 150, 150, 255]));

        let layer_6 = load_texture_fallback("resources/6.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[150, 150, 150, 255]));

        let layer_7 = load_texture_fallback("resources/7.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[150, 150, 150, 255]));

        let layer_8 = load_texture_fallback("resources/8.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[150, 150, 150, 255]));

        let layer_10 = load_texture_fallback("resources/10.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[150, 150, 150, 255]));

        let intro_icon = load_texture_fallback("resources/hummel_icns_temp.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[200, 200, 200, 255])); // Light gray fallback

        let custom_font = load_texture_fallback("resources/custom_font.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[255, 255, 255, 255])); // White fallback

        let enemy_image = load_texture_fallback("resources/enemy.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[255, 255, 255, 255]));

        let shoot_sound = load_sound_fallback("resources/shoot.wav").await.ok();

        let hit_sound = load_sound_fallback("resources/hit.wav").await.ok();

        let background_music = load_sound_fallback("resources/background_music.wav")
            .await
            .ok();

        // Initialize background layers for parallax scrolling
        let background_layers = vec![
            BackgroundLayer::new(0.0, sky.width(), BackgroundLayerType::Sky), // Static sky
            BackgroundLayer::new(-10.0, layer_10.width(), BackgroundLayerType::Layer10), // Very slow layer 10
            BackgroundLayer::new(-20.0, clouds.width(), BackgroundLayerType::Clouds), // Slow clouds
            BackgroundLayer::new(-50.0, layer_5.width(), BackgroundLayerType::Layer5), // Medium-slow layer 5
            BackgroundLayer::new(-100.0, far_field.width(), BackgroundLayerType::FarField), // Medium far-field
            BackgroundLayer::new(-200.0, layer_6.width(), BackgroundLayerType::Layer6), // Medium-fast layer 6
            BackgroundLayer::new(-300.0, near_field.width(), BackgroundLayerType::NearField), // Fast near-field
            BackgroundLayer::new(-400.0, layer_7.width(), BackgroundLayerType::Layer7), // Very fast layer 7
            BackgroundLayer::new(-500.0, layer_8.width(), BackgroundLayerType::Layer8), // Fastest layer 8
        ];

        log::info!("Game state created successfully");

        Self {
            player: Player::new(),
            bullets: Vec::new(),
            enemies: generate_wave(1),
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
            layer_5,
            layer_6,
            layer_7,
            layer_8,
            layer_10,
            intro_icon,
            custom_font,
            enemy_image,
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
                BackgroundLayerType::Layer10 => self.layer_10.width(),
                BackgroundLayerType::Clouds => self.clouds.width(),
                BackgroundLayerType::Layer5 => self.layer_5.width(),
                BackgroundLayerType::FarField => self.far_field.width(),
                BackgroundLayerType::Layer6 => self.layer_6.width(),
                BackgroundLayerType::NearField => self.near_field.width(),
                BackgroundLayerType::Layer7 => self.layer_7.width(),
                BackgroundLayerType::Layer8 => self.layer_8.width(),
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

    fn update_collisions(&mut self) {
        let enemies_destroyed = process_collisions(&mut self.enemies, &self.bullets);

        if enemies_destroyed > 0 {
            if let Some(ref sound) = self.hit_sound {
                play_sound_once(sound);
            }
            self.score += enemies_destroyed as u32 * POINTS_PER_ENEMY;
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
                BackgroundLayerType::Layer10 => self.layer_10.width(),
                BackgroundLayerType::Clouds => self.clouds.width(),
                BackgroundLayerType::Layer5 => self.layer_5.width(),
                BackgroundLayerType::FarField => self.far_field.width(),
                BackgroundLayerType::Layer6 => self.layer_6.width(),
                BackgroundLayerType::NearField => self.near_field.width(),
                BackgroundLayerType::Layer7 => self.layer_7.width(),
                BackgroundLayerType::Layer8 => self.layer_8.width(),
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
                BackgroundLayerType::Layer10 => &self.layer_10,
                BackgroundLayerType::Clouds => &self.clouds,
                BackgroundLayerType::Layer5 => &self.layer_5,
                BackgroundLayerType::FarField => &self.far_field,
                BackgroundLayerType::Layer6 => &self.layer_6,
                BackgroundLayerType::NearField => &self.near_field,
                BackgroundLayerType::Layer7 => &self.layer_7,
                BackgroundLayerType::Layer8 => &self.layer_8,
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

        // Draw intro icon as extra layer (only visible in menu)
        let icon_x = 30.0; // 30px from left edge
        let icon_y = 30.0; // 30px from top edge
        draw_texture(&self.intro_icon, icon_x, icon_y, WHITE);

        // Title - commented out
        /*
        let title = "BumbleBees";
        let title_size = 80.0;
        let title_dims = measure_text(title, None, title_size as u16, 1.0);
        draw_text(
            title,
            SCREEN_WIDTH / 2.0 - title_dims.width / 2.0,
            100.0,
            title_size,
            Color::from_rgba(255, 215, 0, 255),
        );
        */

        // Highscores section - using custom font (moved to top)
        self.draw_custom_text(
            "HIGH SCORES",
            SCREEN_WIDTH / 2.0 - 148.0, // Back to original positioning
            52.0,                       // Moved to top
            2.0,                        // Back to normal size
            Color::from_rgba(0, 0, 0, 128),
        );
        self.draw_custom_text(
            "HIGH SCORES",
            SCREEN_WIDTH / 2.0 - 150.0, // Back to original positioning
            50.0,                       // Moved to top
            2.0,                        // Back to normal size
            BLACK,
        );

        // Display all highscores with scrolling animation (C64 style)
        let top_scores = self.highscore_manager.get_top_scores(usize::MAX);
        for (i, entry) in top_scores.iter().enumerate() {
            let score_text = format!("{}. {} - {}", i + 1, entry.name, entry.score);
            let y_pos = 90.0 + i as f32 * 30.0 + self.highscore_scroll_offset; // Back to original spacing

            // Only draw if visible on screen (adjusted for top position)
            if y_pos > 40.0 && y_pos < 500.0 {
                // Draw shadow for bold effect
                self.draw_custom_text(
                    &score_text,
                    SCREEN_WIDTH / 2.0 - 178.0, // Back to original positioning
                    y_pos + 2.0,                // Back to original shadow offset
                    1.5,                        // Back to normal size
                    Color::from_rgba(0, 0, 0, 128),
                );
                self.draw_custom_text(
                    &score_text,
                    SCREEN_WIDTH / 2.0 - 180.0, // Back to original positioning
                    y_pos,
                    1.5,   // Back to normal size
                    BLACK, // Changed to black
                );
            }
        }

        // Name input section
        // Draw shadow for bold effect
        draw_text(
            "Enter Your Name:",
            SCREEN_WIDTH / 2.0 - 138.0,
            562.0,
            30.0,
            Color::from_rgba(0, 0, 0, 128),
        );
        draw_text(
            "Enter Your Name:",
            SCREEN_WIDTH / 2.0 - 140.0,
            560.0,
            30.0,
            BLACK,
        );

        // Name input box
        draw_rectangle_lines(SCREEN_WIDTH / 2.0 - 150.0, 600.0, 300.0, 40.0, 2.0, WHITE);

        // Display current input
        draw_text(
            &self.player_name,
            SCREEN_WIDTH / 2.0 - 140.0,
            625.0,
            28.0,
            BLACK,
        );

        // Start button
        let button_color = if self.player_name.is_empty() {
            Color::from_rgba(100, 100, 100, 255)
        } else {
            Color::from_rgba(0, 200, 0, 255)
        };

        draw_rectangle(SCREEN_WIDTH / 2.0 - 100.0, 660.0, 200.0, 50.0, button_color);

        draw_text("START GAME", SCREEN_WIDTH / 2.0 - 75.0, 690.0, 32.0, WHITE);
    }

    fn draw_game_over(&self) {
        draw_text("GAME OVER", SCREEN_WIDTH / 2.0 - 200.0, 280.0, 80.0, RED);

        let score_text = format!("Final Score: {}", self.score);
        draw_text(&score_text, SCREEN_WIDTH / 2.0 - 150.0, 380.0, 50.0, WHITE);

        draw_text(
            "Press R to Return to Menu",
            SCREEN_WIDTH / 2.0 - 180.0,
            480.0,
            30.0,
            Color::from_rgba(200, 200, 200, 255),
        );
    }

    fn draw_score(&self) {
        let score_text = format!("Score: {}", self.score);

        // Draw shadow for bold effect
        draw_text(
            &score_text,
            SCREEN_WIDTH - 178.0,
            42.0,
            32.0,
            Color::from_rgba(0, 0, 0, 128),
        );

        // Draw main text in red with larger font for bold effect
        draw_text(&score_text, SCREEN_WIDTH - 180.0, 40.0, 32.0, RED);
    }

    fn handle_input(&mut self) {
        match self.state {
            GameState::Menu => {
                // Handle text input
                if let Some(character) = get_last_key_pressed() {
                    match character {
                        KeyCode::Backspace => {
                            self.player_name.pop();
                        }
                        KeyCode::Enter => {
                            self.start_game();
                        }
                        _ => {}
                    }
                }

                // Handle character input
                if let Some(ch) = get_char_pressed() {
                    if ch.is_alphanumeric() && self.player_name.len() < 20 {
                        self.player_name.push(ch);
                    }
                }

                // Handle mouse click on start button
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();
                    let button_rect = Rect::new(SCREEN_WIDTH / 2.0 - 100.0, 660.0, 200.0, 50.0);

                    if button_rect.contains(Vec2::new(mouse_x, mouse_y)) {
                        self.start_game();
                    }
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
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    // WASM version
    log::info!("Starting BumbleBees game (WASM)");

    wasm_bindgen_futures::spawn_local(async {
        let mut game = Game::new().await;

        loop {
            let dt = get_frame_time();

            game.handle_input();
            game.update(dt);
            game.draw();

            next_frame().await
        }
    });
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
}
