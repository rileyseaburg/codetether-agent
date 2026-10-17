//! Tool-specific hints for analyzing omitted output.

pub(super) fn build_rlm_hint(tool_id: &str, args: &serde_json::Value, tokens: usize) -> String {
    let base = format!("⚠️ OUTPUT TOO LARGE ({tokens} tokens). Use RLM for full analysis:");
    match tool_id {
        "read" => {
            let p = args
                .get("filePath")
                .and_then(|v| v.as_str())
                .unwrap_or("...");
            format!(
                "{base}\n```\nrlm({{ query: \"Analyze this file\", content_paths: [\"{p}\"] }})\n```"
            )
        }
        "bash" => format!(
            "{base}\n```\nrlm({{ query: \"Analyze this command output\", content: \"<paste or use content_paths>\" }})\n```"
        ),
        "grep" => {
            let pat = args
                .get("pattern")
                .and_then(|v| v.as_str())
                .unwrap_or("...");
            let inc = args.get("include").and_then(|v| v.as_str()).unwrap_or("*");
            format!(
                "{base}\n```\nrlm({{ query: \"Summarize search results for {pat}\", content_glob: \"{inc}\" }})\n```"
            )
        }
        _ => format!(
            "{base}\n```\nrlm({{ query: \"Summarize this output\", content: \"...\" }})\n```\n"
        ),
    }
}
