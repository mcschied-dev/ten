use crate::constants::{PLAYER_SPEED, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::entities::Bullet;

pub struct Player {
    pub x: f32,
    pub base_width: f32,
    pub available_shots: u32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: SCREEN_WIDTH / 2.0,
            base_width: 50.0,
            available_shots: 1,
        }
    }

    pub fn move_left(&mut self, dt: f32) {
        self.x -= PLAYER_SPEED * dt;
        self.clamp_position();
    }

    pub fn move_right(&mut self, dt: f32) {
        self.x += PLAYER_SPEED * dt;
        self.clamp_position();
    }

    pub fn clamp_position(&mut self) {
        self.x = self
            .x
            .clamp(self.base_width / 2.0, SCREEN_WIDTH - self.base_width / 2.0);
    }

    pub fn shoot(&self) -> Vec<Bullet> {
        let mut bullets = Vec::new();
        let offset = self.base_width / (self.available_shots + 1) as f32;

        for i in 0..self.available_shots {
            let bullet_x = self.x - self.base_width / 2.0 + offset * (i as f32 + 1.0);
            bullets.push(Bullet::new(bullet_x, SCREEN_HEIGHT - 50.0));
        }

        bullets
    }

    pub fn upgrade(&mut self) {
        self.available_shots += 1;
        self.base_width += crate::constants::BASE_WIDTH_INCREASE;
    }

    pub fn reset(&mut self) {
        self.x = SCREEN_WIDTH / 2.0;
        self.base_width = 50.0;
        self.available_shots = 1;
    }

    pub fn y(&self) -> f32 {
        SCREEN_HEIGHT - 50.0
    }

    pub fn height(&self) -> f32 {
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
        player.upgrade();
        assert_eq!(player.available_shots, 2);
        player.upgrade();
        assert_eq!(player.available_shots, 3);
    }

    #[test]
    fn test_base_width_increase() {
        let mut player = Player::new();
        let initial_width = player.base_width;
        player.upgrade();
        assert_eq!(player.base_width, initial_width + crate::constants::BASE_WIDTH_INCREASE);
    }
}
