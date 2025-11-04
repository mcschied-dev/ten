//! Enemy entity implementation.

use crate::constants::{DEFENDER_LINE, SCREEN_HEIGHT};

/// Represents an enemy in the game.
///
/// Enemies move horizontally across the screen in their own direction,
/// drop down when they hit the edge, and trigger game over if they reach the defender line.
#[derive(Debug, Clone)]
pub struct Enemy {
    /// X position in pixels
    pub x: f32,
    /// Y position in pixels
    pub y: f32,
    /// Movement direction (1.0 = right, -1.0 = left)
    pub direction: f32,
}

impl Enemy {
    /// Create a new enemy at the specified position with a movement direction.
    ///
    /// # Arguments
    ///
    /// * `x` - Initial X coordinate
    /// * `y` - Initial Y coordinate
    /// * `direction` - Movement direction (1.0 = right, -1.0 = left)
    #[must_use]
    pub fn new(x: f32, y: f32, direction: f32) -> Self {
        Self { x, y, direction }
    }

    /// Update enemy position based on speed and delta time.
    /// Uses the enemy's own direction for movement.
    ///
    /// # Arguments
    ///
    /// * `speed` - Movement speed in pixels per second
    /// * `dt` - Delta time in seconds
    pub fn update(&mut self, speed: f32, dt: f32) {
        self.x += self.direction * speed * dt;
    }

    /// Check if enemy has breached the defender line (game over condition).
    #[must_use]
    pub fn has_breached_defender_line(&self) -> bool {
        self.y > SCREEN_HEIGHT - DEFENDER_LINE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defender_line_breach() {
        let enemy = Enemy::new(100.0, SCREEN_HEIGHT - DEFENDER_LINE + 10.0, 1.0);
        assert!(enemy.has_breached_defender_line());
    }

    #[test]
    fn test_no_defender_line_breach() {
        let enemy = Enemy::new(100.0, SCREEN_HEIGHT - DEFENDER_LINE - 10.0, 1.0);
        assert!(!enemy.has_breached_defender_line());
    }
}
