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
/// Removes all enemies hit by bullets and returns their positions for explosion effects.
///
/// # Arguments
///
/// * `enemies` - Mutable vector of enemies to check
/// * `bullets` - Slice of bullets to check against
///
/// # Returns
///
/// A vector of (x, y) positions where enemies were destroyed
pub fn process_collisions(enemies: &mut Vec<Enemy>, bullets: &[Bullet]) -> Vec<(f32, f32)> {
    let mut destroyed_positions = Vec::new();

    // Find all enemies that were hit and collect their positions
    let mut i = 0;
    while i < enemies.len() {
        if bullets
            .iter()
            .any(|bullet| check_collision(bullet, &enemies[i]))
        {
            // Store the position before removing
            destroyed_positions.push((enemies[i].x, enemies[i].y));
            enemies.remove(i);
        } else {
            i += 1;
        }
    }

    if !destroyed_positions.is_empty() {
        log::debug!(
            "Destroyed {} enemies in collision check",
            destroyed_positions.len()
        );
    }

    destroyed_positions
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

        let destroyed_positions = process_collisions(&mut enemies, &bullets);
        assert_eq!(destroyed_positions.len(), 1);
        assert_eq!(enemies.len(), 2);
        assert_eq!(destroyed_positions[0], (100.0, 200.0));
    }

    #[test]
    fn test_multiple_bullets_one_enemy() {
        let mut enemies = vec![Enemy::new(100.0, 200.0, 1.0)];
        let bullets = vec![
            Bullet::new(95.0, 195.0),  // Should hit
            Bullet::new(105.0, 205.0), // Should also hit (but enemy already destroyed)
        ];

        let destroyed_positions = process_collisions(&mut enemies, &bullets);
        assert_eq!(destroyed_positions.len(), 1); // Only one enemy destroyed
        assert_eq!(enemies.len(), 0);
    }

    #[test]
    fn test_no_collisions() {
        let mut enemies = vec![Enemy::new(100.0, 200.0, 1.0)];
        let bullets = vec![Bullet::new(200.0, 300.0)]; // Far away

        let destroyed_positions = process_collisions(&mut enemies, &bullets);
        assert_eq!(destroyed_positions.len(), 0);
        assert_eq!(enemies.len(), 1);
    }

    #[test]
    fn test_empty_inputs() {
        let mut enemies: Vec<Enemy> = vec![];
        let bullets: Vec<Bullet> = vec![];

        let destroyed_positions = process_collisions(&mut enemies, &bullets);
        assert_eq!(destroyed_positions.len(), 0);
        assert_eq!(enemies.len(), 0);
    }
}
