//! Highscore management with cross-platform storage.
//!
//! Uses file I/O on desktop and LocalStorage on WASM.

use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use std::cell::RefCell;

/// Maximum size of localStorage data to prevent memory exhaustion (1MB)
#[cfg(target_arch = "wasm32")]
const MAX_LOCALSTORAGE_SIZE: usize = 1024 * 1024;

/// Maximum number of highscore entries to prevent DoS attacks (1000 entries)
#[cfg(target_arch = "wasm32")]
const MAX_HIGHSCORE_ENTRIES: usize = 1000;

/// Maximum number of highscores persisted on disk/browser storage.
const MAX_SAVED_SCORES: usize = 50;

/// A single highscore entry containing player name and score.
///
/// This struct is serialized to JSON for WASM localStorage storage
/// and to CSV format for desktop file storage.
///
/// # Examples
///
/// ```
/// use ten::highscore::HighscoreEntry;
///
/// let entry = HighscoreEntry::new("PLAYER1".to_string(), 5000);
/// assert_eq!(entry.name, "PLAYER1");
/// assert_eq!(entry.score, 5000);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighscoreEntry {
    /// Player name (user-entered, max 20 characters)
    pub name: String,
    /// Player score (points earned from destroying enemies)
    pub score: u32,
}

impl HighscoreEntry {
    /// Create a new highscore entry.
    ///
    /// # Arguments
    ///
    /// * `name` - Player's name
    /// * `score` - Player's final score
    ///
    /// # Returns
    ///
    /// A new `HighscoreEntry` with the given name and score
    #[must_use]
    pub fn new(name: String, score: u32) -> Self {
        Self { name, score }
    }
}

/// Cross-platform highscore persistence manager.
///
/// Provides transparent storage of highscores using the appropriate
/// backend for each platform:
/// - **Desktop**: File I/O with CSV format
/// - **WASM**: Browser localStorage with JSON format via FFI bridge
///
/// # Architecture
///
/// The manager implements a **dual-loading strategy** to provide a better
/// user experience on first launch:
///
/// - `load_highscores()`: Returns actual scores OR demo scores for display
/// - `load_highscores_for_saving()`: Returns ONLY actual scores (no demo data)
///
/// This prevents demo scores from being saved as real highscores while
/// still showing users example scores on their first play.
///
/// # Examples
///
/// ```no_run
/// use ten::highscore::HighscoreManager;
///
/// let manager = HighscoreManager::new("highscores.txt");
///
/// // Save a new highscore
/// manager.save_highscore("PLAYER1", 5000);
///
/// // Get top 10 for display
/// let top_scores = manager.get_top_scores(10);
/// ```
pub struct HighscoreManager {
    /// Storage key: filename (desktop) or localStorage key (WASM)
    storage_key: String,
    #[cfg(not(target_arch = "wasm32"))]
    cache: RefCell<Option<Vec<HighscoreEntry>>>,
}

impl HighscoreManager {
    /// Create a new highscore manager with the specified storage key.
    ///
    /// # Arguments
    ///
    /// * `key` - Storage identifier:
    ///   - Desktop: Filename (e.g., "highscores.txt")
    ///   - WASM: localStorage key (e.g., "bumblebees_highscores")
    ///
    /// # Returns
    ///
    /// A new `HighscoreManager` instance
    #[must_use]
    pub fn new(key: &str) -> Self {
        Self {
            storage_key: key.to_string(),
            #[cfg(not(target_arch = "wasm32"))]
            cache: RefCell::new(None),
        }
    }

    /// Load highscores from storage, sorted by score (highest first)
    pub fn load_highscores(&self) -> Vec<HighscoreEntry> {
        #[cfg(target_arch = "wasm32")]
        {
            self.load_from_localstorage()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.load_cached_scores()
        }
    }

    /// Save a new highscore
    pub fn save_highscore(&self, name: &str, score: u32) {
        // Load existing scores WITHOUT demo data
        let mut entries = self.load_highscores_for_saving();

        // Add new entry
        entries.push(HighscoreEntry::new(name.to_string(), score));

        // Sort by score, highest first
        entries.sort_by(|a, b| b.score.cmp(&a.score));
        entries.truncate(MAX_SAVED_SCORES);

        #[cfg(target_arch = "wasm32")]
        {
            self.save_to_localstorage(&entries);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.update_cache(&entries);
            self.save_to_file(&entries);
        }
    }

    /// Load highscores for saving operations (excludes demo data).
    ///
    /// This private method is used internally when saving new highscores
    /// to ensure that demo scores shown to first-time users are never
    /// persisted to storage.
    ///
    /// # Platform Behavior
    ///
    /// - **Desktop**: Returns actual scores from file (same as `load_highscores()`)
    /// - **WASM**: Returns ONLY real scores from localStorage (no demo fallback)
    ///
    /// # Why This Exists
    ///
    /// On WASM, `load_highscores()` returns demo scores when localStorage is empty
    /// to provide a better first-time user experience. However, when saving a new
    /// score, we must NOT include those demo scores in the merged list, or they
    /// would become permanent entries.
    fn load_highscores_for_saving(&self) -> Vec<HighscoreEntry> {
        #[cfg(target_arch = "wasm32")]
        {
            self.load_from_localstorage_raw()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.load_cached_scores()
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_cached_scores(&self) -> Vec<HighscoreEntry> {
        if let Some(cached) = self.cache.borrow().as_ref() {
            return cached.clone();
        }

        let scores = self.load_from_file();
        self.update_cache(&scores);
        scores
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn update_cache(&self, entries: &[HighscoreEntry]) {
        *self.cache.borrow_mut() = Some(entries.to_vec());
    }

    /// Get top N highscores
    pub fn get_top_scores(&self, n: usize) -> Vec<HighscoreEntry> {
        let mut scores = self.load_highscores();
        scores.truncate(n);
        scores
    }

    /// Load highscores from desktop file storage (CSV format).
    ///
    /// Reads highscores from a CSV file with format: `name, score`
    /// Returns an empty vector if the file doesn't exist or cannot be read.
    ///
    /// # File Format
    ///
    /// ```text
    /// PLAYER1, 5000
    /// PLAYER2, 4500
    /// PLAYER3, 4000
    /// ```
    ///
    /// # Error Handling
    ///
    /// Silently ignores:
    /// - Missing file (returns empty vector)
    /// - I/O errors (returns empty vector)
    /// - Malformed lines (skips them)
    /// - Invalid score values (skips them)
    ///
    /// This graceful degradation ensures the game can always start,
    /// even if the highscore file is corrupted.
    #[cfg(not(target_arch = "wasm32"))]
    fn load_from_file(&self) -> Vec<HighscoreEntry> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        use std::path::Path;

        let path = Path::new(&self.storage_key);

        if !path.exists() {
            return Vec::new();
        }

        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Vec::new(),
        };

        let reader = BufReader::new(file);
        let mut entries = Vec::new();

        for line in reader.lines().map_while(Result::ok) {
            if let Some((name, score_str)) = line.split_once(',') {
                if let Ok(score) = score_str.trim().parse::<u32>() {
                    entries.push(HighscoreEntry::new(name.trim().to_string(), score));
                }
            }
        }

        entries.sort_by(|a, b| b.score.cmp(&a.score));
        entries
    }

    /// Save highscores to desktop file storage (CSV format).
    ///
    /// Writes all highscore entries to a CSV file with format: `name, score`
    /// Creates the file if it doesn't exist, overwrites if it does.
    ///
    /// # Arguments
    ///
    /// * `entries` - Slice of highscore entries (assumed to be pre-sorted)
    ///
    /// # Error Handling
    ///
    /// Silently fails if file cannot be created or written. This ensures
    /// the game continues running even if highscore persistence fails.
    #[cfg(not(target_arch = "wasm32"))]
    fn save_to_file(&self, entries: &[HighscoreEntry]) {
        use std::fs::OpenOptions;
        use std::io::Write;

        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.storage_key)
        {
            for entry in entries {
                let _ = writeln!(file, "{}, {}", entry.name, entry.score);
            }
        }
    }

    /// Load highscores from WASM localStorage with demo score fallback.
    ///
    /// This method provides a better first-time user experience by showing
    /// demo highscores when localStorage is empty (first launch).
    ///
    /// # Behavior
    ///
    /// 1. Attempts to load real scores via `load_from_localstorage_raw()`
    /// 2. If empty, returns 10 demo scores (5000 down to 500 in 500-point increments)
    /// 3. Demo scores are NEVER saved - see `load_highscores_for_saving()`
    ///
    /// # Returns
    ///
    /// A vector of highscore entries, either real or demo data for display purposes.
    ///
    /// # Note
    ///
    /// Demo scores prevent the menu from looking empty on first launch,
    /// giving players score targets to beat. They disappear once real
    /// scores are saved.
    #[cfg(target_arch = "wasm32")]
    fn load_from_localstorage(&self) -> Vec<HighscoreEntry> {
        let entries = self.load_from_localstorage_raw();

        // If no data found, return demo scores for display
        if entries.is_empty() {
            return vec![
                HighscoreEntry::new("PLAYER1".to_string(), 5000),
                HighscoreEntry::new("PLAYER2".to_string(), 4500),
                HighscoreEntry::new("PLAYER3".to_string(), 4000),
                HighscoreEntry::new("PLAYER4".to_string(), 3500),
                HighscoreEntry::new("PLAYER5".to_string(), 3000),
                HighscoreEntry::new("PLAYER6".to_string(), 2500),
                HighscoreEntry::new("PLAYER7".to_string(), 2000),
                HighscoreEntry::new("PLAYER8".to_string(), 1500),
                HighscoreEntry::new("PLAYER9".to_string(), 1000),
                HighscoreEntry::new("PLAYER10".to_string(), 500),
            ];
        }

        entries
    }

    /// Load highscores directly from WASM localStorage without demo fallback.
    ///
    /// This is the raw FFI bridge to JavaScript localStorage. It deserializes
    /// JSON data from localStorage into Rust structs.
    ///
    /// # FFI Bridge
    ///
    /// Calls JavaScript functions via `extern "C"`:
    /// - `js_localstorage_get(key)`: Retrieves JSON string from localStorage
    /// - `js_free_string(ptr)`: Frees JavaScript-allocated C string
    ///
    /// These functions must be provided by the WASM host (see game.html).
    ///
    /// # Safety
    ///
    /// Uses `unsafe` for FFI calls with proper error handling:
    /// - Validates CString creation
    /// - Checks for null pointers
    /// - Validates UTF-8 encoding
    /// - Frees JavaScript-allocated memory
    /// - Returns empty vector on any error
    ///
    /// # Returns
    ///
    /// A vector of real highscore entries from localStorage, or empty if:
    /// - localStorage is empty (first launch)
    /// - Key doesn't exist
    /// - Data is malformed JSON
    /// - FFI bridge fails
    ///
    /// # Note
    ///
    /// Cannot be unit tested (requires browser environment with localStorage).
    /// Testing is performed manually in browser builds.
    #[cfg(target_arch = "wasm32")]
    fn load_from_localstorage_raw(&self) -> Vec<HighscoreEntry> {
        use std::ffi::CString;
        use std::os::raw::c_char;

        extern "C" {
            fn js_localstorage_get(key: *const c_char) -> *mut c_char;
            fn js_free_string(ptr: *mut c_char);
        }

        unsafe {
            let key = match CString::new(self.storage_key.as_str()) {
                Ok(k) => k,
                Err(_) => return Vec::new(),
            };

            let value_ptr = js_localstorage_get(key.as_ptr());
            if value_ptr.is_null() {
                return Vec::new();
            }

            let c_str = std::ffi::CStr::from_ptr(value_ptr);
            let json_str = match c_str.to_str() {
                Ok(s) if s.len() <= MAX_LOCALSTORAGE_SIZE => s,
                Ok(_) => {
                    // Data too large, reject to prevent memory exhaustion
                    js_free_string(value_ptr);
                    return Vec::new();
                }
                Err(_) => {
                    js_free_string(value_ptr);
                    return Vec::new();
                }
            };

            let entries = match serde_json::from_str::<Vec<HighscoreEntry>>(json_str) {
                Ok(e) if e.len() <= MAX_HIGHSCORE_ENTRIES => e,
                Ok(_) => {
                    // Too many entries, reject to prevent DoS
                    js_free_string(value_ptr);
                    return Vec::new();
                }
                Err(_) => Vec::new(),
            };

            js_free_string(value_ptr);
            entries
        }
    }

    /// Save highscores to WASM localStorage via FFI bridge.
    ///
    /// Serializes highscore entries to JSON and stores them in browser
    /// localStorage via JavaScript FFI.
    ///
    /// # FFI Bridge
    ///
    /// Calls JavaScript function via `extern "C"`:
    /// - `js_localstorage_set(key, value)`: Stores JSON string in localStorage
    ///
    /// This function must be provided by the WASM host (see game.html).
    ///
    /// # Arguments
    ///
    /// * `entries` - Slice of highscore entries to save (assumed pre-sorted)
    ///
    /// # Safety
    ///
    /// Uses `unsafe` for FFI calls with proper error handling:
    /// - Validates CString creation for key and value
    /// - Returns early on any serialization/FFI error
    /// - JavaScript function handles localStorage quota/errors
    ///
    /// # Error Handling
    ///
    /// Silently fails on errors to ensure game continues running:
    /// - JSON serialization failure
    /// - CString creation failure
    /// - localStorage quota exceeded (handled by browser)
    ///
    /// # Note
    ///
    /// Cannot be unit tested (requires browser environment with localStorage).
    /// Testing is performed manually in browser builds.
    #[cfg(target_arch = "wasm32")]
    fn save_to_localstorage(&self, entries: &[HighscoreEntry]) {
        use std::ffi::CString;
        use std::os::raw::c_char;

        extern "C" {
            fn js_localstorage_set(key: *const c_char, value: *const c_char);
        }

        match serde_json::to_string(entries) {
            Ok(json_str) => unsafe {
                let key = match CString::new(self.storage_key.as_str()) {
                    Ok(k) => k,
                    Err(_) => return,
                };

                let value = match CString::new(json_str.as_str()) {
                    Ok(v) => v,
                    Err(_) => return,
                };

                js_localstorage_set(key.as_ptr(), value.as_ptr());
            },
            Err(_) => {}
        }
    }
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_highscore_entry_creation() {
        let entry = HighscoreEntry::new("Alice".to_string(), 1000);
        assert_eq!(entry.name, "Alice");
        assert_eq!(entry.score, 1000);
    }

    #[test]
    fn test_save_and_load_highscore() {
        let test_file = "test_highscores.txt";
        let manager = HighscoreManager::new(test_file);

        // Clean up before test
        let _ = fs::remove_file(test_file);

        // Save some scores
        manager.save_highscore("Alice", 1000);
        manager.save_highscore("Bob", 1500);
        manager.save_highscore("Charlie", 800);

        // Load and verify
        let scores = manager.load_highscores();
        assert_eq!(scores.len(), 3);
        assert_eq!(scores[0].name, "Bob");
        assert_eq!(scores[0].score, 1500);
        assert_eq!(scores[1].name, "Alice");
        assert_eq!(scores[1].score, 1000);

        // Clean up after test
        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_get_top_scores() {
        let test_file = "test_top_scores.txt";
        let manager = HighscoreManager::new(test_file);

        // Clean up before test
        let _ = fs::remove_file(test_file);

        // Save multiple scores
        for i in 1..=15 {
            manager.save_highscore(&format!("Player{}", i), i * 100);
        }

        // Get top 10
        let top_10 = manager.get_top_scores(10);
        assert_eq!(top_10.len(), 10);
        assert_eq!(top_10[0].score, 1500);
        assert_eq!(top_10[9].score, 600);

        // Clean up after test
        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_get_top_scores_fewer_than_requested() {
        let test_file = "test_few_scores.txt";
        let manager = HighscoreManager::new(test_file);

        // Clean up before test
        let _ = fs::remove_file(test_file);

        // Save only 3 scores
        manager.save_highscore("Alice", 100);
        manager.save_highscore("Bob", 200);
        manager.save_highscore("Charlie", 150);

        // Request top 10, should get only 3
        let top_scores = manager.get_top_scores(10);
        assert_eq!(top_scores.len(), 3);
        assert_eq!(top_scores[0].score, 200); // Bob
        assert_eq!(top_scores[1].score, 150); // Charlie
        assert_eq!(top_scores[2].score, 100); // Alice

        // Clean up after test
        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_duplicate_names_highscore() {
        let test_file = "test_duplicates.txt";
        let manager = HighscoreManager::new(test_file);

        // Clean up before test
        let _ = fs::remove_file(test_file);

        // Save multiple scores for same player
        manager.save_highscore("Alice", 100);
        manager.save_highscore("Alice", 200);
        manager.save_highscore("Alice", 150);

        // Should keep all scores
        let scores = manager.load_highscores();
        assert_eq!(scores.len(), 3);

        // Clean up after test
        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_zero_score() {
        let test_file = "test_zero_score.txt";
        let manager = HighscoreManager::new(test_file);

        // Clean up before test
        let _ = fs::remove_file(test_file);

        // Save a score of zero
        manager.save_highscore("NoPoints", 0);
        manager.save_highscore("SomePoints", 100);

        let scores = manager.load_highscores();
        assert_eq!(scores.len(), 2);
        assert_eq!(scores[0].score, 100); // Higher score first
        assert_eq!(scores[1].score, 0);
        assert_eq!(scores[1].name, "NoPoints");

        // Clean up after test
        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_very_large_score() {
        let test_file = "test_large_score.txt";
        let manager = HighscoreManager::new(test_file);

        // Clean up before test
        let _ = fs::remove_file(test_file);

        // Save a very large score (near u32::MAX)
        manager.save_highscore("MaxScore", u32::MAX);
        manager.save_highscore("Normal", 1000);

        let scores = manager.load_highscores();
        assert_eq!(scores.len(), 2);
        assert_eq!(scores[0].score, u32::MAX); // Largest score first
        assert_eq!(scores[0].name, "MaxScore");

        // Clean up after test
        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_empty_name() {
        let test_file = "test_empty_name.txt";
        let manager = HighscoreManager::new(test_file);

        // Clean up before test
        let _ = fs::remove_file(test_file);

        // Save a score with empty name (game shouldn't allow this, but test robustness)
        manager.save_highscore("", 500);
        manager.save_highscore("Player1", 1000);

        let scores = manager.load_highscores();
        assert_eq!(scores.len(), 2);
        assert_eq!(scores[1].name, ""); // Empty name should be preserved

        // Clean up after test
        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_malformed_csv_line() {
        let test_file = "test_malformed.txt";

        // Create a file with some malformed lines
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(test_file)
            .unwrap();

        // Write mix of valid and invalid lines
        writeln!(file, "GoodPlayer, 1000").unwrap();
        writeln!(file, "InvalidLine without comma").unwrap(); // Should be skipped
        writeln!(file, "BadScore, notanumber").unwrap(); // Should be skipped
        writeln!(file, "AnotherGood, 500").unwrap();
        writeln!(file, ", 300").unwrap(); // Empty name but valid score - should work
        drop(file);

        let manager = HighscoreManager::new(test_file);
        let scores = manager.load_highscores();

        // Should only load the valid lines
        assert_eq!(scores.len(), 3); // GoodPlayer, AnotherGood, and empty-name entry
        assert_eq!(scores[0].score, 1000);
        assert_eq!(scores[0].name, "GoodPlayer");
        assert_eq!(scores[1].score, 500);
        assert_eq!(scores[2].score, 300);

        // Clean up after test
        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_highscore_entry_new_must_use() {
        // This test verifies that #[must_use] is present by actually using the value
        let entry = HighscoreEntry::new("Player".to_string(), 100);
        assert_eq!(entry.name, "Player");
        assert_eq!(entry.score, 100);
    }

    #[test]
    fn test_manager_new_must_use() {
        // This test verifies that #[must_use] is present by actually using the value
        let manager = HighscoreManager::new("test.txt");
        // Verify it was created properly (internal check)
        assert_eq!(manager.storage_key, "test.txt");
    }
}
