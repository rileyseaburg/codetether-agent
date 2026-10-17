//! pinned indices policy.
use super::*;

/// Indices of pinned [`PageKind::Constraint`] turns in the canonical
/// transcript. Shared with the incremental scorer so pins are force-kept.
///
/// # Arguments
/// * `pages` - Page classifications for the live window.
/// * `source` - Parallel message window.
///
/// # Returns
/// Constraint indices covered by both slices.
///
/// # Examples
/// ```
/// use codetether_context_policy::{pins::pinned_indices, pages::PageKind};
/// use codetether_provider_core::{Message, Role};
/// let source = [Message { role: Role::User, content: Vec::new() }];
/// assert_eq!(pinned_indices(&[PageKind::Constraint], &source), vec![0]);
/// ```
pub fn pinned_indices(pages: &[PageKind], source: &[Message]) -> Vec<usize> {
    pages
        .iter()
        .zip(source.iter())
        .enumerate()
        .filter(|(_, (kind, _))| matches!(kind, PageKind::Constraint))
        .map(|(idx, _)| idx)
        .collect()
}
