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
/// Removes all enemies hit by bullets, removes the bullets that hit, and returns enemy positions for explosion effects.
///
/// # Arguments
///
/// * `enemies` - Mutable vector of enemies to check
/// * `bullets` - Mutable vector of bullets to check against
///
/// # Returns
///
/// A vector of (x, y) positions where enemies were destroyed
pub fn process_collisions(enemies: &mut Vec<Enemy>, bullets: &mut Vec<Bullet>) -> Vec<(f32, f32)> {
    let mut destroyed_positions = Vec::new();
    let mut bullets_to_remove = Vec::new();

    // Find all enemies that were hit and collect their positions
    let mut i = 0;
    while i < enemies.len() {
        let mut hit = false;
        for (bullet_idx, bullet) in bullets.iter().enumerate() {
            if check_collision(bullet, &enemies[i]) {
                // Store the position before removing
                destroyed_positions.push((enemies[i].x, enemies[i].y));
                bullets_to_remove.push(bullet_idx);
                hit = true;
                break; // One bullet per enemy is enough
            }
        }

        if hit {
            enemies.remove(i);
        } else {
            i += 1;
        }
    }

    // Remove bullets that hit enemies (in reverse order to maintain indices)
    bullets_to_remove.sort_unstable();
    bullets_to_remove.dedup();
    for &idx in bullets_to_remove.iter().rev() {
        bullets.remove(idx);
    }

    if !destroyed_positions.is_empty() {
        log::debug!(
            "Destroyed {} enemies and removed {} bullets in collision check",
            destroyed_positions.len(),
            bullets_to_remove.len()
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
        let mut bullets = vec![
            Bullet::new(105.0, 205.0), // Should hit first enemy
        ];

        let destroyed_positions = process_collisions(&mut enemies, &mut bullets);
        assert_eq!(destroyed_positions.len(), 1);
        assert_eq!(enemies.len(), 2);
        assert_eq!(bullets.len(), 0); // Bullet should be removed
        assert_eq!(destroyed_positions[0], (100.0, 200.0));
    }

    #[test]
    fn test_multiple_bullets_one_enemy() {
        let mut enemies = vec![Enemy::new(100.0, 200.0, 1.0)];
        let mut bullets = vec![
            Bullet::new(95.0, 195.0),  // Should hit
            Bullet::new(105.0, 205.0), // Would also hit, but enemy removed by first bullet
        ];

        let destroyed_positions = process_collisions(&mut enemies, &mut bullets);
        assert_eq!(destroyed_positions.len(), 1); // Only one enemy destroyed
        assert_eq!(enemies.len(), 0);
        assert_eq!(bullets.len(), 1); // Only one bullet removed (first hit)
    }

    #[test]
    fn test_no_collisions() {
        let mut enemies = vec![Enemy::new(100.0, 200.0, 1.0)];
        let mut bullets = vec![Bullet::new(200.0, 300.0)]; // Far away

        let destroyed_positions = process_collisions(&mut enemies, &mut bullets);
        assert_eq!(destroyed_positions.len(), 0);
        assert_eq!(enemies.len(), 1);
        assert_eq!(bullets.len(), 1); // Bullet should remain
    }

    #[test]
    fn test_empty_inputs() {
        let mut enemies: Vec<Enemy> = vec![];
        let mut bullets: Vec<Bullet> = vec![];

        let destroyed_positions = process_collisions(&mut enemies, &mut bullets);
        assert_eq!(destroyed_positions.len(), 0);
        assert_eq!(enemies.len(), 0);
        assert_eq!(bullets.len(), 0);
    }
}
