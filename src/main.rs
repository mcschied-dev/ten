//! BumbleBees - Space Invaders-style arcade shooter
//! Macroquad edition with WASM support

use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound_once, play_sound, stop_sound, PlaySoundParams, Sound};
use macroquad::texture::DrawTextureParams;
use std::sync::{Arc, Mutex};

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
    scroll_text_x: Arc<Mutex<f32>>,
    scroll_direction: Arc<Mutex<f32>>,
    scroll_text_time: f32, // Time accumulator for wobble effect
    background_scroll_x: f32,
    background_scroll_x2: f32, // Second layer for parallax

    // Resources
    background: Texture2D,
    background2: Texture2D, // Second background layer for parallax
    enemy_image: Texture2D,

    // Audio
    shoot_sound: Option<Sound>,
    hit_sound: Option<Sound>,
    background_music: Option<Sound>,
}

impl Game {
    async fn new() -> Self {
        log::info!("Loading game resources");

        let background = load_texture_fallback("resources/background.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[0, 0, 0, 255]));

        // Use same background for second layer (could be different image)
        let background2 = load_texture_fallback("resources/background.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[0, 0, 0, 255]));

        let enemy_image = load_texture_fallback("resources/enemy.png")
            .await
            .unwrap_or_else(|_| Texture2D::from_rgba8(1, 1, &[255, 255, 255, 255]));

        let shoot_sound = load_sound_fallback("resources/shoot.wav").await.ok();

        let hit_sound = load_sound_fallback("resources/hit.wav").await.ok();

        let background_music = load_sound_fallback("resources/background_music.wav").await.ok();

        log::info!("Game state created successfully");

        Self {
            player: Player::new(),
            bullets: Vec::new(),
            enemies: generate_wave(1),
            enemy_speed: INITIAL_ENEMY_SPEED,
            descent_speed: 100.0, // pixels per second for controlled descent
            descent_distance: 0.0, // current descent progress
            wave_number: 1,
            state: GameState::Menu,
            score: 0,
            player_name: String::new(),
            highscore_manager: HighscoreManager::new("highscores.txt"),
            scroll_text_x: Arc::new(Mutex::new(SCREEN_WIDTH)),
            scroll_direction: Arc::new(Mutex::new(-1.0)),
            scroll_text_time: 0.0,
            background_scroll_x: 0.0,
            background_scroll_x2: 0.0,
            background,
            background2,
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
        self.background_scroll_x = 0.0;
        self.background_scroll_x2 = 0.0;
        self.player_name.clear();

        let mut text_x = self.scroll_text_x.lock().unwrap();
        *text_x = SCREEN_WIDTH;
        self.scroll_text_time = 0.0;
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
                play_sound(sound, PlaySoundParams { looped: true, volume: 0.5 });
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

                if (moving_right && enemy.x >= SCREEN_WIDTH - 20.0) ||
                    (moving_left && enemy.x <= 20.0) {
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

    fn update_background_scroll(&mut self, dt: f32) {
        let bg_width = self.background.width();

        // Main background layer (foreground)
        self.background_scroll_x -= BACKGROUND_SCROLL_SPEED * dt;
        if self.background_scroll_x <= -bg_width + 1.0 {
            self.background_scroll_x += bg_width;
        }

        // Second background layer (background) - moves slower for parallax
        self.background_scroll_x2 -= BACKGROUND_SCROLL_SPEED * 0.3 * dt; // 30% speed
        if self.background_scroll_x2 <= -bg_width + 1.0 {
            self.background_scroll_x2 += bg_width;
        }
    }

    fn update(&mut self, dt: f32) {
        match self.state {
            GameState::Menu => {
                self.update_background_scroll(dt);
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
                self.update_scroll_text(dt);

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
                self.draw_scroll_text();
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
        let bg_width = self.background.width();
        let screen_width = screen_width();

        // Calculate how many background instances we need to cover the screen
        let instances_needed = ((screen_width / bg_width).ceil() as i32) + 2;

        // Draw background layer (slower, more transparent, horizontally mirrored)
        for i in -1..instances_needed {
            let x_pos = self.background_scroll_x2 + (i as f32 * bg_width);
            // Use draw_texture_ex for vertical mirroring
            draw_texture_ex(
                &self.background2,
                x_pos,
                0.0,
                Color::from_rgba(255, 255, 255, 180), // 70% opacity
                DrawTextureParams {
                    flip_y: true, // Mirror vertically
                    ..Default::default()
                },
            );
        }

        // Draw foreground layer (faster, fully opaque, normal orientation)
        for i in -1..instances_needed {
            let x_pos = self.background_scroll_x + (i as f32 * bg_width);
            draw_texture(&self.background, x_pos, 0.0, WHITE);
        }
    }

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

    fn draw_menu(&self) {
        // Title
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

        // Highscores section
        draw_text(
            "HIGH SCORES",
            SCREEN_WIDTH / 2.0 - 150.0,
            180.0,
            40.0,
            WHITE,
        );

        // Display top 10 highscores
        let top_scores = self.highscore_manager.get_top_scores(10);
        for (i, entry) in top_scores.iter().enumerate() {
            let score_text = format!("{}. {} - {}", i + 1, entry.name, entry.score);
            draw_text(
                &score_text,
                SCREEN_WIDTH / 2.0 - 180.0,
                220.0 + i as f32 * 30.0,
                24.0,
                Color::from_rgba(200, 200, 200, 255),
            );
        }

        // Name input section
        draw_text(
            "Enter Your Name:",
            SCREEN_WIDTH / 2.0 - 140.0,
            560.0,
            30.0,
            WHITE,
        );

        // Name input box
        draw_rectangle_lines(SCREEN_WIDTH / 2.0 - 150.0, 600.0, 300.0, 40.0, 2.0, WHITE);

        // Display current input
        draw_text(
            &self.player_name,
            SCREEN_WIDTH / 2.0 - 140.0,
            625.0,
            28.0,
            WHITE,
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
        draw_text(&score_text, SCREEN_WIDTH - 178.0, 42.0, 32.0, Color::from_rgba(0, 0, 0, 128));

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
