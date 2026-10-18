//! Native session bindings for shared authoritative tool inputs.
use crate::session::Session;
use serde_json::Value;
use std::path::Path;
pub(crate) fn enrich(input: &Value, cwd: &Path, session: &Session) -> Value {
    codetether_runtime_input::enrich(input, cwd, session)
}
pub(crate) fn enrich_for_turn(
    input: &Value,
    cwd: &Path,
    session: &Session,
    lease_owner: &str,
) -> Value {
    codetether_runtime_input::enrich_for_turn(input, cwd, session, lease_owner)
}
pub(crate) fn enrich_with_model(
    input: &Value,
    cwd: &Path,
    session: &Session,
    model: Option<&str>,
) -> Value {
    codetether_runtime_input::enrich_with_model(input, cwd, session, model)
}
