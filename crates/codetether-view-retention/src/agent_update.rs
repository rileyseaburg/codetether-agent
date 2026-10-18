//! Mutate retained live agent detail buffers.

/// Append to the live-agent detail buffer.
/// # Arguments
/// The history buffer and new item.
/// # Returns
/// Unit after retaining the newest items.
/// # Examples
/// ```
/// let mut values = Vec::new(); codetether_view_retention::agent_update::push(&mut values, 1); assert_eq!(values, vec![1]);
/// ```
pub fn push<T>(items: &mut Vec<T>, item: T) {
    crate::agent::push(items, item);
}

/// Replace an optional output slot with bounded text.
/// # Arguments
/// The target slot, source text and diagnostic label.
/// # Returns
/// Unit after replacing the slot.
/// # Examples
/// ```
/// let mut slot = None; codetether_view_retention::agent_update::output(&mut slot, "ok", "result"); assert_eq!(slot.as_deref(), Some("ok"));
/// ```
pub fn output(slot: &mut Option<String>, text: &str, label: &str) {
    *slot = Some(crate::agent::output(text, label));
}
