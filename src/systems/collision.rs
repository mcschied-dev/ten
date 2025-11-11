//! Collision detection system.

use crate::constants::COLLISION_RADIUS;
use crate::entities::{Bullet, Enemy};

const COLLISION_RADIUS_SQ: f32 = COLLISION_RADIUS * COLLISION_RADIUS;

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
    dx * dx + dy * dy < COLLISION_RADIUS_SQ
}

/// Process collisions between bullets and enemies.
///
/// Damages enemies hit by bullets (reduces health), removes bullets that hit,
/// and returns positions and points for destroyed enemies.
///
/// # Arguments
///
/// * `enemies` - Mutable vector of enemies to check
/// * `bullets` - Mutable vector of bullets to check against
///
/// # Arguments
///
/// * `destroyed_info` - Scratch buffer that will be filled with (x, y, points)
pub fn process_collisions(
    enemies: &mut Vec<Enemy>,
    bullets: &mut Vec<Bullet>,
    destroyed_info: &mut Vec<(f32, f32, u32)>,
) {
    destroyed_info.clear();
    let initial_enemy_count = enemies.len();
    let initial_bullet_count = bullets.len();

    let mut bullet_idx = 0;
    while bullet_idx < bullets.len() {
        let mut bullet_hit = false;

        for enemy in enemies.iter_mut() {
            if enemy.is_destroyed() {
                continue;
            }

            if check_collision(&bullets[bullet_idx], enemy) {
                let destroyed = enemy.take_damage();
                bullet_hit = true;

                if destroyed {
                    destroyed_info.push((enemy.x, enemy.y, enemy.enemy_type.points()));
                }
                break;
            }
        }

        if bullet_hit {
            bullets.swap_remove(bullet_idx);
        } else {
            bullet_idx += 1;
        }
    }

    enemies.retain(|enemy| !enemy.is_destroyed());

    let removed_enemies = initial_enemy_count - enemies.len();
    let removed_bullets = initial_bullet_count - bullets.len();

    if removed_enemies > 0 || removed_bullets > 0 {
        log::debug!(
            "Destroyed {} enemies and removed {} bullets in collision check",
            removed_enemies,
            removed_bullets
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::entities::EnemyType;

    #[test]
    fn test_bullet_collision_detection() {
        let enemy = Enemy::new(100.0, 200.0, 1.0, EnemyType::Standard);
        let bullet = Bullet::new(105.0, 205.0);
        assert!(check_collision(&bullet, &enemy));
    }

    #[test]
    fn test_bullet_no_collision() {
        let enemy = Enemy::new(100.0, 200.0, 1.0, EnemyType::Standard);
        let bullet = Bullet::new(150.0, 250.0);
        assert!(!check_collision(&bullet, &enemy));
    }

    #[test]
    fn test_process_collisions() {
        let mut enemies = vec![
            Enemy::new(100.0, 200.0, 1.0, EnemyType::Standard),
            Enemy::new(200.0, 200.0, 1.0, EnemyType::Standard),
            Enemy::new(300.0, 200.0, 1.0, EnemyType::Standard),
        ];
        let mut bullets = vec![
            Bullet::new(105.0, 205.0), // Should hit first enemy
        ];
        let mut destroyed_info = Vec::new();

        process_collisions(&mut enemies, &mut bullets, &mut destroyed_info);
        assert_eq!(destroyed_info.len(), 1);
        assert_eq!(enemies.len(), 2);
        assert_eq!(bullets.len(), 0); // Bullet should be removed
        assert_eq!(destroyed_info[0], (100.0, 200.0, 10)); // x, y, points
    }

    #[test]
    fn test_multiple_bullets_one_enemy() {
        let mut enemies = vec![Enemy::new(100.0, 200.0, 1.0, EnemyType::Standard)];
        let mut bullets = vec![
            Bullet::new(95.0, 195.0),  // Should hit
            Bullet::new(105.0, 205.0), // Would also hit, but enemy removed by first bullet
        ];
        let mut destroyed_info = Vec::new();

        process_collisions(&mut enemies, &mut bullets, &mut destroyed_info);
        assert_eq!(destroyed_info.len(), 1); // Only one enemy destroyed
        assert_eq!(enemies.len(), 0);
        assert_eq!(bullets.len(), 1); // Only one bullet removed (first hit)
    }

    #[test]
    fn test_tank_multi_hit() {
        let mut enemies = vec![Enemy::new(100.0, 200.0, 1.0, EnemyType::Tank)];
        let mut bullets = vec![
            Bullet::new(105.0, 205.0), // First hit
        ];
        let mut destroyed_info = Vec::new();

        // First hit - Tank takes damage but survives (3 -> 2 health)
        process_collisions(&mut enemies, &mut bullets, &mut destroyed_info);
        assert_eq!(destroyed_info.len(), 0); // Not destroyed yet
        assert_eq!(enemies.len(), 1); // Still alive
        assert_eq!(enemies[0].health, 2); // Health reduced
        assert_eq!(bullets.len(), 0); // Bullet consumed

        // Second hit
        bullets.push(Bullet::new(105.0, 205.0));
        process_collisions(&mut enemies, &mut bullets, &mut destroyed_info);
        assert_eq!(destroyed_info.len(), 0); // Still not destroyed
        assert_eq!(enemies[0].health, 1); // Health reduced again

        // Third hit - Tank destroyed
        bullets.push(Bullet::new(105.0, 205.0));
        process_collisions(&mut enemies, &mut bullets, &mut destroyed_info);
        assert_eq!(destroyed_info.len(), 1); // Now destroyed!
        assert_eq!(enemies.len(), 0);
        assert_eq!(destroyed_info[0].2, 50); // Tank worth 50 points
    }

    #[test]
    fn test_no_collisions() {
        let mut enemies = vec![Enemy::new(100.0, 200.0, 1.0, EnemyType::Standard)];
        let mut bullets = vec![Bullet::new(200.0, 300.0)]; // Far away
        let mut destroyed_info = Vec::new();

        process_collisions(&mut enemies, &mut bullets, &mut destroyed_info);
        assert_eq!(destroyed_info.len(), 0);
        assert_eq!(enemies.len(), 1);
        assert_eq!(bullets.len(), 1); // Bullet should remain
    }

    #[test]
    fn test_empty_inputs() {
        let mut enemies: Vec<Enemy> = vec![];
        let mut bullets: Vec<Bullet> = vec![];
        let mut destroyed_info = Vec::new();

        process_collisions(&mut enemies, &mut bullets, &mut destroyed_info);
        assert_eq!(destroyed_info.len(), 0);
        assert_eq!(enemies.len(), 0);
        assert_eq!(bullets.len(), 0);
    }

    #[test]
    fn test_different_enemy_types_points() {
        let mut enemies = vec![
            Enemy::new(100.0, 200.0, 1.0, EnemyType::Standard), // 10 points
            Enemy::new(200.0, 200.0, 1.0, EnemyType::Fast),     // 20 points
            Enemy::new(300.0, 200.0, 1.0, EnemyType::Swooper),  // 30 points
        ];
        let mut bullets = vec![
            Bullet::new(105.0, 205.0),
            Bullet::new(205.0, 205.0),
            Bullet::new(305.0, 205.0),
        ];

        let mut destroyed_info = Vec::new();
        process_collisions(&mut enemies, &mut bullets, &mut destroyed_info);
        assert_eq!(destroyed_info.len(), 3);

        // Check points are correct
        let points: Vec<u32> = destroyed_info.iter().map(|(_, _, p)| *p).collect();
        assert!(points.contains(&10));
        assert!(points.contains(&20));
        assert!(points.contains(&30));
    }
}
