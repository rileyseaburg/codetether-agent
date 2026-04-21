//! Append-only view of a session's chat history.
//!
//! The core Phase A invariant is that [`Session::messages`] is the pure
//! record of *what happened* and is never mutated by compression,
//! experimental dedup / snippet strategies, or pairing repair. This
//! module provides a typestate-style wrapper, [`History`], whose only
//! mutating method is `append` — making it a compile-time error for a
//! new caller to acquire a `&mut Vec<Message>` and rewrite the buffer
//! in place.
//!
//! ## Scope
//!
//! This is a *foundation* for the visibility tightening described in the
//! refactor plan. The public [`Session::messages`] field stays `pub` for
//! now so the crate keeps building across every caller (19 files, ~51
//! usages at the time of this refactor). New code paths should reach for
//! [`Session::history`] instead. Future PRs can tighten `messages` to
//! `pub(crate)` — each remaining direct-mutation site then surfaces as
//! a compile error and is migrated onto this typestate.
//!
//! ## Examples
//!
//! ```rust,no_run
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! use codetether_agent::session::Session;
//!
//! let session = Session::new().await.unwrap();
//! let view = session.history();
//! assert!(view.is_empty());
//! # });
//! ```
//!
//! ```rust
//! use codetether_agent::provider::{ContentPart, Message, Role};
//! use codetether_agent::session::history::History;
//!
//! let mut buf: Vec<Message> = Vec::new();
//! let mut history = History::new(&mut buf);
//! history.append(Message {
//!     role: Role::User,
//!     content: vec![ContentPart::Text {
//!         text: "hello".to_string(),
//!     }],
//! });
//! assert_eq!(history.view().len(), 1);
//! ```

use crate::provider::Message;

/// Append-only handle to a chat-history buffer.
///
/// Wraps `&mut Vec<Message>` but exposes only [`append`](Self::append)
/// and [`view`](Self::view). Nothing can reach the underlying `Vec` to
/// call `pop`, `truncate`, `clear`, `split_off`, `remove`, or `last_mut`
/// without going through the explicit (and reviewable) escape hatch
/// [`Session::messages_mut_unchecked`](crate::session::Session) — which
/// intentionally does not exist in Phase A.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::{ContentPart, Message, Role};
/// use codetether_agent::session::history::History;
///
/// let mut buf: Vec<Message> = Vec::new();
/// let mut history = History::new(&mut buf);
/// assert!(history.view().is_empty());
///
/// history.append(Message {
///     role: Role::Assistant,
///     content: vec![ContentPart::Text {
///         text: "ok".to_string(),
///     }],
/// });
/// assert_eq!(history.view().len(), 1);
/// ```
pub struct History<'a> {
    buf: &'a mut Vec<Message>,
}

impl<'a> History<'a> {
    /// Wrap an existing `Vec<Message>` as an append-only history.
    ///
    /// # Arguments
    ///
    /// * `buf` — The underlying buffer. Ownership is not taken; the
    ///   caller keeps the `Vec` but loses the ability to rewrite it
    ///   while the `History` handle is alive.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::provider::Message;
    /// use codetether_agent::session::history::History;
    ///
    /// let mut buf: Vec<Message> = Vec::new();
    /// let history = History::new(&mut buf);
    /// assert!(history.view().is_empty());
    /// ```
    pub fn new(buf: &'a mut Vec<Message>) -> Self {
        Self { buf }
    }

    /// Append a single message to the end of the history.
    ///
    /// The only mutating operation on [`History`] — every other mutation
    /// must be routed through a dedicated API on the owning type.
    pub fn append(&mut self, msg: Message) {
        self.buf.push(msg);
    }

    /// Borrow the history as an immutable slice for reads.
    pub fn view(&self) -> &[Message] {
        self.buf
    }

    /// The number of entries currently in the history.
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// `true` when the history is empty.
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::{ContentPart, Role};

    fn text(role: Role, s: &str) -> Message {
        Message {
            role,
            content: vec![ContentPart::Text {
                text: s.to_string(),
            }],
        }
    }

    #[test]
    fn append_then_view_is_monotonic() {
        let mut buf: Vec<Message> = Vec::new();
        let mut history = History::new(&mut buf);

        assert!(history.is_empty());
        assert_eq!(history.len(), 0);

        history.append(text(Role::User, "a"));
        history.append(text(Role::Assistant, "b"));
        history.append(text(Role::Tool, "c"));

        let view = history.view();
        assert_eq!(view.len(), 3);
        assert!(matches!(view[0].role, Role::User));
        assert!(matches!(view[1].role, Role::Assistant));
        assert!(matches!(view[2].role, Role::Tool));
    }

    #[test]
    fn len_tracks_underlying_vec() {
        let mut buf: Vec<Message> = vec![text(Role::User, "seed")];
        let history = History::new(&mut buf);
        assert_eq!(history.len(), 1);
        assert!(!history.is_empty());
    }
}
