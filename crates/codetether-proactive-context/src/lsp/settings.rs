//! Positive limits for proactive LSP context.
pub(super) fn limits() -> (usize, usize) {
    let max_files = std::env::var("CODETETHER_PROACTIVE_LSP_MAX_FILES")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(3);

    let max_chars = std::env::var("CODETETHER_PROACTIVE_LSP_MAX_CHARS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(1600);

    (max_files, max_chars)
}
