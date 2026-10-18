//! Clean terminal selections and normalize pasted text.
//!
//! [has_tui_artifacts] detects copied box borders, [strip_tui_artifacts]
//! removes those borders and reflows visual rows, and [normalize_paste]
//! also normalizes line endings. Plain text is preserved without reflow.
mod clean;
mod detect;
mod paste;
mod strip_line;
pub use clean::strip_tui_artifacts;
pub use detect::has_tui_artifacts;
pub use paste::normalize_paste;
#[cfg(test)]
mod reflow_tests;
#[cfg(test)]
mod tests;
