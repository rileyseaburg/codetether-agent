//! Query and system-prompt builders for RLM analysis.
use super::prompts_context::context_compaction_prompt;

/// Build a tool-specific analysis query.
pub fn build_query_for_tool(tool_id: &str, args: &serde_json::Value) -> String {
    match tool_id {
        "read" => {
            let p = args.get("filePath").and_then(|v| v.as_str()).unwrap_or("unknown");
            format!("Summarize the key contents of file \"{p}\". Focus on: structure, main functions/classes, important logic. Be concise.")
        }
        "bash" => "Summarize the command output. Extract key information, results, errors, warnings. Be concise.".into(),
        "grep" => {
            let pat = args.get("pattern").and_then(|v| v.as_str()).unwrap_or("pattern");
            format!("Summarize search results for \"{pat}\". Group by file, highlight most relevant matches. Be concise.")
        }
        "glob" => "Summarize the file listing. Group by directory, highlight important files. Be concise.".into(),
        "webfetch" => {
            let u = args.get("url").and_then(|v| v.as_str()).unwrap_or("unknown");
            format!("Extract and summarize the main human-readable content from this fetched web page (URL: {u}). Ignore scripts, styles, navigation, cookie banners, and boilerplate. Preserve headings, lists, code blocks, and important links. Be concise.")
        }
        "websearch" => {
            let q = args.get("query").and_then(|v| v.as_str()).unwrap_or("unknown");
            format!("Summarize these web search results for query: \"{q}\". List the most relevant results with titles and URLs, and note why each is relevant. Be concise.")
        }
        "batch" => "Summarize the batch tool output. Group by sub-call, highlight failures/errors first, then key results. Keep actionable details: URLs, file paths, command outputs, and next steps.".into(),
        "session_context" | "context_reset" | "proactive_session_context" | "summary_index" => {
            let prompt = context_compaction_prompt();
            match args.get("target_tokens").and_then(serde_json::Value::as_u64) {
                Some(target) => format!("{prompt}\n\nKeep the briefing within {target} tokens."),
                None => prompt,
            }
        }
        _ => "Summarize this output concisely, extracting the most important information.".into(),
    }
}
