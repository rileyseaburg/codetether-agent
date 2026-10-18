//! Compatibility adapter for native image-context tests.
use crate::{agent::Agent, session::Session};
use serde_json::Value;
pub(super) fn enrich(name: &str, call_id: &str, session: &Session, input: Value) -> Value {
    codetether_agent_tools::enrich::<Agent>(name, call_id, session, input)
}
