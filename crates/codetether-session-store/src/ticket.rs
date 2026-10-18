//! Durable consumer acknowledgement token with invalidation generation.
use std::path::PathBuf;

/// Consumer position captured from one consistent session snapshot.
/// Acknowledge only after processing the represented range successfully.
/// # Examples
/// ```rust
/// let ticket = codetether_session_store::Ticket {
///     path: "session.json".into(), id: "session-123".into(), name: "recall".into(),
///     from: 0, to: 4, generation: 0,
/// };
/// assert_eq!(ticket.to - ticket.from, 4);
/// ```
#[derive(Clone)]
pub struct Ticket {
    /// Session locator path.
    pub path: PathBuf,
    /// Durable session identity.
    pub id: String,
    /// Independent consumer name.
    pub name: String,
    /// Inclusive source sequence.
    pub from: usize,
    /// Exclusive acknowledged sequence.
    pub to: usize,
    /// Invalidation generation captured with the range.
    pub generation: i64,
}
