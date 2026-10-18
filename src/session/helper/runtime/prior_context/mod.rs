//! User-directed access policy for prior session context.

mod blocking;
mod directive;
use codetether_prior_context_policy as directive_message;
mod registry;
mod session_state;

#[cfg(test)]
use crate::provider::Message;

/// Resolve prior-context access from an in-memory message slice.
#[cfg(test)]
pub(crate) fn allowed(messages: &[Message]) -> bool {
    directive::resolve(messages)
}

#[cfg(test)]
pub(crate) use blocking::{for_messages as block, serialized_messages as block_serialized};
pub(crate) use blocking::{
    runtime_context as block_runtime_context, serialized_session as block_serialized_for_session,
};
#[cfg(test)]
pub(crate) use registry::messages_available as tool_available;
pub(crate) use registry::{remove_tools, session_available as tool_available_for_session};
pub(crate) use session_state::allowed as allowed_for_session;

#[cfg(test)]
mod tests;

codetether_prior_context_access::impl_session_ingress!(crate::session::Session);
