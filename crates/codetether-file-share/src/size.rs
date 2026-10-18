//! Human-readable binary-unit file sizes.
pub(crate) fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    if bytes < 1024 {
        format!("{bytes}B")
    } else if (bytes as f64) < MB {
        format!("{:.1}KB", bytes as f64 / KB)
    } else if (bytes as f64) < GB {
        format!("{:.1}MB", bytes as f64 / MB)
    } else {
        format!("{:.2}GB", bytes as f64 / GB)
    }
}
