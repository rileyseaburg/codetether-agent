use crate::provider::ToolDefinition;
use serde_json::{Value, json};

pub fn inject_tool_prompt(base_prompt: &str, tools: &[ToolDefinition]) -> String {
    let tool_lines: String = tools
        .iter()
        .map(|t| format!("- {}: {}", t.name, t.description))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "{base_prompt}\n\n\
         # Tool Use\n\
         You have access to the following tools. To use a tool, output a \
         <tool_call> XML block with a JSON object containing \"name\" and \
         \"arguments\" fields. You may output multiple tool calls in one response.\n\n\
         Example:\n\
         <tool_call>\n\
         {{\"name\": \"bash\", \"arguments\": {{\"command\": \"ls /tmp\"}}}}\n\
         </tool_call>\n\n\
         Available tools:\n{tool_lines}\n\n\
         RULES:\n\
         1. To use a tool, output <tool_call> blocks. Do NOT describe or \
         simulate tool usage in plain text. Do NOT fabricate tool output.\n\
         2. After a tool result is returned, review the result and provide \
         your final answer in plain text WITHOUT any <tool_call> blocks. \
         Only call another tool if the result was insufficient.\n\
         3. Do NOT call the same tool with the same arguments more than once.\n\
         4. Prefer the lsp tool for code intelligence (symbols/definitions/references). \
         Prefer the bash tool for shell commands.\n\
         5. During refactors, NEVER create placeholder/stub implementations \
         (e.g., TODO, FIXME, \"not implemented\", \"fallback\"). Always preserve \
         concrete behavior."
    )
}

pub fn list_tools_bootstrap_definition() -> ToolDefinition {
    ToolDefinition {
        name: "list_tools".to_string(),
        description: "List available tools (optionally filter by query) or fetch a full schema for one tool via {\"tool\":\"name\"}.".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Optional substring filter over tool names/descriptions"
                },
                "tool": {
                    "type": "string",
                    "description": "Optional exact tool name to return full schema"
                }
            }
        }),
    }
}

pub fn list_tools_bootstrap_output(tools: &[ToolDefinition], tool_input: &Value) -> String {
    let requested_tool = tool_input
        .get("tool")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty());

    if let Some(name) = requested_tool {
        if let Some(found) = tools.iter().find(|t| t.name.eq_ignore_ascii_case(name)) {
            return serde_json::to_string_pretty(&json!({
                "mode": "tool",
                "tool": {
                    "name": found.name,
                    "description": found.description,
                    "parameters": found.parameters
                }
            }))
            .unwrap_or_else(|_| format!("tool: {}", found.name));
        }
        return serde_json::to_string_pretty(&json!({
            "mode": "tool",
            "error": format!("Tool '{}' not found", name)
        }))
        .unwrap_or_else(|_| format!("Tool '{}' not found", name));
    }

    let query = tool_input
        .get("query")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_ascii_lowercase());

    let mut listed: Vec<Value> = tools
        .iter()
        .filter(|t| {
            if let Some(q) = &query {
                t.name.to_ascii_lowercase().contains(q)
                    || t.description.to_ascii_lowercase().contains(q)
            } else {
                true
            }
        })
        .map(|t| {
            json!({
                "name": t.name,
                "description": t.description,
            })
        })
        .collect();

    listed.sort_by(|a, b| {
        let an = a["name"].as_str().unwrap_or_default();
        let bn = b["name"].as_str().unwrap_or_default();
        an.cmp(bn)
    });

    serde_json::to_string_pretty(&json!({
        "mode": "list",
        "count": listed.len(),
        "tools": listed,
        "hint": "Call list_tools with {\"tool\":\"<name>\"} to fetch full parameter schema for one tool."
    }))
    .unwrap_or_else(|_| "{\"mode\":\"list\",\"error\":\"serialization_failed\"}".to_string())
}
