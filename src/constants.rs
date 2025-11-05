//! Game constants and configuration values.
//!
//! This module contains all the tunable constants for game balance,
//! screen dimensions, and physics parameters.

/// Screen width in pixels
pub const SCREEN_WIDTH: f32 = 1024.0;

/// Screen height in pixels
pub const SCREEN_HEIGHT: f32 = 575.0;

/// Player movement speed in pixels per second
pub const PLAYER_SPEED: f32 = 300.0;

/// Bullet movement speed in pixels per second
pub const BULLET_SPEED: f32 = 700.0;

/// Initial enemy movement speed in pixels per second (wave 1)
pub const INITIAL_ENEMY_SPEED: f32 = 150.0;

/// Distance from bottom of screen where enemies trigger game over
pub const DEFENDER_LINE: f32 = 100.0;

/// Collision detection radius in pixels
pub const COLLISION_RADIUS: f32 = 20.0;

/// Points awarded for destroying one enemy
pub const POINTS_PER_ENEMY: u32 = 10;

/// Enemy speed increase per wave in pixels per second
pub const SPEED_INCREASE_PER_WAVE: f32 = 20.0;

/// Player base width increase per wave in pixels
pub const BASE_WIDTH_INCREASE: f32 = 20.0;
