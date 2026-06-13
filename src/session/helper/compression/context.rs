//! [`CompressContext`] — session snapshot for buffer-level compression.

use crate::session::Session;

/// Non-buffer inputs for
/// [`compress_messages_keep_last`](super::compress_messages_keep_last) and
/// [`enforce_on_messages`](super::enforce_on_messages).
///
/// Cloned from a [`Session`] by [`CompressContext::from_session`] so the
/// core compression functions can operate on `&mut Vec<Message>` without
/// holding any borrow of the owning [`Session`]. This is the pivot that
/// lets [`derive_context`](crate::session::context::derive_context) run
/// the compression pipeline on a *clone* of the history rather than
/// mutating it in place — Phase A of the history/context split.
///
/// All fields are owned (not borrowed) so the context can be constructed
/// up front and re-used across multiple compression passes without
/// holding an outer borrow of the session.
#[derive(Clone)]
pub(crate) struct CompressContext {
    /// RLM configuration for the session (thresholds, model selectors,
    /// iteration limits).
    pub rlm_config: crate::rlm::RlmConfig,
    /// UUID of the owning session, propagated to RLM traces.
    pub session_id: String,
}

impl CompressContext {
    /// Snapshot the non-buffer fields of `session` into an owned context.
    pub(crate) fn from_session(session: &Session) -> Self {
        Self {
            rlm_config: session.metadata.rlm.clone(),
            session_id: session.id.clone(),
        }
    }
}
