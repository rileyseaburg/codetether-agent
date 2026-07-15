//! Tool-registry filtering for prior-context access policy.

#[cfg(test)]
use crate::provider::Message;
use crate::session::Session;
use crate::tool::ToolRegistry;

/// Return whether a tool is available under transcript-derived policy.
#[cfg(test)]
pub(crate) fn messages_available(messages: &[Message], name: &str) -> bool {
    super::allowed(messages) || !super::call::PRIOR_CONTEXT_TOOLS.contains(&name)
}

/// Return whether a tool is available under durable session policy.
pub(crate) fn session_available(session: &Session, name: &str) -> bool {
    super::session_state::allowed(session) || !super::call::PRIOR_CONTEXT_TOOLS.contains(&name)
}

/// Remove every tool capable of consulting prior context.
pub(crate) fn remove_tools(registry: &mut ToolRegistry) {
    for name in super::call::PRIOR_CONTEXT_TOOLS {
        registry.unregister(name);
    }
}
