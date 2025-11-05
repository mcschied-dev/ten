//! Explosion animation entity.
//!
//! Displays a short stop-motion animation when enemies are destroyed.

/// Represents an explosion animation with multiple frames
pub struct Explosion {
    /// X position in pixels
    pub x: f32,
    /// Y position in pixels
    pub y: f32,
    /// Current frame index (0, 1, or 2 for 3-frame animation)
    pub current_frame: usize,
    /// Time accumulated for frame switching
    pub frame_timer: f32,
    /// Duration each frame is displayed (in seconds)
    pub frame_duration: f32,
    /// Total number of frames
    pub total_frames: usize,
    /// Whether animation is finished
    pub finished: bool,
}

impl Explosion {
    /// Create a new explosion at the given position
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of explosion center
    /// * `y` - Y coordinate of explosion center
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            current_frame: 0,
            frame_timer: 0.0,
            frame_duration: 0.1, // 100ms per frame = 300ms total animation
            total_frames: 3,
            finished: false,
        }
    }

    /// Update the explosion animation
    ///
    /// # Arguments
    ///
    /// * `dt` - Delta time in seconds
    pub fn update(&mut self, dt: f32) {
        if self.finished {
            return;
        }

        self.frame_timer += dt;

        // Check if it's time to advance to next frame
        if self.frame_timer >= self.frame_duration {
            self.frame_timer = 0.0;
            self.current_frame += 1;

            // Check if animation is complete
            if self.current_frame >= self.total_frames {
                self.finished = true;
            }
        }
    }

    /// Check if the animation is complete
    #[must_use]
    pub const fn is_finished(&self) -> bool {
        self.finished
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explosion_creation() {
        let explosion = Explosion::new(100.0, 200.0);
        assert_eq!(explosion.x, 100.0);
        assert_eq!(explosion.y, 200.0);
        assert_eq!(explosion.current_frame, 0);
        assert!(!explosion.finished);
    }

    #[test]
    fn test_explosion_frame_progression() {
        let mut explosion = Explosion::new(0.0, 0.0);

        // First frame
        assert_eq!(explosion.current_frame, 0);

        // After 0.1 seconds, should be on frame 1
        explosion.update(0.1);
        assert_eq!(explosion.current_frame, 1);
        assert!(!explosion.finished);

        // After another 0.1 seconds, should be on frame 2
        explosion.update(0.1);
        assert_eq!(explosion.current_frame, 2);
        assert!(!explosion.finished);

        // After another 0.1 seconds, should be finished
        explosion.update(0.1);
        assert!(explosion.finished);
    }

    #[test]
    fn test_explosion_doesnt_update_when_finished() {
        let mut explosion = Explosion::new(0.0, 0.0);

        // Complete the animation by updating multiple times
        explosion.update(0.1); // frame 0 -> 1
        explosion.update(0.1); // frame 1 -> 2
        explosion.update(0.1); // frame 2 -> 3 (finished)
        assert!(explosion.finished);
        let frame_before = explosion.current_frame;

        // Try to update again
        explosion.update(0.1);
        assert_eq!(explosion.current_frame, frame_before);
    }
}
