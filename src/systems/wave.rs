use crate::entities::Enemy;

/// Generate enemies for a given wave number
pub fn generate_wave(wave: u32) -> Vec<Enemy> {
    let mut enemies = Vec::new();
    let rows = 2 + wave as usize;

    for i in 0..10 {
        for j in 0..rows {
            enemies.push(Enemy::new(
                50.0 + i as f32 * 60.0,
                100.0 + j as f32 * 50.0,
            ));
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

        // Check first enemy position
        assert_eq!(enemies[0].x, 50.0);
        assert_eq!(enemies[0].y, 100.0);

        // Check that enemies are spaced properly vertically (same column, next row)
        assert_eq!(enemies[1].x, 50.0);
        assert_eq!(enemies[1].y, 150.0); // 100.0 + 50.0

        // Check enemies are spaced horizontally (next column, first row)
        // Wave 1 has 3 rows, so enemies[3] is first enemy of second column
        assert_eq!(enemies[3].x, 110.0); // 50.0 + 60.0
        assert_eq!(enemies[3].y, 100.0);
    }
}
