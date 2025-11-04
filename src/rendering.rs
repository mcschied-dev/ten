//! Game rendering module.
//!
//! Handles all drawing operations for different game states:
//! menu, playing, and game over screens.

use ggez::graphics::{self, Canvas, Color, DrawParam, Mesh, PxScale, Rect, Text, TextFragment};
use ggez::{Context, GameResult};

use crate::constants::SCREEN_WIDTH;
use crate::entities::{Bullet, Enemy};
use crate::game_state::{GameState, MainState};

/// Main rendering function that delegates to state-specific renderers.
///
/// # Errors
///
/// Returns an error if any rendering operation fails.
pub fn draw_game(state: &MainState, ctx: &mut Context) -> GameResult {
    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

    match state.state {
        GameState::Menu => {
            draw_menu(&mut canvas, state, ctx)?;
        }
        GameState::Playing => {
            draw_background(&mut canvas, state);
            draw_scroll_text(&mut canvas, state);
            draw_player(&mut canvas, state, ctx)?;
            draw_bullets(&mut canvas, &state.bullets, ctx)?;
            draw_enemies(&mut canvas, &state.enemies, state, ctx)?;
            draw_score(&mut canvas, state.score);
        }
        GameState::GameOver => {
            draw_background(&mut canvas, state);
            draw_game_over(&mut canvas, state.score);
        }
    }

    canvas.finish(ctx)?;
    Ok(())
}

fn draw_background(canvas: &mut Canvas, state: &MainState) {
    let bg_width = state.background.width() as f32;

    // Draw first instance of background
    let bg_param1 = DrawParam::default().dest([state.background_scroll_x, 0.0]);
    canvas.draw(&state.background, bg_param1);

    // Draw second instance for seamless scrolling
    let bg_param2 = DrawParam::default().dest([state.background_scroll_x + bg_width, 0.0]);
    canvas.draw(&state.background, bg_param2);

    // Draw third instance to ensure coverage (in case background is smaller than screen)
    if bg_width < state.background_scroll_x + bg_width * 2.0 {
        let bg_param3 = DrawParam::default().dest([state.background_scroll_x + bg_width * 2.0, 0.0]);
        canvas.draw(&state.background, bg_param3);
    }
}

fn draw_scroll_text(canvas: &mut Canvas, state: &MainState) {
    let text_x = *state.text_x.lock().unwrap();
    let text_position = DrawParam::default().dest([text_x, 20.0]);
    canvas.draw(&state.scroll_text, text_position);
}

fn draw_player(canvas: &mut Canvas, state: &MainState, ctx: &mut Context) -> GameResult {
    let player_rect = Rect::new(
        state.player.x - state.player.base_width / 2.0,
        state.player.y(),
        state.player.base_width,
        state.player.height(),
    );
    let player_color = Color::from_rgb(0, 128, 0);
    let player_mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), player_rect, player_color)?;
    canvas.draw(&player_mesh, DrawParam::default());
    Ok(())
}

fn draw_bullets(canvas: &mut Canvas, bullets: &[Bullet], ctx: &mut Context) -> GameResult {
    for bullet in bullets {
        let bullet_rect = Rect::new(bullet.x - 5.0, bullet.y - 10.0, 10.0, 20.0);
        let bullet_color = Color::WHITE;
        let bullet_mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), bullet_rect, bullet_color)?;
        canvas.draw(&bullet_mesh, DrawParam::default());
    }
    Ok(())
}

fn draw_enemies(canvas: &mut Canvas, enemies: &[Enemy], state: &MainState, _ctx: &mut Context) -> GameResult {
    for enemy in enemies {
        let enemy_position = DrawParam::default().dest([enemy.x - 20.0, enemy.y - 20.0]);
        canvas.draw(&state.enemy_image, enemy_position);
    }
    Ok(())
}

fn draw_menu(canvas: &mut Canvas, state: &MainState, ctx: &mut Context) -> GameResult {
    // Title: "BumbleBees"
    let title = Text::new(TextFragment {
        text: "BumbleBees".to_string(),
        color: Some(Color::from_rgb(255, 215, 0)),
        scale: Some(PxScale::from(80.0)),
        ..Default::default()
    });
    canvas.draw(&title, DrawParam::default().dest([SCREEN_WIDTH / 2.0 - 260.0, 50.0]));

    // Highscores section
    let highscore_title = Text::new(TextFragment {
        text: "HIGH SCORES".to_string(),
        color: Some(Color::from_rgb(255, 255, 255)),
        scale: Some(PxScale::from(40.0)),
        ..Default::default()
    });
    canvas.draw(
        &highscore_title,
        DrawParam::default().dest([SCREEN_WIDTH / 2.0 - 150.0, 160.0]),
    );

    // Display top 10 highscores
    let top_scores = state.highscore_manager.get_top_scores(10);
    for (i, entry) in top_scores.iter().enumerate() {
        let score_text = Text::new(TextFragment {
            text: format!("{}. {} - {}", i + 1, entry.name, entry.score),
            color: Some(Color::from_rgb(200, 200, 200)),
            scale: Some(PxScale::from(24.0)),
            ..Default::default()
        });
        canvas.draw(
            &score_text,
            DrawParam::default().dest([SCREEN_WIDTH / 2.0 - 180.0, 220.0 + i as f32 * 30.0]),
        );
    }

    // Name input section
    let name_label = Text::new(TextFragment {
        text: "Enter Your Name:".to_string(),
        color: Some(Color::from_rgb(255, 255, 255)),
        scale: Some(PxScale::from(30.0)),
        ..Default::default()
    });
    canvas.draw(
        &name_label,
        DrawParam::default().dest([SCREEN_WIDTH / 2.0 - 140.0, 560.0]),
    );

    // Name input box
    let input_box = Rect::new(SCREEN_WIDTH / 2.0 - 150.0, 600.0, 300.0, 40.0);
    let input_box_mesh = Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::stroke(2.0),
        input_box,
        Color::WHITE,
    )?;
    canvas.draw(&input_box_mesh, DrawParam::default());

    // Display current input
    let input_text = Text::new(TextFragment {
        text: state.player_name.clone(),
        color: Some(Color::from_rgb(255, 255, 255)),
        scale: Some(PxScale::from(28.0)),
        ..Default::default()
    });
    canvas.draw(
        &input_text,
        DrawParam::default().dest([SCREEN_WIDTH / 2.0 - 140.0, 607.0]),
    );

    // Start button
    let button_rect = Rect::new(SCREEN_WIDTH / 2.0 - 100.0, 660.0, 200.0, 50.0);
    let button_color = if state.player_name.is_empty() {
        Color::from_rgb(100, 100, 100)
    } else {
        Color::from_rgb(0, 200, 0)
    };
    let button_mesh = Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        button_rect,
        button_color,
    )?;
    canvas.draw(&button_mesh, DrawParam::default());

    let button_text = Text::new(TextFragment {
        text: "START GAME".to_string(),
        color: Some(Color::from_rgb(255, 255, 255)),
        scale: Some(PxScale::from(32.0)),
        ..Default::default()
    });
    canvas.draw(
        &button_text,
        DrawParam::default().dest([SCREEN_WIDTH / 2.0 - 85.0, 670.0]),
    );

    Ok(())
}

fn draw_game_over(canvas: &mut Canvas, score: u32) {
    let game_over_text = Text::new(TextFragment {
        text: "GAME OVER".to_string(),
        color: Some(Color::from_rgb(255, 0, 0)),
        scale: Some(PxScale::from(80.0)),
        ..Default::default()
    });
    canvas.draw(
        &game_over_text,
        DrawParam::default().dest([SCREEN_WIDTH / 2.0 - 250.0, 250.0]),
    );

    let score_text = Text::new(TextFragment {
        text: format!("Final Score: {}", score),
        color: Some(Color::from_rgb(255, 255, 255)),
        scale: Some(PxScale::from(50.0)),
        ..Default::default()
    });
    canvas.draw(
        &score_text,
        DrawParam::default().dest([SCREEN_WIDTH / 2.0 - 200.0, 350.0]),
    );

    let restart_text = Text::new(TextFragment {
        text: "Press R to Return to Menu".to_string(),
        color: Some(Color::from_rgb(200, 200, 200)),
        scale: Some(PxScale::from(30.0)),
        ..Default::default()
    });
    canvas.draw(
        &restart_text,
        DrawParam::default().dest([SCREEN_WIDTH / 2.0 - 220.0, 450.0]),
    );
}

fn draw_score(canvas: &mut Canvas, score: u32) {
    let score_text = Text::new(TextFragment {
        text: format!("Score: {}", score),
        color: Some(Color::from_rgb(255, 255, 255)),
        scale: Some(PxScale::from(30.0)),
        ..Default::default()
    });
    canvas.draw(&score_text, DrawParam::default().dest([SCREEN_WIDTH - 200.0, 20.0]));
}
