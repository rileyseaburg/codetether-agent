//! Structured tool-request extraction prompts.

/// Build the extractor system and user prompts.
///
/// # Arguments
/// * `thought_text` - Test-phase thought to inspect.
/// * `allowed_tools` - Tool names permitted for extraction and dispatch.
///
/// # Returns
/// System prompt followed by user prompt.
///
/// # Examples
/// ```
/// # use serde_json::Value;
/// # fn example(thought_text: &str, allowed_tools: &[String]) {
/// let _result = codetether_cognition_core::execution::requests::extraction_prompts(thought_text, allowed_tools);
/// # }
/// ```
pub fn extraction_prompts(thought_text: &str, allowed_tools: &[String]) -> (String, String) {
    // Structured extraction: ask LLM for tool requests
    let system_prompt = "You are a tool request extractor. \
Given a test/check thought, determine if any tools should be invoked. \
Return ONLY valid JSON, no markdown fences. \
If no tools are needed, return {\"tool_requests\":[]}."
        .to_string();

    let available: Vec<&str> = allowed_tools.iter().map(|s| s.as_str()).collect();
    let user_prompt = format!(
        "Available tools: {tools}\n\nThought:\n{thought}\n\n\
Return JSON only: {{ \"tool_requests\": [{{ \"tool\": \"tool-name\", \"arguments\": {{...}} }}] }}",
        tools = available.join(", "),
        thought = thought_text
    );

    (system_prompt, user_prompt)
}
