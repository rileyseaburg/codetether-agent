//! User-directed access policy for prior session context.

mod actions;
mod blocking;
mod call;
mod delegated;
mod delegation;
mod denial;
mod directive;
mod directive_clause;
mod directive_message;
mod example_line;
mod human;
mod normalize;
mod permission;
mod permission_action;
mod permission_target;
mod quoted;
mod reenable;
mod registry;
mod session_state;
mod session_update;
mod source_preference;
mod targets;

use crate::provider::Message;

/// Resolve prior-context access from an in-memory message slice.
pub(crate) fn allowed(messages: &[Message]) -> bool {
    directive::resolve(messages)
}

pub(crate) use blocking::{
    for_messages as block, runtime_context as block_runtime_context,
    serialized_messages as block_serialized, serialized_session as block_serialized_for_session,
};
pub(crate) use registry::{
    messages_available as tool_available, remove_tools,
    session_available as tool_available_for_session,
};
pub(crate) use session_state::allowed as allowed_for_session;

#[cfg(test)]
mod tests;
