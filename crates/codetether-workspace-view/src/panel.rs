use ratatui::{prelude::*, widgets::*};

use codetether_workspace_snapshot::{WorkspaceEntryKind, WorkspaceSnapshot};

/// Render captured workspace entries and Git metadata.
/// # Arguments
/// * `f` - Frame receiving the panel.
/// * `snapshot` - Previously captured workspace data.
/// * `area` - Available panel rectangle.
/// # Returns
/// Draws the panel into the frame.
/// # Examples
/// ```no_run
/// let backend = ratatui::backend::TestBackend::new(26, 20);
/// let mut terminal = ratatui::Terminal::new(backend).unwrap();
/// let snapshot = codetether_workspace_snapshot::WorkspaceSnapshot::default();
/// terminal.draw(|frame| {
///     let area = frame.area();
///     codetether_workspace_view::render(frame, &snapshot, area);
/// }).unwrap();
/// ```
pub fn render(f: &mut Frame, snapshot: &WorkspaceSnapshot, area: Rect) {
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
                codetether_util::truncate_bytes_safe(&entry.name, 20).to_string(),
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
