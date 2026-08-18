//! Compact bus payloads before retaining them in TUI state.

use crate::tui::retained_payload::bounded;

pub const FIELD_MAX_BYTES: usize = 512;
pub const KIND_MAX_BYTES: usize = 64;
pub const SUMMARY_MAX_BYTES: usize = 512;
pub const DETAIL_MAX_BYTES: usize = 8 * 1024;

pub fn field(input: &str) -> String {
    bounded(input, FIELD_MAX_BYTES, "bus field")
}

pub fn kind(input: &str) -> String {
    bounded(input, KIND_MAX_BYTES, "bus kind")
}

pub fn summary(input: &str) -> String {
    bounded(input, SUMMARY_MAX_BYTES, "bus summary")
}

pub fn detail(input: &str, label: &str) -> String {
    bounded(input, DETAIL_MAX_BYTES, label)
}

pub fn tool_detail(header: &str, output: &str, label: &str) -> String {
    detail(&format!("{header}\n{}", detail(output, label)), label)
}
