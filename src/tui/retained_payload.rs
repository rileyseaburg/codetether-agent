//! Small retained payload helpers for TUI memory budgets.

use crate::util::truncate_bytes_safe;

pub const TOOL_DETAIL_MAX_BYTES: usize = 8 * 1024;
pub const AGENT_OUTPUT_MAX_BYTES: usize = 16 * 1024;
pub const DETAIL_HISTORY_MAX_ITEMS: usize = 100;
pub const CHAT_RETAINED_MAX_ITEMS: usize = 500;

pub fn bounded(input: &str, max_bytes: usize, label: &str) -> String {
    if input.len() <= max_bytes {
        return input.to_string();
    }
    let mut out = truncate_bytes_safe(input, max_bytes).to_string();
    out.push_str("\n\n[truncated in live TUI memory: ");
    out.push_str(label);
    out.push(']');
    out
}

pub fn push_capped<T>(items: &mut Vec<T>, item: T, max_items: usize) {
    items.push(item);
    if items.len() > max_items {
        let overflow = items.len() - max_items;
        items.drain(0..overflow);
    }
}
