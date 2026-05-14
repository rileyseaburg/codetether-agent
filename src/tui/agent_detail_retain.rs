//! Retention helpers for live agent detail panes.

use crate::tui::retained_payload::{
    bounded, push_capped, AGENT_OUTPUT_MAX_BYTES, DETAIL_HISTORY_MAX_ITEMS,
};

pub fn push<T>(items: &mut Vec<T>, item: T) {
    push_capped(items, item, DETAIL_HISTORY_MAX_ITEMS);
}

pub fn output(text: &str, label: &str) -> String {
    bounded(text, AGENT_OUTPUT_MAX_BYTES, label)
}
