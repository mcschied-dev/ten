use crate::constants::BULLET_SPEED;

#[derive(Debug, Clone)]
pub struct Bullet {
    pub x: f32,
    pub y: f32,
}

impl Bullet {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn update(&mut self, dt: f32) {
        self.y -= BULLET_SPEED * dt;
    }

    pub fn is_out_of_bounds(&self) -> bool {
        self.y < 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bullet_out_of_bounds() {
        let bullet = Bullet::new(100.0, -10.0);
        assert!(bullet.is_out_of_bounds());
    }

    #[test]
    fn test_bullet_in_bounds() {
        let bullet = Bullet::new(100.0, 100.0);
        assert!(!bullet.is_out_of_bounds());
    }
}
