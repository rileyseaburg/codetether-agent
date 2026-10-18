//! UTF-8-safe bounding for mux terminal observations.

/// Decode and retain at most the latest 20,000 terminal characters.
pub(super) fn bounded(data: &[u8]) -> String {
    String::from_utf8_lossy(data)
        .chars()
        .rev()
        .take(20_000)
        .collect::<String>()
        .chars()
        .rev()
        .collect()
}
