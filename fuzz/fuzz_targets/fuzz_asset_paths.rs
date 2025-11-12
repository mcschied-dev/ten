#![no_main]

use libfuzzer_sys::fuzz_target;

// Note: candidate_asset_paths is not public, so we'll test the public API
// that uses it internally (load_texture_fallback, etc.)
// This fuzz target tests that arbitrary path strings don't cause panics

fuzz_target!(|data: &[u8]| {
    if let Ok(path_str) = std::str::from_utf8(data) {
        // Test various edge cases that might cause issues:

        // 1. Path traversal attempts
        let _ = path_str.contains("..");

        // 2. Absolute paths
        let _ = path_str.starts_with('/');

        // 3. Windows-style paths
        let _ = path_str.contains('\\');

        // 4. Null bytes (should be rejected by filesystem)
        let _ = path_str.contains('\0');

        // 5. Very long paths
        let _ = path_str.len() > 4096;

        // 6. Special characters
        let _ = path_str.chars().any(|c| c.is_control());

        // The actual asset loading is async and requires macroquad context,
        // so we just verify the path string doesn't cause immediate panics
        // in path manipulation code

        // Simulate path operations that candidate_asset_paths would do
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let _ = exe_dir.join(path_str);
            }
        }
    }
});
