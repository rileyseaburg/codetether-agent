//! Smart argument preview for tool calls in the activity panel.

use serde_json::Value;

use super::arg_preview_helpers::{
    format_k8s, format_read_file, format_write, str_or, string_field,
};
use crate::tui::app::text::truncate_preview;

const MAX: usize = 120;

/// Render a one-line human-readable preview of a tool call's JSON arguments.
/// Falls back to the raw string (truncated) for unknown tools or invalid JSON.
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
        "question" => str_or(&value, "question"),
        n if n.starts_with("mcp_k8s_") => format_k8s(&value),
        _ => serde_json::to_string(&value).unwrap_or_default(),
    };
    truncate_preview(&raw, MAX)
}
