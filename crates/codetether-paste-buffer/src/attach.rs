//! Allocate paste identifiers and attach content to the sidecar buffer.
use crate::PendingTextPaste;

/// Allocate the next paste id.
pub(crate) fn next_paste_id(pastes: &[PendingTextPaste]) -> u32 {
    pastes
        .iter()
        .map(|p| p.id)
        .max()
        .map(|m| m + 1)
        .unwrap_or(1)
}

/// Push a new sidecar entry and return the placeholder string the
/// caller should insert into the input buffer at the cursor.
///
/// # Arguments
/// * `pastes` — Existing sidecar entries.
/// * `content` — New text to retain.
///
/// # Returns
/// The placeholder for the newly appended entry.
///
/// # Panics
/// May panic if incrementing the largest existing identifier overflows.
///
/// # Examples
/// ```
/// let mut pastes = Vec::new();
/// let label = codetether_paste_buffer::attach_paste(&mut pastes, "hello".into());
/// assert_eq!(pastes.len(), 1);
/// assert!(label.contains("#1"));
/// ```
pub fn attach_paste(pastes: &mut Vec<PendingTextPaste>, content: String) -> String {
    let id = next_paste_id(pastes);
    let paste = PendingTextPaste { id, content };
    let placeholder = paste.placeholder();
    pastes.push(paste);
    placeholder
}
