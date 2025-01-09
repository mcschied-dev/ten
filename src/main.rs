// learning RUST - 

use ggez::audio::{SoundSource, Source};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, Image, Rect, Text, TextFragment, Drawable};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameResult};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// CONST

const SCREEN_WIDTH: f32 = 1024.0;
const SCREEN_HEIGHT: f32 = 768.0;
const PLAYER_SPEED: f32 = 300.0;
const BULLET_SPEED: f32 = 700.0;
const INITIAL_ENEMY_SPEED: f32 = 150.0;
const DEFENDER_LINE: f32 = 100.0;
const TEXT_SCROLL_SPEED: f32 = 100.0;

struct Bullet {
    x: f32,
    y: f32,
}

struct Enemy {
    x: f32,
    y: f32,
}

enum GameState {
    Playing,
    GameOver,
}

struct MainState {
    player_x: f32,
    base_width: f32,        
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    enemy_direction: f32,
    enemy_speed: f32,
    wave_number: u32,
    available_shots: u32,   
    state: GameState,
    scroll_text: Text,
    text_x: Arc<Mutex<f32>>,
    shoot_sound: Source,
    hit_sound: Source,
    background: Image,
    score: u32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let shoot_sound = Source::new(ctx, "/shoot.wav")?;
        let hit_sound = Source::new(ctx, "/hit.wav")?;
        let background = Image::from_path(ctx, "/background.png")?;

        let scroll_text = Text::new(TextFragment {
            text: "Happy New Year".to_string(),
            color: Some(Color::from_rgb(255, 255, 255)),
            scale: Some(graphics::PxScale::from(30.0)),
            ..Default::default()
        });

        let text_x = Arc::new(Mutex::new(SCREEN_WIDTH));

        // Hintergrund-Thread für die Laufschrift
        let text_x_clone = Arc::clone(&text_x);
        thread::spawn(move || {
            loop {
                {
                    let mut position = text_x_clone.lock().unwrap();
                    *position -= TEXT_SCROLL_SPEED * 0.016;
                    if *position < -500.0 {
                        *position = SCREEN_WIDTH;
                    }
                }
                thread::sleep(Duration::from_millis(16));
            }
        });

        Ok(MainState {
            player_x: SCREEN_WIDTH / 2.0,
            base_width: 50.0, 
            bullets: Vec::new(),
            enemies: MainState::generate_enemies(1),
            enemy_direction: 1.0,
            enemy_speed: INITIAL_ENEMY_SPEED,
            wave_number: 1,
            available_shots: 1, 
            state: GameState::Playing,
            scroll_text,
            text_x,
            shoot_sound,
            hit_sound,
            background,
            score: 0,
        })
    }

    fn reset(&mut self) {
        self.player_x = SCREEN_WIDTH / 2.0;
        self.base_width = 50.0; 
        self.bullets.clear();
        self.enemies = MainState::generate_enemies(1);
        self.enemy_direction = 1.0;
        self.enemy_speed = INITIAL_ENEMY_SPEED;
        self.wave_number = 1;
        self.available_shots = 1; 
        self.score = 0;

        let mut text_x = self.text_x.lock().unwrap();
        *text_x = SCREEN_WIDTH;

        self.state = GameState::Playing;
    }

    fn generate_enemies(wave: u32) -> Vec<Enemy> {
        let mut enemies = Vec::new();
        let rows = 2 + wave as usize;
        for i in 0..10 {
            for j in 0..rows {
                enemies.push(Enemy {
                    x: 50.0 + i as f32 * 60.0,
                    y: 100.0 + j as f32 * 50.0,
                });
            }
        }
        enemies
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if matches!(self.state, GameState::GameOver) {
            return Ok(());
        }

        let dt = ctx.time.delta().as_secs_f32();

        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.player_x -= PLAYER_SPEED * dt;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.player_x += PLAYER_SPEED * dt;
        }

        self.player_x = self.player_x.clamp(self.base_width / 2.0, SCREEN_WIDTH - self.base_width / 2.0);

        for bullet in &mut self.bullets {
            bullet.y -= BULLET_SPEED * dt;
        }
        self.bullets.retain(|bullet| bullet.y > 0.0);

        let mut reached_edge = false;
        for enemy in &mut self.enemies {
            enemy.x += self.enemy_direction * self.enemy_speed * dt;
            if enemy.x < 20.0 || enemy.x > SCREEN_WIDTH - 20.0 {
                reached_edge = true;
            }
        }

        if reached_edge {
            self.enemy_direction *= -1.0;
            for enemy in &mut self.enemies {
                enemy.y += 40.0;
            }
        }

        let initial_enemy_count = self.enemies.len();

        self.enemies.retain(|enemy| {
            !self.bullets.iter().any(|bullet| {
                let dx = enemy.x - bullet.x;
                let dy = enemy.y - bullet.y;
                (dx * dx + dy * dy).sqrt() < 20.0
            })
        });

        let enemies_destroyed = initial_enemy_count - self.enemies.len();
        self.score += enemies_destroyed as u32 * 10;

        if self.enemies.is_empty() {
            self.wave_number += 1;
            self.enemy_speed += 20.0;
            self.available_shots += 1; 
            self.base_width += 20.0; 
            self.enemies = MainState::generate_enemies(self.wave_number);
        }

        if self.enemies.iter().any(|enemy| enemy.y > SCREEN_HEIGHT - DEFENDER_LINE) {
            self.state = GameState::GameOver;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        canvas.draw(&self.background, graphics::DrawParam::default());

        let text_x = *self.text_x.lock().unwrap();
        let text_position = graphics::DrawParam::default().dest([text_x, 20.0]);
        canvas.draw(&self.scroll_text, text_position);

        // Basisstation zeichnen
        let player_rect = Rect::new(
            self.player_x - self.base_width / 2.0,
            SCREEN_HEIGHT - 50.0,
            self.base_width,
            20.0,
        );
        let player_color = Color::from_rgb(0, 255, 0);
        let player_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            player_rect,
            player_color,
        )?;
        canvas.draw(&player_mesh, graphics::DrawParam::default());

        // Schüsse zeichnen
        for bullet in &self.bullets {
            let bullet_rect = Rect::new(bullet.x - 5.0, bullet.y - 10.0, 10.0, 20.0);
            let bullet_color = Color::WHITE;
            let bullet_mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                bullet_rect,
                bullet_color,
            )?;
            canvas.draw(&bullet_mesh, graphics::DrawParam::default());
        }

        // Feinde zeichnen
        for enemy in &self.enemies {
            let enemy_rect = Rect::new(enemy.x - 20.0, enemy.y - 20.0, 40.0, 40.0);
            let enemy_color = Color::from_rgb(255, 0, 0);
            let enemy_mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                enemy_rect,
                enemy_color,
            )?;
            canvas.draw(&enemy_mesh, graphics::DrawParam::default());
        }

        if matches!(self.state, GameState::GameOver) {
            let game_over_text = Text::new(TextFragment {
                text: "Game Over - Press R to Restart".to_string(),
                color: Some(Color::from_rgb(255, 255, 255)),
                scale: Some(graphics::PxScale::from(40.0)),
                ..Default::default()
            });
            canvas.draw(&game_over_text, graphics::DrawParam::default().dest([300.0, 350.0]));
        }

        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeat: bool,
    ) -> GameResult {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::Space => {
                    if matches!(self.state, GameState::Playing) {
                      
                        let offset = self.base_width / (self.available_shots + 1) as f32;
                        for i in 0..self.available_shots {
                            let bullet_x = self.player_x - self.base_width / 2.0 + offset * (i as f32 + 1.0);
                            self.bullets.push(Bullet {
                                x: bullet_x,
                                y: SCREEN_HEIGHT - 50.0,
                            });
                        }
                        let _ = self.shoot_sound.play(ctx);
                    }
                }
                KeyCode::R => self.reset(),
                _ => {}
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("space_invaders", "Author")
        .window_setup(ggez::conf::WindowSetup::default().title("Space Invaders"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .add_resource_path("./resources")
        .build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}