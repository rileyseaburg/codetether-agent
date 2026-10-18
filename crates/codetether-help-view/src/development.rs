//! Help development content and presentation.
use crate::blank::blank;
use crate::cmd_row::cmd_row;
use crate::heading::heading;
use ratatui::text::Line;
pub(crate) fn append(lines: &mut Vec<Line<'static>>) {
    lines.push(heading("Development Tools"));
    lines.push(cmd_row("/lsp", "", "LSP diagnostics view"));
    lines.push(cmd_row("/rlm", "", "RLM processing view"));
    lines.push(cmd_row("/symbols", "/sym", "Workspace symbol search"));
    lines.push(cmd_row("/keys", "", "Print all commands to status bar"));
    lines.push(blank());
}
