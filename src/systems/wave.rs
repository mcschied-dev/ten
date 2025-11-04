//! Wave generation system.

use crate::constants::SCREEN_WIDTH;
use crate::entities::Enemy;

/// Generate enemies for a given wave number.
///
/// Each wave generates a grid of enemies with progressively more rows.
/// The formula is: rows = 2 + wave_number, with a constant 10 columns.
///
/// # Arguments
///
/// * `wave` - The wave number (1-based)
///
/// # Returns
///
/// A vector of enemies positioned in a grid formation
///
/// # Examples
///
/// ```
/// # use ten::systems::wave::generate_wave;
/// let wave_1 = generate_wave(1);  // 30 enemies (3 rows x 10 columns)
/// let wave_2 = generate_wave(2);  // 40 enemies (4 rows x 10 columns)
/// ```
#[must_use]
pub fn generate_wave(wave: u32) -> Vec<Enemy> {
    let rows = 2 + wave as usize;
    let columns = 10;
    let enemy_count = rows * columns;

    log::info!(
        "Generating wave {} with {} enemies ({} rows x {} columns)",
        wave,
        enemy_count,
        rows,
        columns
    );

    let mut enemies = Vec::with_capacity(enemy_count);

    // Generate alternating directions for each row (-1.0 for left, 1.0 for right)
    // Row 0: right, Row 1: left, Row 2: right, Row 3: left, etc.
    let mut row_directions = Vec::with_capacity(rows);
    for row_idx in 0..rows {
        let direction = if row_idx % 2 == 0 { 1.0 } else { -1.0 };
        row_directions.push(direction);
    }

    // Center the formation horizontally and position at top of screen
    let formation_width = (columns - 1) as f32 * 60.0;
    let start_x = (SCREEN_WIDTH - formation_width) / 2.0;
    let start_y = 50.0; // Position at top of screen

    for i in 0..columns {
        for j in 0..rows {
            let direction = row_directions[j];
            enemies.push(Enemy::new(start_x + i as f32 * 60.0, start_y + j as f32 * 50.0, direction));
        }
    }

    enemies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_enemies_wave_1() {
        let enemies = generate_wave(1);
        // Wave 1 should have 3 rows (2 + 1) and 10 columns
        assert_eq!(enemies.len(), 30);
    }

    #[test]
    fn test_generate_enemies_wave_2() {
        let enemies = generate_wave(2);
        // Wave 2 should have 4 rows (2 + 2) and 10 columns
        assert_eq!(enemies.len(), 40);
    }

    #[test]
    fn test_generate_enemies_positions() {
        let enemies = generate_wave(1);

        // Formation is centered horizontally: (1024.0 - 540.0) / 2.0 = 242.0
        // Positioned at top: y = 50.0

        // Check first enemy position (top-left of formation)
        assert_eq!(enemies[0].x, 242.0);
        assert_eq!(enemies[0].y, 50.0);

        // Check that enemies are spaced properly vertically (same column, next row)
        assert_eq!(enemies[1].x, 242.0);
        assert_eq!(enemies[1].y, 100.0); // 50.0 + 50.0

        // Check enemies are spaced horizontally (next column, first row)
        // Wave 1 has 3 rows, so enemies[3] is first enemy of second column
        assert_eq!(enemies[3].x, 302.0); // 242.0 + 60.0
        assert_eq!(enemies[3].y, 50.0);
    }

    #[test]
    fn test_generate_wave_zero() {
        let enemies = generate_wave(0);
        // Wave 0 should have 2 rows (2 + 0)
        assert_eq!(enemies.len(), 20); // 2 rows × 10 columns
    }

    #[test]
    fn test_generate_wave_high_number() {
        let enemies = generate_wave(10);
        // Wave 10 should have 12 rows (2 + 10)
        assert_eq!(enemies.len(), 120); // 12 rows × 10 columns
    }

    #[test]
    fn test_enemy_positions_wave_2() {
        let enemies = generate_wave(2);

        // Wave 2 has 4 rows
        assert_eq!(enemies.len(), 40);

        // Check positions are still valid (centered at x=242.0, y=50.0)
        assert_eq!(enemies[0].x, 242.0);
        assert_eq!(enemies[0].y, 50.0);

        // Last enemy in first column should be at row 4
        assert_eq!(enemies[3].x, 242.0);
        assert_eq!(enemies[3].y, 200.0); // 50 + 3 * 50
    }
}
