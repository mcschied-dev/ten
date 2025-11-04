//! Game systems modules.
//!
//! Contains pure game logic functions for collision detection and wave generation.

pub mod collision;
pub mod wave;

pub use collision::process_collisions;
pub use wave::generate_wave;
