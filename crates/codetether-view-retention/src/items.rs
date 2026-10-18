//! Keep only the newest items when appending to a view buffer.

/// Append an item and discard excess oldest items.
/// # Arguments
/// The buffer, new item and maximum retained length.
/// # Returns
/// Unit after the buffer is bounded.
/// # Examples
/// ```
/// let mut values = vec![1, 2]; codetether_view_retention::push_capped(&mut values, 3, 2); assert_eq!(values, vec![2, 3]);
/// ```
pub fn push_capped<T>(items: &mut Vec<T>, item: T, max_items: usize) {
    items.push(item);
    if items.len() > max_items {
        let overflow = items.len() - max_items;
        items.drain(0..overflow);
    }
}
