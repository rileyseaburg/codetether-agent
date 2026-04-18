//! Formatted help rows and compact summary for all view modes.

use super::view_mode_display::{view_mode_display_name, view_mode_shortcut_hint};
use super::view_mode_registry::ALL_VIEW_MODES;

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

pub fn view_mode_compact_summary() -> String {
    ALL_VIEW_MODES
        .iter()
        .map(|m| view_mode_display_name(m.clone()))
        .collect::<Vec<_>>()
        .join(" | ")
}
