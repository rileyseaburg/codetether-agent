use serde_json::{json, Value};

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
