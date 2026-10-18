//! Smart argument preview for tool calls in the activity panel.

use serde_json::Value;

use super::arg_preview_helpers::{
    format_k8s, format_read_file, format_write, str_or, string_field,
};
use codetether_util::preview_chars::truncate_preview;

const MAX: usize = 120;

/// Render a one-line human-readable preview of a tool call's JSON arguments.
/// Falls back to the raw string (truncated) for unknown tools or invalid JSON.
///
/// # Arguments
/// * `tool_name` - Tool identifier.
/// * `arguments` - JSON or raw argument text.
///
/// # Returns
/// An abbreviated single-line description.
///
/// # Examples
/// ```
/// assert!(!codetether_tool_panel::arg_preview::smart_arg_preview("bash", r#"{"command":"ls"}"#).is_empty());
/// ```
pub fn smart_arg_preview(tool_name: &str, arguments: &str) -> String {
    let Ok(value) = serde_json::from_str::<Value>(arguments) else {
        return truncate_preview(arguments, MAX);
    };
    let raw = match tool_name {
        "read_file" => format_read_file(&value),
        "list_dir" => format!("📁 {}", string_field(&value, "path")),
        "write_file" | "create_file" => format_write(&value),
        "replace_string_in_file" | "edit_file" => {
            format!("📝 {}", string_field(&value, "filePath"))
        }
        "bash" | "run_in_terminal" => {
            format!("$ {}", truncate_preview(&str_or(&value, "command"), 100))
        }
        "run_notebook_cell" => format!(
            "📓 {} cell {}",
            string_field(&value, "filePath"),
            str_or(&value, "cellId")
        ),
        "grep_search" | "search_files" => format!("🔍 \"{}\"", str_or(&value, "query")),
        "file_search" | "semantic_search" => format!("🔍 {}", str_or(&value, "query")),
        "agent" => super::arg_preview_agent::format(&value),
        "question" => str_or(&value, "question"),
        n if n.starts_with("mcp_k8s_") => format_k8s(&value),
        _ => serde_json::to_string(&value).unwrap_or_default(),
    };
    truncate_preview(&raw, MAX)
}
