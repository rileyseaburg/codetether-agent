//! Public receipt returned when input enters a child mailbox.

/// Durable identity and current position for accepted child input.
///
/// # Examples
/// ```
/// let receipt=codetether_child_mailbox::Submission{id:"delivery".into(),depth:1};
/// assert_eq!(receipt.depth,1);
/// ```
pub struct Submission {
    /// Stable ID retained through retries and crash recovery.
    pub id: String,
    /// One-based position in the durable mailbox.
    pub depth: usize,
}
