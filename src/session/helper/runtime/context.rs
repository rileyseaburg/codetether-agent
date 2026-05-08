use super::{path::normalize_workspace_paths, provenance::insert_provenance_fields};
use serde_json::{Map, Value, json};
use std::path::Path;

/// Enrich tool input with session metadata, provenance, and workspace-aware paths.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::helper::runtime::enrich_tool_input_with_runtime_context;
/// use serde_json::json;
/// use std::path::Path;
///
/// let enriched = enrich_tool_input_with_runtime_context(
///     &json!({"path": "src/lib.rs"}),
///     Path::new("/workspace"),
///     Some("example/model"),
///     "session-1",
///     "agent-build",
///     None,
/// );
///
/// assert_eq!(enriched["path"], "/workspace/src/lib.rs");
/// assert_eq!(enriched["__ct_session_id"], "session-1");
/// assert_eq!(enriched["__ct_parent_workspace"], "/workspace");
/// ```
pub fn enrich_tool_input_with_runtime_context(
    tool_input: &Value,
    workspace_dir: &Path,
    current_model: Option<&str>,
    session_id: &str,
    agent_name: &str,
    provenance: Option<&crate::provenance::ExecutionProvenance>,
) -> Value {
    let mut enriched = tool_input.clone();
    if let Value::Object(ref mut obj) = enriched {
        insert_field(obj, "__ct_current_model", current_model);
        obj.entry("__ct_session_id".to_string())
            .or_insert_with(|| json!(session_id));
        obj.entry("__ct_agent_name".to_string())
            .or_insert_with(|| json!(agent_name));
        if let Some(provenance) = provenance {
            insert_provenance_fields(obj, provenance);
        }
        insert_parent_workspace(obj, workspace_dir);
        normalize_workspace_paths(obj, workspace_dir);
    }
    enriched
}

fn insert_parent_workspace(obj: &mut Map<String, Value>, parent_workspace: &Path) {
    obj.insert(
        "__ct_parent_workspace".to_string(),
        json!(parent_workspace.display().to_string()),
    );
}

/// Insert a string field into a JSON object only when the value is present.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::helper::runtime::insert_field;
/// use serde_json::{Map, Value};
///
/// let mut obj = Map::<String, Value>::new();
/// insert_field(&mut obj, "example", Some("value"));
/// insert_field(&mut obj, "missing", None);
///
/// assert_eq!(obj["example"], "value");
/// assert!(!obj.contains_key("missing"));
/// ```
pub fn insert_field(obj: &mut Map<String, Value>, key: &str, value: Option<&str>) {
    if let Some(value) = value {
        obj.entry(key.to_string()).or_insert_with(|| json!(value));
    }
}
