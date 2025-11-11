//! Player entity implementation.

use crate::constants::{BASE_WIDTH_INCREASE, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::entities::Bullet;

/// Represents the player character.
///
/// The player can move horizontally, shoot bullets, and is upgraded
/// with more firepower after completing each wave.
pub struct Player {
    /// X position in pixels
    pub x: f32,
    /// Width of the player's base in pixels
    pub base_width: f32,
    /// Number of bullets fired per shot
    pub available_shots: u32,
}

impl Player {
    /// Create a new player at the default starting position.
    #[must_use]
    pub fn new() -> Self {
        log::debug!("Creating new player");
        Self {
            x: SCREEN_WIDTH / 2.0,
            base_width: 50.0,
            available_shots: 1,
        }
    }

    /// Move player left based on delta time.
    ///
    /// # Arguments
    ///
    /// * `dt` - Delta time in seconds
    /// * `speed` - Player movement speed in pixels per second
    pub fn move_left(&mut self, dt: f32, speed: f32) {
        self.x -= speed * dt;
        self.clamp_position();
    }

    /// Move player right based on delta time.
    ///
    /// # Arguments
    ///
    /// * `dt` - Delta time in seconds
    /// * `speed` - Player movement speed in pixels per second
    pub fn move_right(&mut self, dt: f32, speed: f32) {
        self.x += speed * dt;
        self.clamp_position();
    }

    /// Clamp player position to screen boundaries.
    pub fn clamp_position(&mut self) {
        self.x = self
            .x
            .clamp(self.base_width / 2.0, SCREEN_WIDTH - self.base_width / 2.0);
    }

    /// Fire bullets based on current upgrade level.
    ///
    /// Push bullets into the provided buffer, avoiding per-shot allocations.
    ///
    /// # Arguments
    ///
    /// * `out` - Buffer receiving the bullets created this frame
    pub fn shoot(&self, out: &mut Vec<Bullet>) {
        let start_len = out.len();
        out.reserve(self.available_shots as usize);
        let offset = self.base_width / (self.available_shots + 1) as f32;

        for i in 0..self.available_shots {
            let bullet_x = self.x - self.base_width / 2.0 + offset * (i as f32 + 1.0);
            out.push(Bullet::new(bullet_x, SCREEN_HEIGHT - 50.0));
        }

        let spawned = out.len() - start_len;
        log::debug!("Player fired {} bullets", spawned);
    }

    /// Upgrade player with more shots and wider base.
    /// Caps at maximum of 3 shots to prevent excessive growth.
    pub fn upgrade(&mut self) {
        if self.available_shots < 3 {
            self.available_shots += 1;
            self.base_width += BASE_WIDTH_INCREASE;
            log::info!(
                "Player upgraded: {} shots, width {}",
                self.available_shots,
                self.base_width
            );
        } else {
            log::info!(
                "Player already at maximum: {} shots, width {}",
                self.available_shots,
                self.base_width
            );
        }
    }

    /// Reset player to initial state.
    pub fn reset(&mut self) {
        log::debug!("Resetting player to initial state");
        self.x = SCREEN_WIDTH / 2.0;
        self.base_width = 50.0;
        self.available_shots = 1;
    }

    /// Get player Y position.
    #[must_use]
    pub const fn y(&self) -> f32 {
        SCREEN_HEIGHT - 50.0
    }

    /// Get player height.
    #[must_use]
    pub const fn height(&self) -> f32 {
        20.0
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_clamping_left_boundary() {
        let mut player = Player::new();
        player.x = -10.0;
        player.clamp_position();
        assert_eq!(player.x, player.base_width / 2.0);
    }

    #[test]
    fn test_player_clamping_right_boundary() {
        let mut player = Player::new();
        player.x = SCREEN_WIDTH + 100.0;
        player.clamp_position();
        assert_eq!(player.x, SCREEN_WIDTH - player.base_width / 2.0);
    }

    #[test]
    fn test_player_within_boundaries() {
        let mut player = Player::new();
        let original_x = player.x;
        player.clamp_position();
        assert_eq!(player.x, original_x);
    }

    #[test]
    fn test_available_shots_increase() {
        let mut player = Player::new();
        assert_eq!(player.available_shots, 1);

        // Test upgrades up to the limit of 3 shots
        for i in 2..=3 {
            player.upgrade();
            assert_eq!(player.available_shots, i);
        }

        // Test that further upgrades don't increase beyond 3
        player.upgrade(); // 4th upgrade - should not increase
        assert_eq!(player.available_shots, 3);
        player.upgrade(); // 5th upgrade - should not increase
        assert_eq!(player.available_shots, 3);
    }

    #[test]
    fn test_base_width_increase() {
        let mut player = Player::new();
        let initial_width = player.base_width;

        // Test width increases up to 3 upgrades (2 increases from initial)
        for i in 1..=2 {
            player.upgrade();
            assert_eq!(
                player.base_width,
                initial_width + (i as f32 * crate::constants::BASE_WIDTH_INCREASE)
            );
        }

        // Test that further upgrades don't increase width beyond the maximum
        let max_width = player.base_width;
        player.upgrade(); // 3rd upgrade - should not increase
        assert_eq!(player.base_width, max_width);
        player.upgrade(); // 4th upgrade - should not increase
        assert_eq!(player.base_width, max_width);
    }

    #[test]
    fn test_player_reset() {
        let mut player = Player::new();
        player.x = 200.0;
        player.base_width = 100.0;
        player.available_shots = 5;

        player.reset();

        assert_eq!(player.x, crate::constants::SCREEN_WIDTH / 2.0);
        assert_eq!(player.base_width, 50.0);
        assert_eq!(player.available_shots, 1);
    }

    #[test]
    fn test_player_shoot_multiple_shots() {
        let mut player = Player::new();
        player.upgrade(); // Now has 2 shots
        player.upgrade(); // Now has 3 shots

        let mut bullets = Vec::new();
        player.shoot(&mut bullets);
        assert_eq!(bullets.len(), 3);

        // Check bullet positions are spread across player width
        assert_eq!(
            bullets[0].x,
            player.x - player.base_width / 2.0 + player.base_width / 4.0
        );
        assert_eq!(bullets[1].x, player.x);
        assert_eq!(
            bullets[2].x,
            player.x + player.base_width / 2.0 - player.base_width / 4.0
        );
    }

    #[test]
    fn test_player_position_clamping_extremes() {
        let mut player = Player::new();

        // Test extreme left
        player.x = -1000.0;
        player.clamp_position();
        assert_eq!(player.x, player.base_width / 2.0);

        // Test extreme right
        player.x = 2000.0;
        player.clamp_position();
        assert_eq!(
            player.x,
            crate::constants::SCREEN_WIDTH - player.base_width / 2.0
        );
    }
}
