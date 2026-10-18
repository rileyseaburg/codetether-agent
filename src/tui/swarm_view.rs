//! Swarm mode view for the TUI
//!
//! Displays real-time status of parallel sub-agent execution,
//! with per-sub-agent detail view showing tool call history,
//! conversation messages, and output.

use codetether_swarm_core::SubTaskStatus;
pub use codetether_swarm_view::{SubTaskInfo, SwarmEvent, SwarmViewState};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListState, Paragraph, Wrap},
};

#[path = "swarm_view_detail.rs"]
mod swarm_view_detail;
#[path = "swarm_view_fmt.rs"]
pub(crate) mod swarm_view_fmt;
#[path = "swarm_view_list.rs"]
mod swarm_view_list;
#[path = "swarm_view_row.rs"]
mod swarm_view_row;
pub use swarm_view_detail::{AgentMessageEntry, AgentToolCallDetail};

/// Render the swarm view (takes &mut for StatefulWidget rendering)
pub fn render_swarm_view(f: &mut Frame, state: &mut SwarmViewState, area: Rect) {
    // If in detail mode, render full-screen detail for selected agent
    if state.detail_mode {
        render_agent_detail(f, state, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header with task + progress
            Constraint::Length(3), // Stage progress
            Constraint::Min(1),    // Subtask list
            Constraint::Length(3), // Stats
        ])
        .split(area);

    // Header
    render_header(f, state, chunks[0]);

    // Stage progress
    render_stage_progress(f, state, chunks[1]);

    // Subtask list (stateful for selection)
    render_subtask_list(f, state, chunks[2]);

    // Stats footer
    render_stats(f, state, chunks[3]);
}

fn render_header(f: &mut Frame, state: &SwarmViewState, area: Rect) {
    let (pending, running, completed, failed) = state.status_counts();
    let total = state.subtasks.len();

    let title = if state.complete {
        if state.error.is_some() {
            " Swarm [ERROR] "
        } else {
            " Swarm [COMPLETE] "
        }
    } else if state.active {
        " Swarm [ACTIVE] "
    } else {
        " Swarm [IDLE] "
    };

    let status_line = Line::from(vec![
        Span::styled("Task: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            truncate_str(&state.task, 47),
            Style::default().fg(Color::White),
        ),
        Span::raw("  "),
        Span::styled(format!("⏳{}", pending), Style::default().fg(Color::Yellow)),
        Span::raw(" "),
        Span::styled(format!("⚡{}", running), Style::default().fg(Color::Cyan)),
        Span::raw(" "),
        Span::styled(format!("✓{}", completed), Style::default().fg(Color::Green)),
        Span::raw(" "),
        Span::styled(format!("✗{}", failed), Style::default().fg(Color::Red)),
        Span::raw(" "),
        Span::styled(format!("/{}", total), Style::default().fg(Color::DarkGray)),
    ]);

    let paragraph = Paragraph::new(status_line).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(if state.complete {
                if state.error.is_some() {
                    Color::Red
                } else {
                    Color::Green
                }
            } else {
                Color::Cyan
            })),
    );

    f.render_widget(paragraph, area);
}

fn render_stage_progress(f: &mut Frame, state: &SwarmViewState, area: Rect) {
    let progress = state.progress();
    let label = format!(
        "Stage {}/{} - {:.0}%",
        state.current_stage.min(state.total_stages),
        state.total_stages,
        progress
    );

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Progress "))
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
        .percent(progress as u16)
        .label(label);

    f.render_widget(gauge, area);
}

fn render_subtask_list(f: &mut Frame, state: &mut SwarmViewState, area: Rect) {
    // Sync ListState selection from selected_index
    state.list_state.select(Some(state.selected_index));

    let items = swarm_view_list::subtask_items(state);

    let title = if state.subtasks.is_empty() {
        " SubTasks (none yet) "
    } else {
        " SubTasks (↑↓:select  Enter:detail) "
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(swarm_view_list::highlight_style())
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, area, &mut state.list_state);
}

/// Render a full-screen detail view for the selected sub-agent
fn render_agent_detail(f: &mut Frame, state: &SwarmViewState, area: Rect) {
    let task = match state.selected_subtask() {
        Some(t) => t,
        None => {
            let p = Paragraph::new("No subtask selected").block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Agent Detail "),
            );
            f.render_widget(p, area);
            return;
        }
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Agent info header
            Constraint::Min(1),    // Content (tool calls + messages + output)
            Constraint::Length(1), // Key hints
        ])
        .split(area);

    // --- Header: agent info ---
    let (status_icon, status_color) = match task.status {
        SubTaskStatus::Pending => ("○ Pending", Color::DarkGray),
        SubTaskStatus::Blocked => ("⊘ Blocked", Color::Yellow),
        SubTaskStatus::Running => ("● Running", Color::Cyan),
        SubTaskStatus::Completed => ("✓ Completed", Color::Green),
        SubTaskStatus::Failed => ("✗ Failed", Color::Red),
        SubTaskStatus::Cancelled => ("⊗ Cancelled", Color::DarkGray),
        SubTaskStatus::TimedOut => ("⏱ Timed Out", Color::Red),
    };

    let agent_label = task.agent_name.as_deref().unwrap_or("(unassigned)");
    let header_lines = vec![
        Line::from(vec![
            Span::styled("Task: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                &task.name,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Agent: ", Style::default().fg(Color::DarkGray)),
            Span::styled(agent_label, Style::default().fg(Color::Cyan)),
            Span::raw("  "),
            Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
            Span::styled(status_icon, Style::default().fg(status_color)),
            Span::raw("  "),
            Span::styled("Stage: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{}", task.stage), Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("Steps: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}/{}", task.steps, task.max_steps),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Deps: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                if task.dependencies.is_empty() {
                    "none".to_string()
                } else {
                    task.dependencies.join(", ")
                },
                Style::default().fg(Color::DarkGray),
            ),
        ]),
    ];

    let title = format!(" Agent Detail: {} ", task.id);
    let header = Paragraph::new(header_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(status_color)),
    );
    f.render_widget(header, chunks[0]);

    // --- Content area: tool history, messages, output ---
    let mut content_lines: Vec<Line> = Vec::new();

    // Tool call history section
    if !task.tool_call_history.is_empty() {
        content_lines.push(Line::from(Span::styled(
            "─── Tool Call History ───",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        content_lines.push(Line::from(""));

        for (i, tc) in task.tool_call_history.iter().enumerate() {
            let icon = if tc.success { "✓" } else { "✗" };
            let icon_color = if tc.success { Color::Green } else { Color::Red };
            content_lines.push(Line::from(vec![
                Span::styled(format!(" {icon} "), Style::default().fg(icon_color)),
                Span::styled(format!("#{} ", i + 1), Style::default().fg(Color::DarkGray)),
                Span::styled(
                    &tc.tool_name,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            if !tc.input_preview.is_empty() {
                content_lines.push(Line::from(vec![
                    Span::raw("     "),
                    Span::styled("in: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        truncate_str(&tc.input_preview, 80),
                        Style::default().fg(Color::White),
                    ),
                ]));
            }
            if !tc.output_preview.is_empty() {
                content_lines.push(Line::from(vec![
                    Span::raw("     "),
                    Span::styled("out: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        truncate_str(&tc.output_preview, 80),
                        Style::default().fg(Color::White),
                    ),
                ]));
            }
        }
        content_lines.push(Line::from(""));
    } else if task.steps > 0 {
        // At least show that tool calls happened even without detail
        content_lines.push(Line::from(Span::styled(
            format!("─── {} tool calls (no detail captured) ───", task.steps),
            Style::default().fg(Color::DarkGray),
        )));
        content_lines.push(Line::from(""));
    }

    // Messages section
    if !task.messages.is_empty() {
        content_lines.push(Line::from(Span::styled(
            "─── Conversation ───",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        content_lines.push(Line::from(""));

        for msg in &task.messages {
            let (role_color, role_label) = match msg.role.as_str() {
                "user" => (Color::White, "USER"),
                "assistant" => (Color::Cyan, "ASST"),
                "tool" => (Color::Yellow, "TOOL"),
                "system" | "developer" => (Color::DarkGray, "SYS "),
                _ => (Color::White, "    "),
            };
            content_lines.push(Line::from(vec![Span::styled(
                format!(" [{role_label}] "),
                Style::default().fg(role_color).add_modifier(Modifier::BOLD),
            )]));
            // Show message content (truncated lines)
            for line in msg.content.lines().take(10) {
                content_lines.push(Line::from(vec![
                    Span::raw("   "),
                    Span::styled(line, Style::default().fg(Color::White)),
                ]));
            }
            if msg.content.lines().count() > 10 {
                content_lines.push(Line::from(vec![
                    Span::raw("   "),
                    Span::styled("... (truncated)", Style::default().fg(Color::DarkGray)),
                ]));
            }
            content_lines.push(Line::from(""));
        }
    }

    // Output section
    if let Some(ref output) = task.output {
        content_lines.push(Line::from(Span::styled(
            "─── Output ───",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )));
        content_lines.push(Line::from(""));
        for line in output.lines().take(20) {
            content_lines.push(Line::from(Span::styled(
                line,
                Style::default().fg(Color::White),
            )));
        }
        if output.lines().count() > 20 {
            content_lines.push(Line::from(Span::styled(
                "... (truncated)",
                Style::default().fg(Color::DarkGray),
            )));
        }
        content_lines.push(Line::from(""));
    }

    // Error section
    if let Some(ref err) = task.error {
        content_lines.push(Line::from(Span::styled(
            "─── Error ───",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )));
        content_lines.push(Line::from(""));
        for line in err.lines() {
            content_lines.push(Line::from(Span::styled(
                line,
                Style::default().fg(Color::Red),
            )));
        }
        content_lines.push(Line::from(""));
    }

    // If nothing to show
    if content_lines.is_empty() {
        content_lines.push(Line::from(Span::styled(
            "  Waiting for agent activity...",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let content = Paragraph::new(content_lines)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: false })
        .scroll((state.detail_scroll as u16, 0));
    f.render_widget(content, chunks[1]);

    // --- Key hints bar ---
    let hints = Paragraph::new(Line::from(vec![
        Span::styled(" Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": Back  "),
        Span::styled("PgUp/PgDn", Style::default().fg(Color::Yellow)),
        Span::raw(": Scroll  "),
        Span::styled("↑/↓", Style::default().fg(Color::Yellow)),
        Span::raw(": Prev/Next agent"),
    ]));
    f.render_widget(hints, chunks[2]);
}

/// Truncate a string for display, respecting char boundaries
fn truncate_str(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.replace('\n', " ")
    } else {
        let mut end = max;
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
        format!("{}...", s[..end].replace('\n', " "))
    }
}

fn render_stats(f: &mut Frame, state: &SwarmViewState, area: Rect) {
    let content = if let Some(ref stats) = state.stats {
        Line::from(vec![
            Span::styled("Time: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:.1}s", stats.execution_time_ms as f64 / 1000.0),
                Style::default().fg(Color::White),
            ),
            Span::raw("  "),
            Span::styled("Speedup: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:.1}x", stats.speedup_factor),
                Style::default().fg(Color::Green),
            ),
            Span::raw("  "),
            Span::styled("Tool calls: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}", stats.total_tool_calls),
                Style::default().fg(Color::White),
            ),
            Span::raw("  "),
            Span::styled("Critical path: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}", stats.critical_path_length),
                Style::default().fg(Color::White),
            ),
        ])
    } else if let Some(ref err) = state.error {
        Line::from(vec![Span::styled(
            format!("Error: {}", err),
            Style::default().fg(Color::Red),
        )])
    } else {
        Line::from(vec![Span::styled(
            "Executing...",
            Style::default().fg(Color::DarkGray),
        )])
    };

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(" Stats "))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}
