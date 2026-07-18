//! Public receipt returned when input enters a child mailbox.

/// Durable identity and current position for accepted child input.
pub(crate) struct Submission {
    /// Stable ID retained through retries and crash recovery.
    pub(crate) id: String,
    /// One-based position in the durable mailbox.
    pub(crate) depth: usize,
}
