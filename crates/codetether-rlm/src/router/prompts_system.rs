//! System prompt and exploration summary builders.

/// Build the system prompt for the RLM analysis loop.
pub fn build_system_prompt(input_tokens: usize, tool_id: &str, query: &str) -> String {
    let ctx_type = if matches!(tool_id, "session_context" | "context_reset") {
        "conversation history"
    } else {
        "tool output"
    };
    format!(
        r#"You are tasked with analyzing large content that cannot fit in a normal context window.

The content is a {ctx_type} with {input_tokens} total tokens.

YOUR TASK: {query}

## Analysis Strategy

1. First, examine the exploration (head + tail of content) to understand structure
2. Identify the most important information for answering the query
3. Focus on: errors, key decisions, file paths, recent activity
4. Provide a concise but complete answer

When ready, call FINAL("your detailed answer") with your findings.

Be SPECIFIC - include actual file paths, function names, error messages. Generic summaries are not useful."#
    )
}

/// Build a head/tail exploration summary of `content`.
pub fn build_exploration_summary(content: &str, input_tokens: usize) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let total = lines.len();
    let head = lines.iter().take(30).copied().collect::<Vec<_>>().join("\n");
    let tail = lines.iter().rev().take(50).collect::<Vec<_>>().into_iter().rev().copied().collect::<Vec<_>>().join("\n");
    format!(
        "=== CONTEXT EXPLORATION ===\n\
         Total: {} chars, {total} lines, ~{input_tokens} tokens\n\n\
         === FIRST 30 LINES ===\n{head}\n\n\
         === LAST 50 LINES ===\n{tail}\n\
         === END EXPLORATION ===",
        content.len()
    )
}
