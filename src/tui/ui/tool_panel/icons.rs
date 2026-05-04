//! Tool-name → semantic icon and color lookup for the activity panel.

use ratatui::style::Color;

/// Returns a `(icon, color)` pair for the given tool name, grouped by
/// category (read, write, execute, network, etc.).
pub fn tool_icon_and_color(name: &str) -> (&'static str, Color) {
    if name.starts_with("mcp_k8s_") || name.starts_with("kubectl_") {
        return ("☸️", Color::Blue);
    }
    match name {
        "read_file"
        | "list_dir"
        | "search_files"
        | "grep_search"
        | "file_search"
        | "semantic_search"
        | "read_notebook_cell_output"
        | "view_image"
        | "copilot_getNotebookSummary" => ("📖", Color::Blue),
        "write_file"
        | "create_file"
        | "edit_file"
        | "edit_notebook_file"
        | "replace_string_in_file"
        | "multi_replace_string_in_file" => ("✏️", Color::Magenta),
        "bash"
        | "run_command"
        | "run_in_terminal"
        | "run_notebook_cell"
        | "send_to_terminal"
        | "create_and_run_task" => ("⚡", Color::Yellow),
        "question" => ("❓", Color::Cyan),
        "web_fetch" | "web_scrape" | "browser_navigate" | "browser_screenshot"
        | "browser_click" => ("🌐", Color::Green),
        _ => ("🔧", Color::Cyan),
    }
}
