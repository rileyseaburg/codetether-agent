//! Legacy executor extract code.

/// Extract code from LLM response
/// Select fenced code, or recognize supported DSL command lines.
///
/// # Arguments
/// * `text` - Model response to inspect.
/// # Returns
/// Joined code lines in their original order.
/// # Examples
/// ```
/// assert_eq!(codetether_rlm::legacy_support::extract_code("head(2)"), "head(2)");
/// ```
pub fn extract_code(text: &str) -> String {
    // Look for fenced code blocks
    let mut code_lines = Vec::new();
    let mut in_code_block = false;

    for line in text.lines() {
        if line.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            code_lines.push(line);
        }
    }

    if !code_lines.is_empty() {
        return code_lines.join("\n");
    }

    // If no code blocks, look for lines that look like code
    text.lines()
        .filter(|line| {
            let l = line.trim();
            l.starts_with("head(")
                || l.starts_with("tail(")
                || l.starts_with("grep(")
                || l.starts_with("count(")
                || l.starts_with("llm_query(")
                || l.starts_with("ast_query(")
                || l.starts_with("FINAL(")
                || l.starts_with("let ")
                || l.starts_with("const ")
                || l.starts_with("print")
                || l.starts_with("console.")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
