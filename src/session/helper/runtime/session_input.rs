//! Authoritative runtime fields derived from a live session.

use crate::session::Session;
use serde_json::{Value, json};
use std::path::Path;

/// Enrich tool input with identity and the live prior-context ceiling.
pub(crate) fn enrich(input: &Value, cwd: &Path, session: &Session) -> Value {
    enrich_with_model(input, cwd, session, session.metadata.model.as_deref())
}

/// Enrich session input while using a caller-resolved model selector.
pub(crate) fn enrich_with_model(
    input: &Value,
    cwd: &Path,
    session: &Session,
    model: Option<&str>,
) -> Value {
    let mut enriched = super::context::enrich_tool_input_with_runtime_context(
        input,
        cwd,
        model,
        &session.id,
        &session.agent,
        session.metadata.provenance.as_ref(),
    );
    if let Value::Object(fields) = &mut enriched {
        fields.insert(
            "__ct_prior_context_allowed".to_string(),
            json!(super::prior_context::allowed_for_session(session)),
        );
    }
    enriched
}
