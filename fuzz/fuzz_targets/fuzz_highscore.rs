#![no_main]

use libfuzzer_sys::fuzz_target;
use ten::highscore::HighscoreManager;
use std::fs;
use std::io::Write;

fuzz_target!(|data: &[u8]| {
    // Convert fuzzer input to string (may contain invalid UTF-8)
    if let Ok(csv_data) = std::str::from_utf8(data) {
        // Create temporary test file with fuzzed CSV data
        let test_file = format!("test_fuzz_highscore_{}.txt", rand::random::<u64>());

        if let Ok(mut file) = fs::File::create(&test_file) {
            let _ = file.write_all(csv_data.as_bytes());
            drop(file);

            // Try to load the fuzzed CSV - should not panic
            let manager = HighscoreManager::new(&test_file);
            let _ = manager.get_top_scores(10);

            // Clean up
            let _ = fs::remove_file(&test_file);
        }
    }
});
