use ratatui::{prelude::*, widgets::*};

use crate::tui::{app::state::App, utils::workspace_types::WorkspaceEntryKind};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let snapshot = &app.state.workspace;
    let mut lines = vec![Line::from(format!("Updated {}", snapshot.captured_at))];
    if let Some(branch) = snapshot.git_branch.as_deref() {
        lines.push(Line::from(format!(
            "git {branch} • {} dirty",
            snapshot.git_dirty_files
        )));
    }
    lines.push(Line::from(""));
    let visible = area.height.saturating_sub(6) as usize;
    for entry in snapshot.entries.iter().take(visible) {
        lines.push(Line::from(vec![
            Span::raw(icon(entry.kind)),
            Span::raw(" "),
            Span::styled(
                crate::util::truncate_bytes_safe(&entry.name, 20).to_string(),
                Style::default().fg(Color::Gray),
            ),
        ]));
    }
    if snapshot.entries.is_empty() {
        lines.push(Line::styled(
            "Use /refresh to rescan",
            Style::default().fg(Color::DarkGray),
        ));
    }
    let block = Block::default().borders(Borders::ALL).title(" Workspace ");
    f.render_widget(
        Paragraph::new(lines).block(block).wrap(Wrap { trim: true }),
        area,
    );
}

fn icon(kind: WorkspaceEntryKind) -> &'static str {
    match kind {
        WorkspaceEntryKind::Directory => "📁",
        WorkspaceEntryKind::File => "📄",
    }
}
