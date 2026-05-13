//! Smart truncation with RLM hint generation.

use crate::chunker::RlmChunker;
use tracing::info;

/// Truncate output to `max_tokens` with head/tail preservation.
///
/// Returns `(truncated_text, was_truncated, original_token_count)`.
pub fn smart_truncate(output: &str, tool_id: &str, tool_args: &serde_json::Value,
    max_tokens: usize) -> (String, bool, usize) {
    let estimated = RlmChunker::estimate_tokens(output);
    if estimated <= max_tokens {
        return (output.to_string(), false, estimated);
    }

    info!(
        tool = tool_id,
        original_tokens = estimated,
        max_tokens,
        "Smart truncating large output"
    );

    let max_chars = max_tokens * 4;
    let head_chars = (max_chars as f64 * 0.6) as usize;
    let tail_chars = (max_chars as f64 * 0.3) as usize;

    let head: String = output.chars().take(head_chars).collect();
    let tail: String = output.chars().rev().take(tail_chars).collect::<String>().chars().rev().collect();

    let omitted = estimated - RlmChunker::estimate_tokens(&head) - RlmChunker::estimate_tokens(&tail);
    let hint = build_rlm_hint(tool_id, tool_args, estimated);

    let truncated = format!("{}\n\n[... {} tokens truncated ...]\n\n{}\n\n{}", head, omitted, hint, tail);
    (truncated, true, estimated)
}

fn build_rlm_hint(tool_id: &str, args: &serde_json::Value, tokens: usize) -> String {
    let base = format!("⚠️ OUTPUT TOO LARGE ({tokens} tokens). Use RLM for full analysis:");
    match tool_id {
        "read" => {
            let p = args.get("filePath").and_then(|v| v.as_str()).unwrap_or("...");
            format!("{base}\n```\nrlm({{ query: \"Analyze this file\", content_paths: [\"{p}\"] }})\n```")
        }
        "bash" => format!("{base}\n```\nrlm({{ query: \"Analyze this command output\", content: \"<paste or use content_paths>\" }})\n```"),
        "grep" => {
            let pat = args.get("pattern").and_then(|v| v.as_str()).unwrap_or("...");
            let inc = args.get("include").and_then(|v| v.as_str()).unwrap_or("*");
            format!("{base}\n```\nrlm({{ query: \"Summarize search results for {pat}\", content_glob: \"{inc}\" }})\n```")
        }
        _ => format!("{base}\n```\nrlm({{ query: \"Summarize this output\", content: \"...\" }})\n```\n"),
    }
}
