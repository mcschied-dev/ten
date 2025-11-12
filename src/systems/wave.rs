//! Wave generation system.

use crate::constants::SCREEN_WIDTH;
use crate::entities::{Enemy, EnemyType};

#[cfg(not(target_arch = "wasm32"))]
use rand::{rngs::SmallRng, Rng, SeedableRng};

#[cfg(target_arch = "wasm32")]
use macroquad::rand::gen_range;

/// Formation pattern types.
#[derive(Debug, Clone, Copy)]
enum FormationType {
    /// Classic grid formation (Space Invaders style)
    Grid,
    /// V-shaped formation
    VShape,
    /// Diamond formation
    Diamond,
    /// Scattered random formation
    Scattered,
}

/// Get enemy type based on row position (for variety).
fn get_enemy_type_for_row(row: usize, wave: u32) -> EnemyType {
    match row {
        0 => {
            // Top row: Fast enemies appear from wave 2+
            if wave >= 2 {
                EnemyType::Fast
            } else {
                EnemyType::Standard
            }
        }
        1 | 2 => EnemyType::Standard,
        3 => {
            // Row 3: Tank enemies appear from wave 3+
            if wave >= 3 {
                EnemyType::Tank
            } else {
                EnemyType::Standard
            }
        }
        _ => {
            // Bottom rows: Swoopers appear from wave 4+
            if wave >= 4 && row == 4 {
                EnemyType::Swooper
            } else {
                EnemyType::Standard
            }
        }
    }
}

/// Generate a classic grid formation.
fn generate_grid_formation(wave: u32) -> Vec<Enemy> {
    let rows = 5;
    let columns = 10;
    let mut enemies = Vec::with_capacity(rows * columns);

    let formation_width = (columns - 1) as f32 * 60.0;
    let start_x = (SCREEN_WIDTH - formation_width) / 2.0;
    let start_y = 50.0;

    for i in 0..columns {
        for j in 0..rows {
            let direction = if j % 2 == 0 { 1.0 } else { -1.0 };
            let enemy_type = get_enemy_type_for_row(j, wave);
            enemies.push(Enemy::new(
                start_x + i as f32 * 60.0,
                start_y + j as f32 * 50.0,
                direction,
                enemy_type,
            ));
        }
    }

    enemies
}

/// Generate a V-shaped formation.
fn generate_v_formation(wave: u32) -> Vec<Enemy> {
    let mut enemies = Vec::new();
    let center_x = SCREEN_WIDTH / 2.0;
    let start_y = 80.0;

    // Create V shape with 7 rows
    for row in 0..7 {
        let offset = (row as f32) * 40.0; // Width increases as we go down
        let y = start_y + row as f32 * 45.0;
        let enemy_type = get_enemy_type_for_row(row, wave);

        // Left arm of V
        for i in 0..=row {
            enemies.push(Enemy::new(
                center_x - offset - i as f32 * 55.0,
                y,
                -1.0,
                enemy_type,
            ));
        }

        // Right arm of V
        for i in 0..=row {
            enemies.push(Enemy::new(
                center_x + offset + i as f32 * 55.0,
                y,
                1.0,
                enemy_type,
            ));
        }
    }

    enemies
}

/// Generate a diamond formation.
fn generate_diamond_formation(wave: u32) -> Vec<Enemy> {
    let mut enemies = Vec::new();
    let center_x = SCREEN_WIDTH / 2.0;
    let center_y = 200.0;

    // Diamond: expanding then contracting rows
    let rows = vec![1, 3, 5, 7, 5, 3, 1]; // Creates diamond shape

    for (row_idx, &count) in rows.iter().enumerate() {
        let y = center_y - 100.0 + row_idx as f32 * 40.0;
        let spacing = 60.0;
        let row_width = (count - 1) as f32 * spacing;
        let start_x = center_x - row_width / 2.0;
        let enemy_type = get_enemy_type_for_row(row_idx, wave);

        for i in 0..count {
            let direction = if row_idx % 2 == 0 { 1.0 } else { -1.0 };
            enemies.push(Enemy::new(
                start_x + i as f32 * spacing,
                y,
                direction,
                enemy_type,
            ));
        }
    }

    enemies
}

/// Generate a scattered formation (Desktop version with seeded RNG).
#[cfg(not(target_arch = "wasm32"))]
fn generate_scattered_formation(wave: u32) -> Vec<Enemy> {
    let mut enemies = Vec::new();
    let enemy_count = 35;

    let mut rng = SmallRng::seed_from_u64(wave as u64);
    let x_min = 100.0;
    let x_max = SCREEN_WIDTH - 100.0;

    for i in 0..enemy_count {
        let x = rng.gen_range(x_min..x_max);
        let y = rng.gen_range(60.0..260.0);
        let direction = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

        // Mix of enemy types
        let enemy_type = match i % 7 {
            0 if wave >= 2 => EnemyType::Fast,
            1 | 2 if wave >= 3 => EnemyType::Tank,
            3 if wave >= 4 => EnemyType::Swooper,
            _ => EnemyType::Standard,
        };

        enemies.push(Enemy::new(x, y, direction, enemy_type));
    }

    enemies
}

/// Generate a scattered formation (WASM version using macroquad rand).
#[cfg(target_arch = "wasm32")]
fn generate_scattered_formation(wave: u32) -> Vec<Enemy> {
    let mut enemies = Vec::new();
    let enemy_count = 35;

    let x_min = 100.0;
    let x_max = SCREEN_WIDTH - 100.0;

    for i in 0..enemy_count {
        let x = gen_range(x_min, x_max);
        let y = gen_range(60.0, 260.0);
        let direction = if gen_range(0.0, 1.0) < 0.5 { 1.0 } else { -1.0 };

        // Mix of enemy types
        let enemy_type = match i % 7 {
            0 if wave >= 2 => EnemyType::Fast,
            1 | 2 if wave >= 3 => EnemyType::Tank,
            3 if wave >= 4 => EnemyType::Swooper,
            _ => EnemyType::Standard,
        };

        enemies.push(Enemy::new(x, y, direction, enemy_type));
    }

    enemies
}

/// Generate enemies for a given wave number with varied formations and enemy types.
///
/// Uses different formations per wave:
/// - Wave 1, 5, 9, ...: Classic grid
/// - Wave 2, 6, 10, ...: V-shape
/// - Wave 3, 7, 11, ...: Diamond
/// - Wave 4, 8, 12, ...: Scattered
///
/// Enemy types introduced progressively:
/// - Wave 1: Standard only
/// - Wave 2+: Fast enemies added
/// - Wave 3+: Tank enemies added
/// - Wave 4+: Swooper enemies added
///
/// # Arguments
///
/// * `wave` - The wave number (1-based)
///
/// # Returns
///
/// A vector of enemies positioned according to the wave's formation pattern
#[must_use]
pub fn generate_wave(wave: u32) -> Vec<Enemy> {
    let formation = match wave % 4 {
        1 => FormationType::Grid,
        2 => FormationType::VShape,
        3 => FormationType::Diamond,
        0 => FormationType::Scattered,
        _ => FormationType::Grid, // Fallback
    };

    let enemies = match formation {
        FormationType::Grid => generate_grid_formation(wave),
        FormationType::VShape => generate_v_formation(wave),
        FormationType::Diamond => generate_diamond_formation(wave),
        FormationType::Scattered => generate_scattered_formation(wave),
    };

    log::info!(
        "Generating wave {} with {} enemies - {:?} formation",
        wave,
        enemies.len(),
        formation
    );

    enemies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_wave_1_grid() {
        let enemies = generate_wave(1);
        // Wave 1: Grid formation (5 rows × 10 columns = 50)
        assert_eq!(enemies.len(), 50);
        // Wave 1 only has Standard enemies
        assert!(enemies.iter().all(|e| e.enemy_type == EnemyType::Standard));
    }

    #[test]
    fn test_generate_wave_2_v_shape() {
        let enemies = generate_wave(2);
        // Wave 2: V-shape formation
        assert!(enemies.len() > 0);
        // Wave 2 introduces Fast enemies
        assert!(enemies.iter().any(|e| e.enemy_type == EnemyType::Fast));
    }

    #[test]
    fn test_generate_wave_3_diamond() {
        let enemies = generate_wave(3);
        // Wave 3: Diamond formation (1+3+5+7+5+3+1 = 25 enemies)
        assert_eq!(enemies.len(), 25);
        // Wave 3 introduces Tank enemies
        assert!(enemies.iter().any(|e| e.enemy_type == EnemyType::Tank));
    }

    #[test]
    fn test_generate_wave_4_scattered() {
        let enemies = generate_wave(4);
        // Wave 4: Scattered formation (35 enemies)
        assert_eq!(enemies.len(), 35);
        // Wave 4 introduces Swooper enemies
        assert!(enemies.iter().any(|e| e.enemy_type == EnemyType::Swooper));
    }

    #[test]
    fn test_enemy_types_progressive() {
        // Wave 1: Standard only
        let wave1 = generate_wave(1);
        assert!(wave1.iter().all(|e| e.enemy_type == EnemyType::Standard));

        // Wave 2: Standard + Fast
        let wave2 = generate_wave(2);
        let types2: Vec<_> = wave2.iter().map(|e| e.enemy_type).collect();
        assert!(types2.contains(&EnemyType::Standard) || types2.contains(&EnemyType::Fast));

        // Wave 3: Can include Tank
        let wave3 = generate_wave(3);
        let types3: Vec<_> = wave3.iter().map(|e| e.enemy_type).collect();
        assert!(
            types3.contains(&EnemyType::Standard)
                || types3.contains(&EnemyType::Tank)
                || types3.contains(&EnemyType::Fast)
        );
    }

    #[test]
    fn test_grid_formation_positions() {
        let enemies = generate_grid_formation(1);
        assert_eq!(enemies.len(), 50);

        // Check first enemy position (top-left)
        assert_eq!(enemies[0].x, 242.0);
        assert_eq!(enemies[0].y, 50.0);

        // Check spacing
        assert_eq!(enemies[1].x, 242.0);
        assert_eq!(enemies[1].y, 100.0);
    }

    #[test]
    fn test_formation_cycle() {
        // Formations cycle: Grid (1), V (2), Diamond (3), Scattered (4), repeat
        let wave5 = generate_wave(5);
        let wave1 = generate_wave(1);
        // Wave 5 should be same formation as wave 1 (Grid)
        assert_eq!(wave5.len(), wave1.len());
    }

    #[test]
    fn test_enemy_health_initialization() {
        let enemies = generate_wave(3);
        for enemy in enemies {
            match enemy.enemy_type {
                EnemyType::Standard | EnemyType::Fast | EnemyType::Swooper => {
                    assert_eq!(enemy.health, 1);
                }
                EnemyType::Tank => {
                    assert_eq!(enemy.health, 3);
                }
            }
        }
    }
}
