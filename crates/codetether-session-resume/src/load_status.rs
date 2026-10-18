//! Human-readable summary of a loaded session tail.

/// Format a successful interactive session load.
///
/// # Arguments
/// * `label` — Session display label.
/// * `visible_messages` — Loaded message count.
/// * `dropped` — Older messages retained only on disk.
/// * `file_bytes` — Source locator byte size.
/// * `original_id` — Source ID to show for a truncated load.
///
/// # Returns
/// The existing complete-load or truncated-load status text.
///
/// # Examples
/// ```
/// assert_eq!(codetether_session_resume::load_status_with_original("demo", 3, 0, 10, None), "Loaded session demo");
/// ```
pub fn load_status_with_original(
    label: &str,
    visible_messages: usize,
    dropped: usize,
    file_bytes: u64,
    original_id: Option<&str>,
) -> String {
    if dropped == 0 {
        return format!("Loaded session {label}");
    }
    let mb = file_bytes as f64 / (1024.0 * 1024.0);
    format!(
        "Loaded session {label}: showing last {} entries; {dropped} older entries remain on disk ({mb:.1} MiB locator); source {}",
        visible_messages,
        original_id.unwrap_or("unknown")
    )
}
