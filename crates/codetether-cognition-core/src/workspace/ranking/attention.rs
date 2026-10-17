//! Attention ranking for the global workspace summary.

use std::cmp::Ordering;

use crate::AttentionItem;

/// How many attention items the workspace retains.
const TOP_ATTENTION: usize = 10;

/// Rank unresolved attention items by descending priority.
///
/// # Arguments
/// * `queue` - Attention records to rank.
///
/// # Returns
/// The result of the policy described above.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::workspace::ranking::rank_attention;
/// let result = rank_attention(&[]);
/// assert!(result.is_empty());
/// ```
pub fn rank_attention(queue: &[AttentionItem]) -> Vec<String> {
    let mut open: Vec<&AttentionItem> = queue.iter().filter(|a| a.resolved_at.is_none()).collect();
    open.sort_by(|a, b| {
        b.priority
            .partial_cmp(&a.priority)
            .unwrap_or(Ordering::Equal)
    });
    open.iter()
        .take(TOP_ATTENTION)
        .map(|a| a.id.clone())
        .collect()
}
