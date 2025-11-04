//! BumbleBees - A Space Invaders Clone
//!
//! This is a classic arcade-style space shooter game built with Rust and macroquad.
//! Features include:
//! - Progressive difficulty with wave-based gameplay
//! - Parallax scrolling background
//! - Highscore tracking with persistent storage
//! - Sound effects and background music

pub mod constants;
pub mod entities;
pub mod highscore;
pub mod systems;

pub use constants::*;
pub use entities::*;
pub use highscore::{HighscoreEntry, HighscoreManager};
pub use systems::*;
