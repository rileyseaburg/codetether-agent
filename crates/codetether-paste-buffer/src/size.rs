//! Human-readable byte sizes for paste placeholders.

/// Format a byte count as a short human-readable string (e.g. `1.2 KiB`).
///
/// # Arguments
/// * `bytes` — Byte count to display.
///
/// # Returns
/// A byte, KiB or MiB label.
///
/// # Examples
/// ```
/// assert_eq!(codetether_paste_buffer::format_size(1024), "1.0 KiB");
/// ```
pub fn format_size(bytes: usize) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    if bytes < 1024 {
        format!("{bytes} B")
    } else if (bytes as f64) < MIB {
        format!("{:.1} KiB", bytes as f64 / KIB)
    } else {
        format!("{:.1} MiB", bytes as f64 / MIB)
    }
}
