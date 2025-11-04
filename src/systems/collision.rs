//! Collision detection system.

use crate::constants::COLLISION_RADIUS;
use crate::entities::{Bullet, Enemy};

/// Check if a bullet collides with an enemy using circle collision.
///
/// # Arguments
///
/// * `bullet` - The bullet to check
/// * `enemy` - The enemy to check
///
/// # Returns
///
/// `true` if the distance between bullet and enemy is less than the collision radius
#[must_use]
pub fn check_collision(bullet: &Bullet, enemy: &Enemy) -> bool {
    let dx = enemy.x - bullet.x;
    let dy = enemy.y - bullet.y;
    (dx * dx + dy * dy).sqrt() < COLLISION_RADIUS
}

/// Process collisions between bullets and enemies.
///
/// Removes all enemies hit by bullets and returns the count of destroyed enemies.
///
/// # Arguments
///
/// * `enemies` - Mutable vector of enemies to check
/// * `bullets` - Slice of bullets to check against
///
/// # Returns
///
/// The number of enemies destroyed
pub fn process_collisions(enemies: &mut Vec<Enemy>, bullets: &[Bullet]) -> usize {
    let initial_count = enemies.len();

    enemies.retain(|enemy| !bullets.iter().any(|bullet| check_collision(bullet, enemy)));

    let destroyed = initial_count - enemies.len();
    if destroyed > 0 {
        log::debug!("Destroyed {} enemies in collision check", destroyed);
    }

    destroyed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bullet_collision_detection() {
        let enemy = Enemy::new(100.0, 200.0, 1.0);
        let bullet = Bullet::new(105.0, 205.0);
        assert!(check_collision(&bullet, &enemy));
    }

    #[test]
    fn test_bullet_no_collision() {
        let enemy = Enemy::new(100.0, 200.0, 1.0);
        let bullet = Bullet::new(150.0, 250.0);
        assert!(!check_collision(&bullet, &enemy));
    }

    #[test]
    fn test_process_collisions() {
        let mut enemies = vec![
            Enemy::new(100.0, 200.0, 1.0),
            Enemy::new(200.0, 200.0, 1.0),
            Enemy::new(300.0, 200.0, 1.0),
        ];
        let bullets = vec![
            Bullet::new(105.0, 205.0), // Should hit first enemy
        ];

        let destroyed = process_collisions(&mut enemies, &bullets);
        assert_eq!(destroyed, 1);
        assert_eq!(enemies.len(), 2);
    }

    #[test]
    fn test_multiple_bullets_one_enemy() {
        let mut enemies = vec![Enemy::new(100.0, 200.0, 1.0)];
        let bullets = vec![
            Bullet::new(95.0, 195.0),  // Should hit
            Bullet::new(105.0, 205.0), // Should also hit (but enemy already destroyed)
        ];

        let destroyed = process_collisions(&mut enemies, &bullets);
        assert_eq!(destroyed, 1); // Only one enemy destroyed
        assert_eq!(enemies.len(), 0);
    }

    #[test]
    fn test_no_collisions() {
        let mut enemies = vec![Enemy::new(100.0, 200.0, 1.0)];
        let bullets = vec![Bullet::new(200.0, 300.0)]; // Far away

        let destroyed = process_collisions(&mut enemies, &bullets);
        assert_eq!(destroyed, 0);
        assert_eq!(enemies.len(), 1);
    }

    #[test]
    fn test_empty_inputs() {
        let mut enemies: Vec<Enemy> = vec![];
        let bullets: Vec<Bullet> = vec![];

        let destroyed = process_collisions(&mut enemies, &bullets);
        assert_eq!(destroyed, 0);
        assert_eq!(enemies.len(), 0);
    }
}
