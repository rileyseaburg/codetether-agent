//! Bound tool output by line and Unicode character counts.

const MAX_TOOL_OUTPUT_LINES: usize = 1200;

const MAX_TOOL_OUTPUT_CHARS: usize = 120_000;

fn truncate_chars(input: &str, max_chars: usize) -> String {
    if input.chars().count() <= max_chars {
        return input.to_string();
    }
    input.chars().take(max_chars).collect()
}

pub(super) fn clamp_tool_output(output: String, tool_name: &str) -> String {
    let original_lines = output.lines().count();
    let original_chars = output.chars().count();
    let mut clamped = output;
    let mut trimmed = false;

    if original_lines > MAX_TOOL_OUTPUT_LINES {
        clamped = clamped
            .lines()
            .take(MAX_TOOL_OUTPUT_LINES)
            .collect::<Vec<_>>()
            .join("\n");
        trimmed = true;
    }

    if clamped.chars().count() > MAX_TOOL_OUTPUT_CHARS {
        clamped = truncate_chars(&clamped, MAX_TOOL_OUTPUT_CHARS);
        trimmed = true;
    }

    if trimmed {
        clamped.push_str(&format!(
            "\n\n[RLM TOOL OUTPUT TRUNCATED by {tool_name}: {original_lines} lines/{original_chars} chars → <= {MAX_TOOL_OUTPUT_LINES} lines/{MAX_TOOL_OUTPUT_CHARS} chars]"
        ));
    }

    clamped
}
