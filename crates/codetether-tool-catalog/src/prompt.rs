//! Render fallback tool-call instructions from the available catalog.
use codetether_provider_core::ToolDefinition;

/// Append markup-based tool instructions and catalog descriptions to a prompt.
/// # Arguments
/// `base_prompt` is the existing system prompt; `tools` is the available catalog.
/// # Returns
/// The complete prompt with tool-call syntax, rules and tool descriptions.
/// # Examples
/// ```
/// let prompt = codetether_tool_catalog::inject_tool_prompt("Assist the user.", &[]);
/// assert!(prompt.starts_with("Assist the user."));
/// assert!(prompt.contains("<tool_call>"));
/// ```
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
