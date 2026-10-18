//! Native test adapter for shared tool-input preparation.
use crate::{agent::Agent, session::Session};
#[cfg(test)]
pub(super) fn prepare(
    session: &Session,
    call_id: &str,
    name: &str,
    arguments: &str,
) -> Result<serde_json::Value, String> {
    codetether_agent_tools::prepare::<Agent>(session, call_id, name, arguments)
}
