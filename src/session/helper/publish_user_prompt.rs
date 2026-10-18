//! Native provenance adapter for shared human-prompt publication.
use crate::session::Session;
pub(crate) fn publish(session: &Session, message: &str, correlation_id: Option<String>) {
    let context = codetether_bus_core::user_prompt::Context {
        agent: &session.agent,
        session_id: &session.id,
        workspace: session.metadata.directory.as_deref(),
    };
    codetether_bus_core::user_prompt::publish(
        session.bus.as_ref(),
        &context,
        message,
        correlation_id,
    );
}
