//! Compare pre-execution and post-execution Git file snapshots.
use std::collections::BTreeSet;
/// Select newly changed files, retaining the existing snapshot iteration order.
///
/// # Arguments
/// `before` and `after` are optional Git snapshots.
/// # Returns
/// Files introduced to the changed set, or the entire after-set if before is absent.
/// # Examples
/// ```
/// assert!(codetether_forage_core::execution::changed_files(None, None).is_empty());
/// ```
pub fn changed_files(
    before: Option<BTreeSet<String>>,
    after: Option<BTreeSet<String>>,
) -> Vec<String> {
    match (before, after) {
        (Some(before_set), Some(after_set)) => after_set.difference(&before_set).cloned().collect(),
        (_, Some(after_set)) => after_set.into_iter().collect(),
        _ => Vec::new(),
    }
}
