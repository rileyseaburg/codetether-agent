//! Swarm mode view for the TUI
//!
//! Displays real-time status of parallel sub-agent execution.

use crate::swarm::{SubTaskStatus, SwarmStats};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

/// Events emitted by swarm execution for TUI updates
#[derive(Debug, Clone)]
pub enum SwarmEvent {
    /// Swarm execution started
    Started {
        task: String,
        total_subtasks: usize,
    },
    /// Task decomposition complete
    Decomposed {
        subtasks: Vec<SubTaskInfo>,
    },
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
    /// SubAgent made a tool call
    AgentToolCall {
        subtask_id: String,
        tool_name: String,
    },
    /// SubAgent completed
    AgentComplete {
        subtask_id: String,
        success: bool,
        steps: usize,
    },
    /// Stage completed
    StageComplete {
        stage: usize,
        completed: usize,
        failed: usize,
    },
    /// Swarm execution complete
    Complete {
        success: bool,
        stats: SwarmStats,
    },
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
}

/// State for the swarm view
#[derive(Debug, Default)]
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
    /// Scroll position in subtask list
    pub scroll: usize,
    /// Whether execution is complete
    pub complete: bool,
}

impl SwarmViewState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Handle a swarm event
    pub fn handle_event(&mut self, event: SwarmEvent) {
        match event {
            SwarmEvent::Started { task, total_subtasks } => {
                self.active = true;
                self.task = task;
                self.subtasks.clear();
                self.current_stage = 0;
                self.complete = false;
                self.error = None;
                // Pre-allocate
                self.subtasks.reserve(total_subtasks);
            }
            SwarmEvent::Decomposed { subtasks } => {
                self.subtasks = subtasks;
                self.total_stages = self.subtasks.iter().map(|s| s.stage).max().unwrap_or(0) + 1;
            }
            SwarmEvent::SubTaskUpdate { id, name, status, agent_name } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == id) {
                    task.status = status;
                    task.name = name;
                    if agent_name.is_some() {
                        task.agent_name = agent_name;
                    }
                }
            }
            SwarmEvent::AgentStarted { subtask_id, agent_name, .. } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == subtask_id) {
                    task.status = SubTaskStatus::Running;
                    task.agent_name = Some(agent_name);
                }
            }
            SwarmEvent::AgentToolCall { subtask_id, tool_name } => {
                if let Some(task) = self.subtasks.iter_mut().find(|t| t.id == subtask_id) {
                    task.current_tool = Some(tool_name);
                    task.steps += 1;
                }
            }
            SwarmEvent::AgentComplete { subtask_id, success, steps } => {
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

/// Render the swarm view
pub fn render_swarm_view(f: &mut Frame, state: &SwarmViewState, area: Rect) {
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

    // Subtask list
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
        Span::styled(
            format!("/{}", total),
            Style::default().fg(Color::DarkGray),
        ),
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

fn render_subtask_list(f: &mut Frame, state: &SwarmViewState, area: Rect) {
    use ratatui::widgets::ListState;
    
    let items: Vec<ListItem> = state
        .subtasks
        .iter()
        .enumerate()
        .map(|(i, task)| {
            let is_selected = i == state.scroll;
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
                Span::styled(
                    if is_selected { "▶ " } else { "  " },
                    Style::default().fg(Color::Cyan),
                ),
                Span::styled(format!("{} ", icon), Style::default().fg(color)),
                Span::styled(
                    format!("[S{}] ", task.stage),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    &task.name,
                    if is_selected {
                        Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    },
                ),
            ];

            // Show agent/tool info for running tasks or selected task
            if task.status == SubTaskStatus::Running || is_selected {
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

    let help_text = if state.subtasks.is_empty() {
        " SubTasks (none yet) "
    } else {
        " SubTasks (↑↓/jk to navigate) "
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(help_text),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    // Create list state for proper scrolling
    let mut list_state = ListState::default();
    list_state.select(Some(state.scroll));

    f.render_stateful_widget(list, area, &mut list_state);
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
