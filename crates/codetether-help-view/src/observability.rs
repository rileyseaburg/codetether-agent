//! Help observability content and presentation.
use crate::blank::blank;
use crate::cmd_row::cmd_row;
use crate::heading::heading;
use ratatui::text::Line;
pub(crate) fn append(lines: &mut Vec<Line<'static>>) {
    lines.push(heading("Protocol & Observability"));
    lines.push(cmd_row("/bus", "/b", "Protocol bus log"));
    lines.push(cmd_row("/protocol", "/p", "Protocol bus (alias)"));
    lines.push(cmd_row("/swarm", "/w", "Swarm agent view"));
    lines.push(cmd_row("/ralph", "/r", "Ralph PRD loop view"));
    lines.push(cmd_row("/forage", "", "Scan OKRs ('execute' runs top one)"));
    lines.push(cmd_row("/latency", "", "Provider + tool latency inspector"));
    lines.push(cmd_row(
        "/inspector",
        "",
        "Token metrics & tool call inspector",
    ));
    lines.push(blank());
}
