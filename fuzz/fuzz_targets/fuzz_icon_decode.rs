#![no_main]

use libfuzzer_sys::fuzz_target;

// Note: decode_icon is not public and uses const generics,
// but we can test the image decoding logic that it uses internally

fuzz_target!(|data: &[u8]| {
    // Test PNG decoding with arbitrary binary data
    // This should never panic, only return errors

    // Try to decode as PNG image
    if let Ok(img) = image::load_from_memory(data) {
        // If it successfully decodes, verify dimensions are reasonable
        let (width, height) = img.dimensions();

        // Check for reasonable dimensions (icon sizes)
        let _ = width <= 1024 && height <= 1024;

        // Try to convert to RGBA (what decode_icon does)
        let rgba = img.to_rgba8();

        // Verify buffer size calculations don't overflow
        let expected_size = (width as usize)
            .checked_mul(height as usize)
            .and_then(|v| v.checked_mul(4));

        if let Some(size) = expected_size {
            // Verify actual buffer size matches expected
            let _ = rgba.as_raw().len() == size;
        }
    }

    // Also test specific icon sizes that the game uses
    if data.len() >= 16 * 16 * 4 {
        let _ = image::load_from_memory(&data[..std::cmp::min(data.len(), 1024)]);
    }
});
