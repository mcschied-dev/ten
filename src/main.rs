// @mcschied
// Space Invaders Game - Modular Architecture

use ggez::event;
use ggez::{ContextBuilder, GameResult};
use std::path::Path;

use ten::{MainState, SCREEN_HEIGHT, SCREEN_WIDTH};

fn main() -> GameResult {
    // Determine resource path based on project root
    let resources_dir = format!("{}/resources", env!("CARGO_MANIFEST_DIR"));

    // Create context
    let (mut ctx, event_loop) = ContextBuilder::new("space_invaders", "Author")
        .window_setup(ggez::conf::WindowSetup::default().title("Hummel Invaders"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;

    // Mount resource directory
    println!("Mounting resources from: {}", resources_dir);
    ctx.fs.mount(Path::new(&resources_dir), true);

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
