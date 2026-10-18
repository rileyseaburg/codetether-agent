//! Authoritative runtime fields derived from a live session.

use crate::Source;
use serde_json::{Value, json};
use std::path::Path;

/// Enrich tool input with identity and the live prior-context ceiling.
/// # Arguments
/// Receives input JSON, workspace root, session view.
/// # Returns
/// Enriched input JSON, or an unchanged non-object value.
/// # Examples
/// ```
/// # fn example(source: &impl codetether_runtime_input::Source) {
/// let value = codetether_runtime_input::enrich(&serde_json::json!({}), std::path::Path::new("/workspace"), source);
/// assert!(value.is_object());
/// # }
/// ```
pub fn enrich<S: Source>(input: &Value, cwd: &Path, session: &S) -> Value {
    enrich_with_model(input, cwd, session, session.metadata().model.as_deref())
}

/// Enrich input and overwrite its prompt-run lease owner.
/// # Arguments
/// Receives input JSON, workspace root, session view and authoritative lease owner.
/// # Returns
/// Enriched input JSON, or an unchanged non-object value.
/// # Examples
/// ```
/// # fn example(source: &impl codetether_runtime_input::Source) {
/// let value = codetether_runtime_input::enrich_for_turn(&serde_json::json!({}), std::path::Path::new("/workspace"), source, "turn-1");
/// assert!(value.is_object());
/// # }
/// ```
pub fn enrich_for_turn<S: Source>(
    input: &Value,
    cwd: &Path,
    session: &S,
    lease_owner: &str,
) -> Value {
    let mut enriched = enrich(input, cwd, session);
    if let Value::Object(fields) = &mut enriched {
        fields.insert("__ct_lease_owner".into(), json!(lease_owner));
    }
    enriched
}

/// Enrich session input while using a caller-resolved model selector.
/// # Arguments
/// Receives input JSON, workspace root, session view and model override.
/// # Returns
/// Enriched input JSON, or an unchanged non-object value.
/// # Examples
/// ```
/// # fn example(source: &impl codetether_runtime_input::Source) {
/// let value = codetether_runtime_input::enrich_with_model(&serde_json::json!({}), std::path::Path::new("/workspace"), source, Some("model"));
/// assert!(value.is_object());
/// # }
/// ```
pub fn enrich_with_model<S: Source>(
    input: &Value,
    cwd: &Path,
    session: &S,
    model: Option<&str>,
) -> Value {
    let mut enriched = super::context::enrich_tool_input_with_runtime_context(
        input,
        cwd,
        model,
        session.id(),
        session.agent(),
        session.metadata().provenance.as_ref(),
    );
    if let Value::Object(fields) = &mut enriched {
        fields.insert(
            "__ct_prior_context_allowed".to_string(),
            json!(codetether_prior_context_access::allowed(
                session.metadata(),
                session.messages()
            )),
        );
    }
    enriched
}
