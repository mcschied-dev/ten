use crate::constants::{DEFENDER_LINE, SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Debug, Clone)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn update(&mut self, direction: f32, speed: f32, dt: f32) {
        self.x += direction * speed * dt;
    }

    pub fn move_down(&mut self, amount: f32) {
        self.y += amount;
    }

    pub fn has_reached_edge(&self) -> bool {
        self.x < 20.0 || self.x > SCREEN_WIDTH - 20.0
    }

    pub fn has_breached_defender_line(&self) -> bool {
        self.y > SCREEN_HEIGHT - DEFENDER_LINE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defender_line_breach() {
        let enemy = Enemy::new(100.0, SCREEN_HEIGHT - DEFENDER_LINE + 10.0);
        assert!(enemy.has_breached_defender_line());
    }

    #[test]
    fn test_no_defender_line_breach() {
        let enemy = Enemy::new(100.0, SCREEN_HEIGHT - DEFENDER_LINE - 10.0);
        assert!(!enemy.has_breached_defender_line());
    }

    #[test]
    fn test_reached_left_edge() {
        let enemy = Enemy::new(15.0, 100.0);
        assert!(enemy.has_reached_edge());
    }

    #[test]
    fn test_reached_right_edge() {
        let enemy = Enemy::new(SCREEN_WIDTH - 15.0, 100.0);
        assert!(enemy.has_reached_edge());
    }
}
