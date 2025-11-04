use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, Clone)]
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
    file_path: String,
}

impl HighscoreManager {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }

    /// Load highscores from file, sorted by score (highest first)
    pub fn load_highscores(&self) -> Vec<HighscoreEntry> {
        let path = Path::new(&self.file_path);

        if !path.exists() {
            return Vec::new();
        }

        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Vec::new(),
        };

        let reader = BufReader::new(file);
        let mut entries = Vec::new();

        for line in reader.lines() {
            if let Ok(line) = line {
                if let Some((name, score_str)) = line.split_once(',') {
                    if let Ok(score) = score_str.trim().parse::<u32>() {
                        entries.push(HighscoreEntry::new(name.trim().to_string(), score));
                    }
                }
            }
        }

        // Sort by score, highest first
        entries.sort_by(|a, b| b.score.cmp(&a.score));
        entries
    }

    /// Get top N highscores
    pub fn get_top_scores(&self, n: usize) -> Vec<HighscoreEntry> {
        let mut scores = self.load_highscores();
        scores.truncate(n);
        scores
    }

    /// Save a new highscore
    pub fn save_highscore(&self, name: &str, score: u32) {
        let mut entries = self.load_highscores();

        // Add new entry
        entries.push(HighscoreEntry::new(name.to_string(), score));

        // Sort by score, highest first
        entries.sort_by(|a, b| b.score.cmp(&a.score));

        // Write all entries back to file
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)
        {
            for entry in entries {
                let _ = writeln!(file, "{}, {}", entry.name, entry.score);
            }
        }
    }

    /// Check if a score qualifies for the top 10
    pub fn is_highscore(&self, score: u32) -> bool {
        let top_scores = self.get_top_scores(10);

        if top_scores.len() < 10 {
            return true;
        }

        score > top_scores.last().map(|e| e.score).unwrap_or(0)
    }
}

#[cfg(test)]
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
}
