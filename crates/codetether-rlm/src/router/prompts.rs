//! Query and system-prompt builders for RLM analysis.

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
        "session_context" | "context_reset" => context_compaction_prompt(),
        _ => "Summarize this output concisely, extracting the most important information.".into(),
    }
}

fn context_compaction_prompt() -> String {
    "You are a CONTEXT MEMORY SYSTEM. Create a BRIEFING for an AI assistant to continue this conversation.\n\n\
     CRITICAL: The assistant will ONLY see your briefing - it has NO memory of the conversation.\n\n\
     ## What to Extract\n\n\
     1. **PRIMARY GOAL**: What is the user ultimately trying to achieve?\n\
     2. **CURRENT STATE**: What has been accomplished? Current status?\n\
     3. **LAST ACTIONS**: What just happened? (last 3-5 tool calls, their results)\n\
     4. **ACTIVE FILES**: Which files were modified?\n\
     5. **PENDING TASKS**: What remains to be done?\n\
     6. **CRITICAL DETAILS**: File paths, error messages, specific values, decisions made\n\
     7. **NEXT STEPS**: What should happen next?\n\n\
     Be SPECIFIC with file paths, function names, error messages.".into()
}
