//! Formats user-facing help text for all registered TUI view modes.
//!
//! The functions in this module turn the canonical view-mode registry into
//! display strings used by help overlays and compact status summaries. The
//! registry controls which modes are shown and in what order; this module only
//! applies presentation formatting.

use super::view_mode_display::{view_mode_display_name, view_mode_shortcut_hint};
use super::view_mode_registry::ALL_VIEW_MODES;

/// Builds aligned help rows that pair each view mode with its shortcut hint.
///
/// Rows are returned in the same order as [`ALL_VIEW_MODES`]. Each row starts
/// with the human-readable view-mode name padded to a fixed width, followed by
/// the shortcut hint text for that mode. The function performs no I/O, has no
/// side effects, and depends only on the static view-mode registry.
///
/// # Returns
///
/// A vector containing one formatted help row for each registered view mode.
/// If no view modes are registered, the returned vector is empty.
pub fn view_mode_help_rows() -> Vec<String> {
    ALL_VIEW_MODES
        .iter()
        .map(|mode| {
            format!(
                "{:<14} {}",
                view_mode_display_name(mode.clone()),
                view_mode_shortcut_hint(mode.clone())
            )
        })
        .collect()
}

/// Builds a compact one-line summary of every registered view mode.
///
/// The summary contains only display names, joined with `" | "` separators, in
/// the order defined by [`ALL_VIEW_MODES`]. It is intended for constrained UI
/// locations where shortcut details would be too verbose.
///
/// # Returns
///
/// A single string containing all view-mode display names. If the registry is
/// empty, the returned string is empty.
pub fn view_mode_compact_summary() -> String {
    ALL_VIEW_MODES
        .iter()
        .map(|m| view_mode_display_name(m.clone()))
        .collect::<Vec<_>>()
        .join(" | ")
}
