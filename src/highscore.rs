//! Highscore management with cross-platform storage.
//!
//! Uses file I/O on desktop and LocalStorage on WASM.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighscoreEntry {
    pub name: String,
    pub score: u32,
}

impl HighscoreEntry {
    pub fn new(name: String, score: u32) -> Self {
        Self { name, score }
    }
}

pub struct HighscoreManager {
    storage_key: String,
}

impl HighscoreManager {
    pub fn new(key: &str) -> Self {
        Self {
            storage_key: key.to_string(),
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
            self.load_from_file()
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

        #[cfg(target_arch = "wasm32")]
        {
            self.save_to_localstorage(&entries);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.save_to_file(&entries);
        }
    }

    /// Load highscores for saving (no demo data)
    fn load_highscores_for_saving(&self) -> Vec<HighscoreEntry> {
        #[cfg(target_arch = "wasm32")]
        {
            self.load_from_localstorage_raw()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.load_from_file()
        }
    }

    /// Get top N highscores
    pub fn get_top_scores(&self, n: usize) -> Vec<HighscoreEntry> {
        let mut scores = self.load_highscores();
        scores.truncate(n);
        scores
    }

    // Desktop file I/O implementation
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

    // WASM LocalStorage implementation using custom FFI bridge (with demo scores)
    #[cfg(target_arch = "wasm32")]
    fn load_from_localstorage(&self) -> Vec<HighscoreEntry> {
        let entries = self.load_from_localstorage_raw();

        // If no data found, return demo scores for display
        if entries.is_empty() {
            println!("WASM: No data found in localStorage, returning demo scores");
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

    // WASM LocalStorage raw implementation (no demo scores)
    #[cfg(target_arch = "wasm32")]
    fn load_from_localstorage_raw(&self) -> Vec<HighscoreEntry> {
        use std::ffi::CString;
        use std::os::raw::c_char;

        extern "C" {
            fn js_localstorage_get(key: *const c_char) -> *mut c_char;
            fn js_free_string(ptr: *mut c_char);
        }

        println!("WASM: Attempting to load highscores from localStorage key: {}", self.storage_key);

        unsafe {
            let key = match CString::new(self.storage_key.as_str()) {
                Ok(k) => k,
                Err(e) => {
                    println!("WASM: Failed to create CString for key: {:?}", e);
                    return Vec::new();
                }
            };

            let value_ptr = js_localstorage_get(key.as_ptr());
            if value_ptr.is_null() {
                println!("WASM: No data found in localStorage");
                return Vec::new();
            }

            let c_str = std::ffi::CStr::from_ptr(value_ptr);
            let json_str = match c_str.to_str() {
                Ok(s) => s,
                Err(e) => {
                    println!("WASM: Failed to convert C string to Rust string: {:?}", e);
                    js_free_string(value_ptr);
                    return Vec::new();
                }
            };

            println!("WASM: Found data in localStorage: {}", json_str);

            let entries = match serde_json::from_str::<Vec<HighscoreEntry>>(json_str) {
                Ok(e) => {
                    println!("WASM: Successfully loaded {} highscores from localStorage", e.len());
                    e
                }
                Err(e) => {
                    println!("WASM: Failed to parse highscores JSON: {:?}", e);
                    Vec::new()
                }
            };

            js_free_string(value_ptr);
            entries
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn save_to_localstorage(&self, entries: &[HighscoreEntry]) {
        use std::ffi::CString;
        use std::os::raw::c_char;

        extern "C" {
            fn js_localstorage_set(key: *const c_char, value: *const c_char);
        }

        println!("WASM: Attempting to save {} highscores to localStorage", entries.len());

        match serde_json::to_string(entries) {
            Ok(json_str) => {
                println!("WASM: Serialized highscores to JSON: {}", json_str);

                unsafe {
                    let key = match CString::new(self.storage_key.as_str()) {
                        Ok(k) => k,
                        Err(e) => {
                            println!("WASM: Failed to create CString for key: {:?}", e);
                            return;
                        }
                    };

                    let value = match CString::new(json_str.as_str()) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("WASM: Failed to create CString for value: {:?}", e);
                            return;
                        }
                    };

                    js_localstorage_set(key.as_ptr(), value.as_ptr());
                    println!("WASM: Successfully saved to localStorage key: {}", self.storage_key);
                }
            }
            Err(e) => {
                println!("WASM: Failed to serialize highscores: {:?}", e);
            }
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
}
