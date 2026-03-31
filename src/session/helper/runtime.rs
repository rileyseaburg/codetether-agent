use crate::provenance::sign_provenance;
use serde_json::{Value, json};

pub fn is_interactive_tool(tool_name: &str) -> bool {
    matches!(tool_name, "question")
}

pub fn is_local_cuda_provider(provider: &str) -> bool {
    matches!(provider, "local-cuda" | "local_cuda" | "localcuda")
}

pub fn local_cuda_light_system_prompt() -> String {
    std::env::var("CODETETHER_LOCAL_CUDA_SYSTEM_PROMPT").unwrap_or_else(|_| {
        "You are CodeTether local CUDA assistant. Be concise and execution-focused. \
Use tools only when needed. For tool discovery, call list_tools first, then call specific tools with valid JSON arguments. \
Do not invent tool outputs."
            .to_string()
    })
}

pub fn enrich_tool_input_with_runtime_context(
    tool_input: &Value,
    current_model: Option<&str>,
    session_id: &str,
    agent_name: &str,
    provenance: Option<&crate::provenance::ExecutionProvenance>,
) -> Value {
    let mut enriched = tool_input.clone();
    if let Value::Object(ref mut obj) = enriched {
        if let Some(model) = current_model {
            obj.entry("__ct_current_model".to_string())
                .or_insert_with(|| json!(model));
        }
        obj.entry("__ct_session_id".to_string())
            .or_insert_with(|| json!(session_id));
        obj.entry("__ct_agent_name".to_string())
            .or_insert_with(|| json!(agent_name));
        if let Some(provenance) = provenance {
            obj.entry("__ct_provenance_id".to_string())
                .or_insert_with(|| json!(provenance.provenance_id));
            obj.entry("__ct_origin".to_string())
                .or_insert_with(|| json!(provenance.identity.origin.as_str()));
            if let Some(worker_id) = provenance.identity.worker_id.as_deref() {
                obj.entry("__ct_worker_id".to_string())
                    .or_insert_with(|| json!(worker_id));
            }
            if let Some(task_id) = provenance.task_id.as_deref() {
                obj.entry("__ct_task_id".to_string())
                    .or_insert_with(|| json!(task_id));
            }
            if let Some(run_id) = provenance.run_id.as_deref() {
                obj.entry("__ct_run_id".to_string())
                    .or_insert_with(|| json!(run_id));
            }
            if let Some(attempt_id) = provenance.attempt_id.as_deref() {
                obj.entry("__ct_attempt_id".to_string())
                    .or_insert_with(|| json!(attempt_id));
            }
            if let Some(tenant_id) = provenance.identity.tenant_id.as_deref() {
                obj.entry("__ct_tenant_id".to_string())
                    .or_insert_with(|| json!(tenant_id));
            }
            if let Some(agent_identity_id) = provenance.identity.agent_identity_id.as_deref() {
                obj.entry("__ct_agent_identity_id".to_string())
                    .or_insert_with(|| json!(agent_identity_id));
            }
            if let Some(key_id) = provenance.identity.key_id.as_deref() {
                obj.entry("__ct_key_id".to_string())
                    .or_insert_with(|| json!(key_id));
            }
            if let Some(signature) = sign_provenance(provenance) {
                obj.entry("__ct_signature".to_string())
                    .or_insert_with(|| json!(signature));
            }
        }
    }
    enriched
}

pub fn is_codesearch_no_match_output(tool_name: &str, success: bool, output: &str) -> bool {
    tool_name == "codesearch"
        && success
        && output
            .to_ascii_lowercase()
            .contains("no matches found for pattern:")
}
