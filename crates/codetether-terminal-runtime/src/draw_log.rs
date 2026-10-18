//! Diagnostics for skipped terminal dimensions.

pub(super) fn log_skipped_size(size: ratatui::layout::Size) {
    tracing::warn!(
        width = size.width,
        height = size.height,
        cells = size.width as u32 * size.height as u32,
        "skipping TUI draw because terminal reported an invalid size"
    );
}
