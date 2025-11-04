use crate::constants::COLLISION_RADIUS;
use crate::entities::{Bullet, Enemy};

/// Check if a bullet collides with an enemy
pub fn check_collision(bullet: &Bullet, enemy: &Enemy) -> bool {
    let dx = enemy.x - bullet.x;
    let dy = enemy.y - bullet.y;
    (dx * dx + dy * dy).sqrt() < COLLISION_RADIUS
}

/// Remove enemies that have been hit by bullets and return the count
pub fn process_collisions(enemies: &mut Vec<Enemy>, bullets: &[Bullet]) -> usize {
    let initial_count = enemies.len();

    enemies.retain(|enemy| {
        !bullets.iter().any(|bullet| check_collision(bullet, enemy))
    });

    initial_count - enemies.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bullet_collision_detection() {
        let enemy = Enemy::new(100.0, 200.0);
        let bullet = Bullet::new(105.0, 205.0);
        assert!(check_collision(&bullet, &enemy));
    }

    #[test]
    fn test_bullet_no_collision() {
        let enemy = Enemy::new(100.0, 200.0);
        let bullet = Bullet::new(150.0, 250.0);
        assert!(!check_collision(&bullet, &enemy));
    }

    #[test]
    fn test_process_collisions() {
        let mut enemies = vec![
            Enemy::new(100.0, 200.0),
            Enemy::new(200.0, 200.0),
            Enemy::new(300.0, 200.0),
        ];
        let bullets = vec![
            Bullet::new(105.0, 205.0), // Should hit first enemy
        ];

        let destroyed = process_collisions(&mut enemies, &bullets);
        assert_eq!(destroyed, 1);
        assert_eq!(enemies.len(), 2);
    }
}
