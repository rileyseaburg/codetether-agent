//! One inbound turn's completion slot.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::oneshot;

/// The final assistant text for one inbound turn, or why it did not finish.
///
/// # Examples
/// ```rust
/// use codetether_a2a_inbox::Outcome;
/// let outcome: Outcome = Ok("done".into());
/// assert!(outcome.is_ok());
/// ```
pub type Outcome = Result<String, String>;

/// Server-side handle: resolves when the TUI finishes the turn.
///
/// Implements [`Future`] so callers can wrap it in timeouts and keep polling
/// the same slot across heartbeats.
///
/// # Examples
/// ```rust
/// let pending = codetether_a2a_inbox::enqueue("task", None, "peer", "hello");
/// let inbound = codetether_a2a_inbox::dequeue().unwrap();
/// inbound.responder.resolve(Ok("done".into()));
/// ```
pub struct Pending {
    receiver: oneshot::Receiver<Outcome>,
}

/// TUI-side handle: resolves the turn once the assistant replies.
///
/// # Examples
/// ```rust
/// let _pending = codetether_a2a_inbox::enqueue("task", None, "peer", "hello");
/// let inbound = codetether_a2a_inbox::dequeue().unwrap();
/// inbound.responder.resolve(Ok("done".into()));
/// ```
pub struct Responder {
    sender: oneshot::Sender<Outcome>,
}

pub(super) fn channel() -> (Pending, Responder) {
    let (sender, receiver) = oneshot::channel();
    (Pending { receiver }, Responder { sender })
}

impl Future for Pending {
    type Output = Outcome;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Outcome> {
        Pin::new(&mut self.receiver).poll(cx).map(|received| {
            received.unwrap_or_else(|_| Err("interactive session dropped the turn".to_string()))
        })
    }
}

impl Responder {
    /// Complete the pending turn; a dropped receiver is tolerated.
    ///
    /// # Examples
    /// ```rust
    /// let _pending = codetether_a2a_inbox::enqueue("task", None, "peer", "hello");
    /// codetether_a2a_inbox::dequeue().unwrap().responder.resolve(Ok("done".into()));
    /// ```
    pub fn resolve(self, outcome: Outcome) {
        // The server may have timed out and gone; that is not an error here.
        let _ = self.sender.send(outcome);
    }
}
