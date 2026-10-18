//! Map RGB values to an ANSI cube or grayscale entry.

/// Convert RGB to closest ANSI 256-color index
pub(super) fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    // Standard 256-color conversion
    if r == g && g == b {
        // Grayscale
        if r < 8 {
            16
        } else if r > 248 {
            231
        } else {
            232 + ((r - 8) / 10)
        }
    } else {
        // Color
        16 + (36 * (r / 51)) + (6 * (g / 51)) + (b / 51)
    }
}
