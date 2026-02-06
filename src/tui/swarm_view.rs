//! Swarm mode view for the TUI
//!
//! Displays real-time status of parallel sub-agent execution,
//! with per-sub-agent detail view showing tool call history,
//! conversation messages, and output.

use crate::swarm::{SubTaskStatus, SwarmStats};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph, Wrap},
};

/// A recorded tool call for a sub-agent
#[derive(Debug, Clone)]
pub struct AgentToolCallDetail {
    pub tool_name: String,
    pub input_preview: String,
    pub output_preview: String,
    pub success: bool,
}

/// A conversation message entry for a sub-agent
#[derive(Debug, Clone)]
pub struct AgentMessageEntry {
    pub role: String,
    pub content: String,
    pub is_tool_call: bool,
}

/// Events emitted by swarm execution for TUI updates
#[derive(Debug, Clone)]
pub enum SwarmEvent {
    /// Swarm execution started
    Started { task: String, total_subtasks: usize },
    /// Task decomposition complete
    Decomposed { subtasks: Vec<SubTaskInfo> },
    /// SubTask status changed
    SubTaskUpdate {
        id: String,
        name: String,
        status: SubTaskStatus,
        agent_name: Option<String>,
    },
    /// SubAgent started working
    AgentStarted {
        subtask_id: String,
        agent_name: String,
        specialty: String,
    },
    /// SubAgent made a tool call (basic)
    AgentToolCall {
        subtask_id: String,
        tool_name: String,
    },
    /// SubAgent made a tool call with detail
    AgentToolCallDetail {
        subtask_id: String,
        detail: AgentToolCallDetail,
    },
    /// SubAgent sent/received a message
    AgentMessage {
        subtask_id: String,
        entry: AgentMessageEntry,
    },
    /// SubAgent completed
    AgentComplete {
        subtask_id: String,
        success: bool,
        steps: usize,
    },
    /// SubAgent produced output text
    AgentOutput { subtask_id: String, output: String },
    /// SubAgent encountered an error
    AgentError { subtask_id: String, error: String },
    /// Stage completed
    StageComplete {
        stage: usize,
        completed: usize,
        failed: usize,
    },
    /// Swarm execution complete
    Complete { success: bool, stats: SwarmStats },
    /// Error occurred
    Error(String),
}

/// Information about a subtask for display
#[derive(Debug, Clone)]
pub struct SubTaskInfo {
    pub id: String,
    pub name: String,
    pub status: SubTaskStatus,
    pub stage: usize,
    pub dependencies: Vec<String>,
    pub agent_name: Option<String>,
    pub current_tool: Option<String>,
    pub steps: usize,
    pub max_steps: usize,
    /// Tool call history for this sub-agent
    pub tool_call_history: Vec<AgentToolCallDetail>,
    /// Conversation messages for this sub-agent
    pub messages: Vec<AgentMessageEntry>,
    /// Final output text
    pub output: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
}

/// State for the swarm view
#[derive(Debug)]
pub struct SwarmViewState {
    /// Whether swarm mode is active
    pub active: bool,
    /// The main task being executed
    pub task: String,
    /// All subtasks
    pub subtasks: Vec<SubTaskInfo>,
    /// Current stage (0-indexed)
    pub current_stage: usize,
    /// Total stages
    pub total_stages: usize,
    /// Stats from execution
    pub stats: Option<SwarmStats>,
    /// Any error message
    pub error: Option<String>,
    /// Whether execution is complete
    pub complete: bool,
    /// Currently selected subtask index
    pub selected_index: usize,
    /// Whether we're in detail mode (viewing a single agent)
    pub detail_mode: bool,
    /// Scroll offset within the detail view
    pub detail_scroll: usize,
    /// ListState for StatefulWidget rendering
    pub list_state: ListState,
}

impl Default for SwarmViewState {
    fn default() -> Self {
        Self {
            active: false,
            task: String::new(),
            subtasks: Vec::new(),
            current_stage: 0,
            total_stages: 0,
            stats: None,
            error: None,
            complete: false,
            selected_index: 0,
            detail_mode: false,
            detail_scroll: 0,
            list_state: ListState::default(),
        }
    }
}

impl SwarmViewState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Handle a swarm event
    pub fn handle_event(&mut self, event: SwarmEvent) {
        match event {
            SwarmEvent::Started {
                task,
                total_subtasks,
            } => {
                self.active = true;
                self.task = task;
                self.subtasks.clear();
                self.current_stage = 0;
                self.complete = false;
                self.error = None;
                self.selected_index = 0;
                self.detail_mode = false;
                self.detail_scroll = 0;
                self.list_state = ListState::default();
                // Pre-allocate
                self.subtasks.reserve(total_subtasks);
            }
            SwarmEvent::Decomposed { subtasks } => {
                self.subtasks = subtasks;
                self.total_stages = self.subtasks.iter().map(|s| s.stage).max().unwrap_or(0) + 1;
                if !self.subtasks.is_empty() {
                    self.list_state.select(Some(0));
                }
            }
            SwarmEvent::SubTaskUpdate {
                id,
                name,
                status,
                agent_name,
            } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == id) {
                    task.status = status;
                    task.name = name;
                    if agent_name.is_some() {
                        task.agent_name = agent_name;
                    }
                }
            }
            SwarmEvent::AgentStarted {
                subtask_id,
                agent_name,
                ..
            } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == subtask_id) {
                    task.status = SubTaskStatus::Running;
                    task.agent_name = Some(agent_name);
                }
            }
            SwarmEvent::AgentToolCall {
                subtask_id,
                tool_name,
            } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == subtask_id) {
                    task.current_tool = Some(tool_name);
                    task.steps += 1;
                }
            }
            SwarmEvent::AgentToolCallDetail { subtask_id, detail } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == subtask_id) {
                    task.current_tool = Some(detail.tool_name.clone());
                    task.steps += 1;
                    task.tool_call_history.push(detail);
                }
            }
            SwarmEvent::AgentMessage { subtask_id, entry } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == subtask_id) {
                    task.messages.push(entry);
                }
            }
            SwarmEvent::AgentComplete {
                subtask_id,
                success,
                steps,
            } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == subtask_id) {
                    task.status = if success {
                        SubTaskStatus::Completed
                    } else {
                        SubTaskStatus::Failed
                    };
                    task.steps = steps;
                    task.current_tool = None;
                }
            }
            SwarmEvent::AgentOutput { subtask_id, output } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == subtask_id) {
                    task.output = Some(output);
                }
            }
            SwarmEvent::AgentError { subtask_id, error } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == subtask_id) {
                    task.error = Some(error);
                }
            }
            SwarmEvent::StageComplete { stage, .. } => {
                self.current_stage = stage + 1;
            }
            SwarmEvent::Complete { success: _, stats } => {
                self.stats = Some(stats);
                self.complete = true;
            }
            SwarmEvent::Error(err) => {
                self.error = Some(err);
            }
        }
    }

    /// Move selection up
    pub fn select_prev(&mut self) {
        if self.subtasks.is_empty() {
            return;
        }
        self.selected_index = self.selected_index.saturating_sub(1);
        self.list_state.select(Some(self.selected_index));
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        if self.subtasks.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + 1).min(self.subtasks.len() - 1);
        self.list_state.select(Some(self.selected_index));
    }

    /// Enter detail mode for the selected subtask
    pub fn enter_detail(&mut self) {
        if !self.subtasks.is_empty() {
            self.detail_mode = true;
            self.detail_scroll = 0;
        }
    }

    /// Exit detail mode, return to list
    pub fn exit_detail(&mut self) {
        self.detail_mode = false;
        self.detail_scroll = 0;
    }

    /// Scroll detail view down
    pub fn detail_scroll_down(&mut self, amount: usize) {
        self.detail_scroll = self.detail_scroll.saturating_add(amount);
    }

    /// Scroll detail view up
    pub fn detail_scroll_up(&mut self, amount: usize) {
        self.detail_scroll = self.detail_scroll.saturating_sub(amount);
    }

    /// Get the currently selected subtask
    pub fn selected_subtask(&self) -> Option<&SubTaskInfo> {
        self.subtasks.get(self.selected_index)
    }

    /// Get count of tasks by status
    pub fn status_counts(&self) -> (usize, usize, usize, usize) {
        let mut pending = 0;
        let mut running = 0;
        let mut completed = 0;
        let mut failed = 0;

        for task in &self.subtasks {
            match task.status {
                SubTaskStatus::Pending | SubTaskStatus::Blocked => pending += 1,
                SubTaskStatus::Running => running += 1,
                SubTaskStatus::Completed => completed += 1,
                SubTaskStatus::Failed | SubTaskStatus::Cancelled | SubTaskStatus::TimedOut => {
                    failed += 1
                }
            }
        }

        (pending, running, completed, failed)
    }

    /// Overall progress as percentage
    pub fn progress(&self) -> f64 {
        if self.subtasks.is_empty() {
            return 0.0;
        }
        let (_, _, completed, failed) = self.status_counts();
        ((completed + failed) as f64 / self.subtasks.len() as f64) * 100.0
    }
}

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
    } else {
        " Swarm [ACTIVE] "
    };

    let status_line = Line::from(vec![
        Span::styled("Task: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            if state.task.len() > 50 {
                format!("{}...", &state.task[..47])
            } else {
                state.task.clone()
            },
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

    let items: Vec<ListItem> = state
        .subtasks
        .iter()
        .map(|task| {
            let (icon, color) = match task.status {
                SubTaskStatus::Pending => ("○", Color::DarkGray),
                SubTaskStatus::Blocked => ("⊘", Color::Yellow),
                SubTaskStatus::Running => ("●", Color::Cyan),
                SubTaskStatus::Completed => ("✓", Color::Green),
                SubTaskStatus::Failed => ("✗", Color::Red),
                SubTaskStatus::Cancelled => ("⊗", Color::DarkGray),
                SubTaskStatus::TimedOut => ("⏱", Color::Red),
            };

            let mut spans = vec![
                Span::styled(format!("{} ", icon), Style::default().fg(color)),
                Span::styled(
                    format!("[S{}] ", task.stage),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(&task.name, Style::default().fg(Color::White)),
            ];

            // Show agent/tool info for running tasks
            if task.status == SubTaskStatus::Running {
                if let Some(ref agent) = task.agent_name {
                    spans.push(Span::styled(
                        format!(" → {}", agent),
                        Style::default().fg(Color::Cyan),
                    ));
                }
                if let Some(ref tool) = task.current_tool {
                    spans.push(Span::styled(
                        format!(" [{}]", tool),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::DIM),
                    ));
                }
            }

            // Show step count
            if task.steps > 0 {
                spans.push(Span::styled(
                    format!(" ({}/{})", task.steps, task.max_steps),
                    Style::default().fg(Color::DarkGray),
                ));
            }

            ListItem::new(Line::from(spans))
        })
        .collect();

    let title = if state.subtasks.is_empty() {
        " SubTasks (none yet) "
    } else {
        " SubTasks (↑↓:select  Enter:detail) "
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
                "system" => (Color::DarkGray, "SYS "),
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
