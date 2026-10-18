//! Retention helpers for live agent detail panes.

use crate::{AGENT_OUTPUT_MAX_BYTES, DETAIL_HISTORY_MAX_ITEMS, bounded, push_capped};

/// Append to the bounded live-agent detail history.
/// # Arguments
/// The history buffer and new item.
/// # Returns
/// Unit after retaining the newest items.
/// # Examples
/// ```
/// let mut values = Vec::new(); codetether_view_retention::agent::push(&mut values, 1); assert_eq!(values, vec![1]);
/// ```
pub fn push<T>(items: &mut Vec<T>, item: T) {
    push_capped(items, item, DETAIL_HISTORY_MAX_ITEMS);
}

/// Bound agent output using the shared agent byte limit.
/// # Arguments
/// Output text and diagnostic label.
/// # Returns
/// Bounded output text.
/// # Examples
/// ```
/// assert_eq!(codetether_view_retention::agent::output("ok", "result"), "ok");
/// ```
pub fn output(text: &str, label: &str) -> String {
    bounded(text, AGENT_OUTPUT_MAX_BYTES, label)
}
