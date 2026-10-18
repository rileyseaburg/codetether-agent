//! Snapshot payload and its durable acknowledgement token.
use crate::Ticket;
/// One consistent consumer snapshot and the token for acknowledging its range.
/// # Examples
/// ```rust
/// # fn example(ticket: codetether_session_store::Ticket) {
/// let delivery = codetether_session_store::Delivery { session: (), ticket };
/// assert_eq!(delivery.session, ());
/// # }
/// ```
pub struct Delivery<S> {
    /// Caller-decoded snapshot payload.
    pub session: S,
    /// Durable consumer position and invalidation generation.
    pub ticket: Ticket,
}
