//! Compact bus payloads before retaining them in TUI state.

use crate::tui::retained_payload::{bounded, TOOL_DETAIL_MAX_BYTES};

pub fn detail(input: &str, label: &str) -> String {
    bounded(input, TOOL_DETAIL_MAX_BYTES, label)
}

pub fn tool_detail(header: &str, output: &str, label: &str) -> String {
    format!("{header}\n{}", detail(output, label))
}
