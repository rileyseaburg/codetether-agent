//! Supply conversation messages to shared image-tool argument enrichment.

use crate::session::Session;
use serde_json::Value;

pub(super) fn enrich(name: &str, call_id: &str, session: &Session, input: Value) -> Value {
    codetether_agent_core::execution::images::enrich(name, call_id, &session.messages, input)
}
