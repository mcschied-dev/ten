use ggez::audio::{SoundSource, Source};
use ggez::event::EventHandler;
use ggez::graphics::{Color, Image, Text, TextFragment};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};
use std::sync::{Arc, Mutex};

use crate::constants::*;
use crate::entities::{Bullet, Enemy, Player};
use crate::highscore::HighscoreManager;
use crate::systems::{generate_wave, process_collisions};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

pub struct MainState {
    pub player: Player,
    pub bullets: Vec<Bullet>,
    pub enemies: Vec<Enemy>,
    pub enemy_direction: f32,
    pub enemy_speed: f32,
    pub wave_number: u32,
    pub state: GameState,
    pub score: u32,
    pub moved_down: bool,

    // Player and highscore
    pub player_name: String,
    pub highscore_manager: HighscoreManager,

    // UI elements
    pub scroll_text: Text,
    pub text_x: Arc<Mutex<f32>>,
    pub scroll_direction: Arc<Mutex<f32>>,
    pub background_scroll_x: f32,

    // Resources
    pub shoot_sound: Source,
    pub hit_sound: Source,
    pub background_music: Source,
    pub background: Image,
    pub enemy_image: Image,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let shoot_sound = Source::new(ctx, "/shoot.wav")?;
        let hit_sound = Source::new(ctx, "/hit.wav")?;
        let background = Image::from_path(ctx, "/background.png")?;
        let enemy_image = Image::from_path(ctx, "/enemy.png")?;

        let mut background_music = Source::new(ctx, "/background_music.wav")?;
        background_music.set_repeat(true);
        background_music.play(ctx)?;

        let scroll_text = Text::new(TextFragment {
            text: "Happy New Year".to_string(),
            color: Some(Color::from_rgb(0, 0, 0)),
            scale: Some(ggez::graphics::PxScale::from(50.0)),
            ..Default::default()
        });

        let text_x = Arc::new(Mutex::new(SCREEN_WIDTH));
        let scroll_direction = Arc::new(Mutex::new(-1.0));

        Ok(Self {
            player: Player::new(),
            bullets: Vec::new(),
            enemies: generate_wave(1),
            enemy_direction: 1.0,
            enemy_speed: INITIAL_ENEMY_SPEED,
            wave_number: 1,
            state: GameState::Menu,
            score: 0,
            moved_down: false,
            player_name: String::new(),
            highscore_manager: HighscoreManager::new("highscores.txt"),
            scroll_text,
            text_x,
            scroll_direction,
            background_scroll_x: 0.0,
            shoot_sound,
            hit_sound,
            background_music,
            background,
            enemy_image,
        })
    }

    pub fn reset(&mut self, ctx: &mut Context) {
        self.player.reset();
        self.bullets.clear();
        self.enemies = generate_wave(1);
        self.enemy_direction = 1.0;
        self.enemy_speed = INITIAL_ENEMY_SPEED;
        self.wave_number = 1;
        self.score = 0;
        self.moved_down = false;
        self.state = GameState::Menu;
        self.background_scroll_x = 0.0;
        self.player_name.clear();

        let mut text_x = self.text_x.lock().unwrap();
        *text_x = SCREEN_WIDTH;

        self.background_music.stop(ctx).ok();
        self.background_music
            .play(ctx)
            .expect("Failed to play background music");
    }

    pub fn start_game(&mut self) {
        if !self.player_name.is_empty() {
            self.state = GameState::Playing;
            self.score = 0;
            self.wave_number = 1;
            self.enemies = generate_wave(1);
            self.bullets.clear();
            self.player.reset();
            self.enemy_direction = 1.0;
            self.enemy_speed = INITIAL_ENEMY_SPEED;
            self.moved_down = false;
        }
    }

    pub fn save_and_return_to_menu(&mut self, ctx: &mut Context) {
        // Save highscore
        if !self.player_name.is_empty() && self.score > 0 {
            self.highscore_manager
                .save_highscore(&self.player_name, self.score);
        }

        // Return to menu
        self.reset(ctx);
    }

    pub fn shoot(&mut self, ctx: &mut Context) {
        if matches!(self.state, GameState::Playing) {
            let new_bullets = self.player.shoot();
            self.bullets.extend(new_bullets);
            let _ = self.shoot_sound.play(ctx);
        }
    }

    pub fn update_bullets(&mut self, dt: f32) {
        for bullet in &mut self.bullets {
            bullet.update(dt);
        }
        self.bullets.retain(|bullet| !bullet.is_out_of_bounds());
    }

    pub fn update_enemies(&mut self, dt: f32) -> GameResult<()> {
        let mut reached_edge = false;

        for enemy in &mut self.enemies {
            enemy.update(self.enemy_direction, self.enemy_speed, dt);
            if enemy.has_reached_edge() {
                reached_edge = true;
            }
        }

        if reached_edge && !self.moved_down {
            self.enemy_direction *= -1.0;
            self.moved_down = true;

            for enemy in &mut self.enemies {
                enemy.move_down(40.0);

                if enemy.has_breached_defender_line() {
                    println!("Enemy breached the defender line: y = {}", enemy.y);
                    self.state = GameState::GameOver;
                    // Save highscore immediately when game over
                    if !self.player_name.is_empty() && self.score > 0 {
                        self.highscore_manager
                            .save_highscore(&self.player_name, self.score);
                    }
                    return Ok(());
                }
            }
        } else if !reached_edge {
            self.moved_down = false;
        }

        Ok(())
    }

    pub fn update_collisions(&mut self, ctx: &mut Context) {
        let enemies_destroyed = process_collisions(&mut self.enemies, &self.bullets);

        if enemies_destroyed > 0 {
            let _ = self.hit_sound.play(ctx);
            self.score += enemies_destroyed as u32 * POINTS_PER_ENEMY;
        }
    }

    pub fn check_wave_complete(&mut self) {
        if self.enemies.is_empty() {
            self.wave_number += 1;
            self.enemy_speed += SPEED_INCREASE_PER_WAVE;
            self.player.upgrade();
            self.enemies = generate_wave(self.wave_number);
        }
    }

    pub fn update_scroll_text(&mut self, dt: f32) {
        let mut position = self.text_x.lock().unwrap();
        let mut direction = self.scroll_direction.lock().unwrap();

        *position += *direction * TEXT_SCROLL_SPEED * dt;

        if *position <= 0.0 && *direction < 0.0 {
            *direction = 1.0;
        } else if *position >= SCREEN_WIDTH && *direction > 0.0 {
            *direction = -1.0;
        }
    }

    pub fn update_background_scroll(&mut self, dt: f32) {
        // Scroll background from right to left
        self.background_scroll_x -= BACKGROUND_SCROLL_SPEED * dt;

        // Get background width for seamless wrapping
        let bg_width = self.background.width() as f32;

        // Wrap around when the first image scrolls off screen
        if self.background_scroll_x <= -bg_width {
            self.background_scroll_x += bg_width;
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();

        match self.state {
            GameState::Menu => {
                // Just update scrolling background in menu
                self.update_background_scroll(dt);
            }
            GameState::Playing => {
                // Handle player input
                if ctx.keyboard.is_key_pressed(KeyCode::Left) {
                    self.player.move_left(dt);
                }
                if ctx.keyboard.is_key_pressed(KeyCode::Right) {
                    self.player.move_right(dt);
                }

                // Update scrolling background
                self.update_background_scroll(dt);

                // Update scrolling text
                self.update_scroll_text(dt);

                // Update bullets
                self.update_bullets(dt);

                // Update enemies
                self.update_enemies(dt)?;

                // Process collisions
                self.update_collisions(ctx);

                // Check if wave is complete
                self.check_wave_complete();
            }
            GameState::GameOver => {
                self.background_music.stop(ctx)?;
                // Just update background in game over
                self.update_background_scroll(dt);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        crate::rendering::draw_game(self, ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeat: bool,
    ) -> GameResult {
        match self.state {
            GameState::Menu => {
                // Handle text input for player name
                if let Some(keycode) = input.keycode {
                    match keycode {
                        KeyCode::Back => {
                            self.player_name.pop();
                        }
                        KeyCode::Return => {
                            self.start_game();
                        }
                        _ => {}
                    }
                }
            }
            GameState::Playing => {
                if let Some(keycode) = input.keycode {
                    match keycode {
                        KeyCode::Space => self.shoot(ctx),
                        _ => {}
                    }
                }
            }
            GameState::GameOver => {
                if let Some(keycode) = input.keycode {
                    if keycode == KeyCode::R {
                        self.save_and_return_to_menu(ctx);
                    }
                }
            }
        }
        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut Context, character: char) -> GameResult {
        if matches!(self.state, GameState::Menu) {
            // Only allow alphanumeric characters and limit length
            if character.is_alphanumeric() && self.player_name.len() < 20 {
                self.player_name.push(character);
            }
        }
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        if matches!(self.state, GameState::Menu) && button == ggez::event::MouseButton::Left {
            // Check if click is on start button
            let button_rect = ggez::graphics::Rect::new(
                SCREEN_WIDTH / 2.0 - 100.0,
                660.0,
                200.0,
                50.0,
            );

            if button_rect.contains([x, y]) {
                self.start_game();
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemy_speed_increase() {
        let initial_speed = INITIAL_ENEMY_SPEED;
        let wave_2_speed = initial_speed + SPEED_INCREASE_PER_WAVE;
        let wave_3_speed = wave_2_speed + SPEED_INCREASE_PER_WAVE;

        assert_eq!(wave_2_speed, 170.0);
        assert_eq!(wave_3_speed, 190.0);
    }

    #[test]
    fn test_score_calculation() {
        let enemies_destroyed = 5;
        let score = enemies_destroyed * POINTS_PER_ENEMY;
        assert_eq!(score, 50);
    }
}
