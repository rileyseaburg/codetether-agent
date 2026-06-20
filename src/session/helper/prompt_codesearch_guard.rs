//! Detects codesearch no-match thrash in the build agent loop.
//!
//! When a build agent runs codesearch repeatedly with no matches, it is
//! usually retrying query variants fruitlessly. This nudges it to stop.

use crate::session::helper::loop_constants::{
    CODESEARCH_THRASH_NUDGE, MAX_CONSECUTIVE_CODESEARCH_NO_MATCHES,
};
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;

/// Update the no-match counter and, when the thrash threshold is hit, append a
/// nudge to the session and return `true` (caller should break the tool loop).
pub(super) fn guard(
    session: &mut Session,
    is_build: bool,
    codesearch_no_match: bool,
    counter: &mut u32,
    step: usize,
) -> bool {
    if !is_build {
        return false;
    }
    *counter = if codesearch_no_match { *counter + 1 } else { 0 };
    if *counter < MAX_CONSECUTIVE_CODESEARCH_NO_MATCHES {
        return false;
    }
    tracing::warn!(
        step,
        consecutive = *counter,
        "Detected codesearch no-match thrash; nudging model to stop variant retries"
    );
    session.add_message(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: CODESEARCH_THRASH_NUDGE.to_string(),
        }],
    });
    true
}