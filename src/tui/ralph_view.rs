//! Ralph view for the TUI
//!
//! Displays real-time status of the autonomous PRD-driven Ralph loop,
//! showing story progress, quality gate results, and per-story sub-agent activity.

pub use codetether_ralph_view::{RalphEvent, RalphStoryInfo, RalphStoryStatus, RalphViewState};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph, Wrap},
};

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

/// Render the Ralph view
pub fn render_ralph_view(f: &mut Frame, state: &mut RalphViewState, area: Rect) {
    if state.detail_mode {
        render_story_detail(f, state, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Header: project/feature + iteration
            Constraint::Length(3), // Progress bar
            Constraint::Min(1),    // Story list
            Constraint::Length(3), // Footer / status
        ])
        .split(area);

    render_header(f, state, chunks[0]);
    render_progress(f, state, chunks[1]);
    render_story_list(f, state, chunks[2]);
    render_footer(f, state, chunks[3]);
}

fn render_header(f: &mut Frame, state: &RalphViewState, area: Rect) {
    let (pending, running, passed, failed) = state.status_counts();
    let total = state.stories.len();

    let title = if state.complete {
        if state.error.is_some() {
            " Ralph [ERROR] "
        } else if failed > 0 {
            " Ralph [PARTIAL] "
        } else {
            " Ralph [COMPLETE] "
        }
    } else {
        " Ralph [ACTIVE] "
    };

    let border_color = if state.complete {
        if state.error.is_some() || failed > 0 {
            Color::Red
        } else {
            Color::Green
        }
    } else {
        Color::Magenta
    };

    let lines = vec![
        Line::from(vec![
            Span::styled("Project: ", Style::default().fg(Color::DarkGray)),
            Span::styled(&state.project, Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled("Feature: ", Style::default().fg(Color::DarkGray)),
            Span::styled(&state.feature, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                format!(
                    "Iteration: {}/{}",
                    state.current_iteration, state.max_iterations
                ),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw("  "),
            Span::styled(format!("⏳{}", pending), Style::default().fg(Color::Yellow)),
            Span::raw(" "),
            Span::styled(format!("⚡{}", running), Style::default().fg(Color::Cyan)),
            Span::raw(" "),
            Span::styled(format!("✓{}", passed), Style::default().fg(Color::Green)),
            Span::raw(" "),
            Span::styled(format!("✗{}", failed), Style::default().fg(Color::Red)),
            Span::raw(" "),
            Span::styled(format!("/{}", total), Style::default().fg(Color::DarkGray)),
        ]),
    ];

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(border_color)),
    );
    f.render_widget(paragraph, area);
}

fn render_progress(f: &mut Frame, state: &RalphViewState, area: Rect) {
    let progress = state.progress();
    let (_, _, passed, _) = state.status_counts();
    let total = state.stories.len();
    let label = format!("{}/{} stories — {:.0}%", passed, total, progress);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Progress "))
        .gauge_style(Style::default().fg(Color::Magenta).bg(Color::DarkGray))
        .percent(progress as u16)
        .label(label);

    f.render_widget(gauge, area);
}

fn render_story_list(f: &mut Frame, state: &mut RalphViewState, area: Rect) {
    state.list_state.select(Some(state.selected_index));

    let items: Vec<ListItem> = state
        .stories
        .iter()
        .map(|story| {
            let (icon, color) = match story.status {
                RalphStoryStatus::Pending => ("○", Color::DarkGray),
                RalphStoryStatus::Blocked => ("⊘", Color::Yellow),
                RalphStoryStatus::Running => ("●", Color::Cyan),
                RalphStoryStatus::QualityCheck => ("◎", Color::Magenta),
                RalphStoryStatus::Passed => ("✓", Color::Green),
                RalphStoryStatus::Failed => ("✗", Color::Red),
            };

            let mut spans = vec![
                Span::styled(format!("{} ", icon), Style::default().fg(color)),
                Span::styled(
                    format!("[{}] ", story.id),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(&story.title, Style::default().fg(Color::White)),
            ];

            // Show current tool for running stories
            if story.status == RalphStoryStatus::Running {
                if let Some(ref tool) = story.current_tool {
                    spans.push(Span::styled(
                        format!(" → {}", tool),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::DIM),
                    ));
                }
                if story.steps > 0 {
                    spans.push(Span::styled(
                        format!(" (step {})", story.steps),
                        Style::default().fg(Color::DarkGray),
                    ));
                }
            }

            // Show quality check status
            if story.status == RalphStoryStatus::QualityCheck {
                let qc_summary: Vec<&str> = story
                    .quality_checks
                    .iter()
                    .map(|(name, _passed)| name.as_str())
                    .collect();
                if !qc_summary.is_empty() {
                    let checks: String = story
                        .quality_checks
                        .iter()
                        .map(|(name, passed)| {
                            if *passed {
                                format!("✓{}", name)
                            } else {
                                format!("✗{}", name)
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ");
                    spans.push(Span::styled(
                        format!(" [{}]", checks),
                        Style::default().fg(Color::Magenta),
                    ));
                }
            }

            // Show passed quality checks for completed stories
            if story.status == RalphStoryStatus::Passed && !story.quality_checks.is_empty() {
                let all_passed = story.quality_checks.iter().all(|(_, p)| *p);
                if all_passed {
                    spans.push(Span::styled(
                        " ✓QC",
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::DIM),
                    ));
                }
            }

            ListItem::new(Line::from(spans))
        })
        .collect();

    let title = if state.stories.is_empty() {
        " Stories (loading...) "
    } else {
        " Stories (↑↓:select  Enter:detail) "
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, area, &mut state.list_state);
}

/// Render full-screen detail view for the selected story's sub-agent
fn render_story_detail(f: &mut Frame, state: &RalphViewState, area: Rect) {
    let story = match state.selected_story() {
        Some(s) => s,
        None => {
            let p = Paragraph::new("No story selected").block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Story Detail "),
            );
            f.render_widget(p, area);
            return;
        }
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6), // Story info header
            Constraint::Min(1),    // Content
            Constraint::Length(1), // Key hints
        ])
        .split(area);

    // --- Header ---
    let (status_text, status_color) = match story.status {
        RalphStoryStatus::Pending => ("○ Pending", Color::DarkGray),
        RalphStoryStatus::Blocked => ("⊘ Blocked", Color::Yellow),
        RalphStoryStatus::Running => ("● Running", Color::Cyan),
        RalphStoryStatus::QualityCheck => ("◎ Quality Check", Color::Magenta),
        RalphStoryStatus::Passed => ("✓ Passed", Color::Green),
        RalphStoryStatus::Failed => ("✗ Failed", Color::Red),
    };

    let header_lines = vec![
        Line::from(vec![
            Span::styled("Story: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} — {}", story.id, story.title),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
            Span::styled(status_text, Style::default().fg(status_color)),
            Span::raw("  "),
            Span::styled("Priority: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}", story.priority),
                Style::default().fg(Color::White),
            ),
            Span::raw("  "),
            Span::styled("Steps: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}", story.steps),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Deps: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                if story.depends_on.is_empty() {
                    "none".to_string()
                } else {
                    story.depends_on.join(", ")
                },
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        // Quality checks row
        Line::from({
            let mut spans = vec![Span::styled(
                "Quality: ",
                Style::default().fg(Color::DarkGray),
            )];
            if story.quality_checks.is_empty() {
                spans.push(Span::styled(
                    "not run yet",
                    Style::default().fg(Color::DarkGray),
                ));
            } else {
                for (name, passed) in &story.quality_checks {
                    let (icon, color) = if *passed {
                        ("✓", Color::Green)
                    } else {
                        ("✗", Color::Red)
                    };
                    spans.push(Span::styled(
                        format!("{}{} ", icon, name),
                        Style::default().fg(color),
                    ));
                }
            }
            spans
        }),
    ];

    let title = format!(" Story Detail: {} ", story.id);
    let header = Paragraph::new(header_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(status_color)),
    );
    f.render_widget(header, chunks[0]);

    // --- Content: tool history, messages, output, error ---
    let mut content_lines: Vec<Line> = Vec::new();

    // Tool call history
    if !story.tool_call_history.is_empty() {
        content_lines.push(Line::from(Span::styled(
            "─── Tool Call History ───",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        content_lines.push(Line::from(""));

        for (i, tc) in story.tool_call_history.iter().enumerate() {
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
    } else if story.steps > 0 {
        content_lines.push(Line::from(Span::styled(
            format!("─── {} tool calls (no detail captured) ───", story.steps),
            Style::default().fg(Color::DarkGray),
        )));
        content_lines.push(Line::from(""));
    }

    // Messages
    if !story.messages.is_empty() {
        content_lines.push(Line::from(Span::styled(
            "─── Conversation ───",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        content_lines.push(Line::from(""));

        for msg in &story.messages {
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

    // Output
    if let Some(ref output) = story.output {
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

    // Merge summary
    if let Some(ref summary) = story.merge_summary {
        content_lines.push(Line::from(Span::styled(
            "─── Merge ───",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )));
        content_lines.push(Line::from(""));
        content_lines.push(Line::from(Span::styled(
            summary.as_str(),
            Style::default().fg(Color::White),
        )));
        content_lines.push(Line::from(""));
    }

    // Error
    if let Some(ref err) = story.error {
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

    // --- Key hints ---
    let hints = Paragraph::new(Line::from(vec![
        Span::styled(" Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": Back  "),
        Span::styled("PgUp/PgDn", Style::default().fg(Color::Yellow)),
        Span::raw(": Scroll  "),
        Span::styled("↑/↓", Style::default().fg(Color::Yellow)),
        Span::raw(": Prev/Next story"),
    ]));
    f.render_widget(hints, chunks[2]);
}

fn render_footer(f: &mut Frame, state: &RalphViewState, area: Rect) {
    let content = if let Some(ref status) = state.final_status {
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                status.as_str(),
                Style::default().fg(if status.contains("Completed") {
                    Color::Green
                } else {
                    Color::Yellow
                }),
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
        .block(Block::default().borders(Borders::ALL).title(" Status "))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

/// Truncate a string for display
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
