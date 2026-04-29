//! Protocol bus log view for the TUI
//!
//! Displays a real-time, scrollable log of every `BusEnvelope` that
//! travels over the `AgentBus`, giving full visibility into how
//! sub-agents, the gRPC layer, and workers communicate.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::bus::{BusEnvelope, BusMessage};

#[derive(Debug, Clone)]
pub struct ProtocolSummary {
    pub cwd_display: String,
    pub worker_id: Option<String>,
    pub worker_name: Option<String>,
    pub a2a_connected: bool,
    pub processing: Option<bool>,
    pub registered_agents: Vec<String>,
    pub queued_tasks: usize,
    pub recent_task: Option<String>,
    pub peer_endpoint_ready: bool,
}

#[derive(Debug, Clone)]
pub struct BusLogEntry {
    pub timestamp: String,
    pub topic: String,
    pub sender_id: String,
    pub kind: String,
    pub summary: String,
    pub detail: String,
    pub kind_color: Color,
}

impl BusLogEntry {
    pub fn from_envelope(env: &BusEnvelope) -> Self {
        let timestamp = env.timestamp.format("%H:%M:%S%.3f").to_string();
        let topic = env.topic.clone();
        let sender_id = env.sender_id.clone();

        let (kind, summary, detail, kind_color) = match &env.message {
            BusMessage::AgentReady {
                agent_id,
                capabilities,
            } => (
                "READY".to_string(),
                format!("{agent_id} online ({} caps)", capabilities.len()),
                format!(
                    "Agent: {agent_id}\nCapabilities: {}",
                    capabilities.join(", ")
                ),
                Color::Green,
            ),
            BusMessage::AgentShutdown { agent_id } => (
                "SHUTDOWN".to_string(),
                format!("{agent_id} shutting down"),
                format!("Agent: {agent_id}"),
                Color::Red,
            ),
            BusMessage::AgentMessage { from, to, parts } => {
                let text_preview: String = parts
                    .iter()
                    .filter_map(|p| match p {
                        crate::a2a::types::Part::Text { text } => Some(text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                let preview = truncate(&text_preview, 80);
                let a2a = from == "remote-a2a" || to == "remote-a2a";
                let kind = if a2a { "A2A•MSG" } else { "MSG" };
                let detail = format!(
                    "From: {from}\nTo: {to}\nTransport: {}\nParts ({}):\n{text_preview}",
                    if a2a { "A2A/mDNS peer" } else { "local bus" },
                    parts.len()
                );
                (
                    kind.to_string(),
                    format!("{from} → {to}: {preview}"),
                    detail,
                    Color::Cyan,
                )
            }
            BusMessage::TaskUpdate {
                task_id,
                state,
                message,
            } => {
                let msg = message.as_deref().unwrap_or("");
                (
                    "TASK".to_string(),
                    format!("{task_id} → {state:?} {}", truncate(msg, 50)),
                    format!("Task: {task_id}\nState: {state:?}\nMessage: {msg}"),
                    Color::Yellow,
                )
            }
            BusMessage::ArtifactUpdate { task_id, artifact } => (
                "ARTIFACT".to_string(),
                format!("task={task_id} parts={}", artifact.parts.len()),
                format!(
                    "Task: {task_id}\nArtifact: {}\nParts: {}",
                    artifact.name.as_deref().unwrap_or("(unnamed)"),
                    artifact.parts.len()
                ),
                Color::Magenta,
            ),
            BusMessage::SharedResult { key, tags, .. } => (
                "RESULT".to_string(),
                format!("key={key} tags=[{}]", tags.join(",")),
                format!("Key: {key}\nTags: {}", tags.join(", ")),
                Color::Blue,
            ),
            BusMessage::ToolRequest {
                request_id,
                agent_id,
                tool_name,
                arguments,
                step,
            } => {
                let args_str = serde_json::to_string(arguments).unwrap_or_default();
                (
                    "TOOL→".to_string(),
                    format!("{agent_id} call {tool_name}"),
                    format!(
                        "Request: {request_id}\nAgent: {agent_id}\nStep: {step}\nTool: {tool_name}\nArgs: {}",
                        truncate(&args_str, 200)
                    ),
                    Color::Yellow,
                )
            }
            BusMessage::ToolResponse {
                request_id,
                agent_id,
                tool_name,
                result,
                success,
                step,
            } => {
                let icon = if *success { "✓" } else { "✗" };
                (
                    "←TOOL".to_string(),
                    format!("{icon} {agent_id} {tool_name}"),
                    format!(
                        "Request: {request_id}\nAgent: {agent_id}\nStep: {step}\nTool: {tool_name}\nSuccess: {success}\nResult: {}",
                        truncate(result, 200)
                    ),
                    if *success { Color::Green } else { Color::Red },
                )
            }
            BusMessage::Heartbeat { agent_id, status } => {
                let is_a2a = status.starts_with("discovered via A2A");
                (
                    if is_a2a { "A2A•PEER" } else { "BEAT" }.to_string(),
                    format!("{agent_id} [{status}]"),
                    format!("Agent: {agent_id}\nStatus: {status}"),
                    if is_a2a {
                        Color::LightCyan
                    } else {
                        Color::DarkGray
                    },
                )
            }
            BusMessage::RalphLearning {
                prd_id,
                story_id,
                iteration,
                learnings,
                ..
            } => (
                "LEARN".to_string(),
                format!("{story_id} iter {iteration} ({} items)", learnings.len()),
                format!(
                    "PRD: {prd_id}\nStory: {story_id}\nIteration: {iteration}\nLearnings:\n{}",
                    learnings.join("\n")
                ),
                Color::Cyan,
            ),
            BusMessage::RalphHandoff {
                prd_id,
                from_story,
                to_story,
                progress_summary,
                ..
            } => (
                "HANDOFF".to_string(),
                format!("{from_story} → {to_story}"),
                format!(
                    "PRD: {prd_id}\nFrom: {from_story}\nTo: {to_story}\nSummary: {progress_summary}"
                ),
                Color::Blue,
            ),
            BusMessage::RalphProgress {
                prd_id,
                passed,
                total,
                iteration,
                status,
            } => (
                "PRD".to_string(),
                format!("{passed}/{total} stories (iter {iteration}) [{status}]"),
                format!(
                    "PRD: {prd_id}\nPassed: {passed}/{total}\nIteration: {iteration}\nStatus: {status}"
                ),
                Color::Yellow,
            ),
            BusMessage::ToolOutputFull {
                agent_id,
                tool_name,
                output,
                success,
                step,
            } => {
                let icon = if *success { "✓" } else { "✗" };
                let preview = truncate(output, 120);
                (
                    "TOOL•FULL".to_string(),
                    format!("{icon} {agent_id} step {step} {tool_name}: {preview}"),
                    format!(
                        "Agent: {agent_id}\nTool: {tool_name}\nStep: {step}\nSuccess: {success}\n\n--- Full Output ---\n{output}"
                    ),
                    if *success { Color::Green } else { Color::Red },
                )
            }
            BusMessage::AgentThinking {
                agent_id,
                thinking,
                step,
            } => {
                let preview = truncate(thinking, 120);
                (
                    "THINK".to_string(),
                    format!("{agent_id} step {step}: {preview}"),
                    format!("Agent: {agent_id}\nStep: {step}\n\n--- Reasoning ---\n{thinking}"),
                    Color::LightMagenta,
                )
            }
            BusMessage::VoiceSessionStarted {
                room_name,
                agent_id,
                voice_id,
            } => (
                "VOICE+".to_string(),
                format!("{room_name} agent={agent_id} voice={voice_id}"),
                format!("Room: {room_name}\nAgent: {agent_id}\nVoice: {voice_id}"),
                Color::LightCyan,
            ),
            BusMessage::VoiceTranscript {
                room_name,
                text,
                role,
                is_final,
            } => {
                let fin = if *is_final { " [final]" } else { "" };
                let preview = truncate(text, 100);
                (
                    "VOICE•T".to_string(),
                    format!("{room_name} [{role}]{fin}: {preview}"),
                    format!("Room: {room_name}\nRole: {role}\nFinal: {is_final}\n\n{text}"),
                    Color::LightCyan,
                )
            }
            BusMessage::VoiceAgentStateChanged { room_name, state } => (
                "VOICE•S".to_string(),
                format!("{room_name} → {state}"),
                format!("Room: {room_name}\nState: {state}"),
                Color::LightCyan,
            ),
            BusMessage::VoiceSessionEnded { room_name, reason } => (
                "VOICE-".to_string(),
                format!("{room_name} ended: {reason}"),
                format!("Room: {room_name}\nReason: {reason}"),
                Color::DarkGray,
            ),
        };

        Self {
            timestamp,
            topic,
            sender_id,
            kind,
            summary,
            detail,
            kind_color,
        }
    }
}

#[derive(Debug)]
pub struct BusLogState {
    pub entries: Vec<BusLogEntry>,
    pub selected_index: usize,
    pub detail_mode: bool,
    pub detail_scroll: usize,
    pub filter: String,
    pub filter_input_mode: bool,
    pub auto_scroll: bool,
    pub list_state: ListState,
    pub max_entries: usize,
}

impl Default for BusLogState {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            selected_index: 0,
            detail_mode: false,
            detail_scroll: 0,
            filter: String::new(),
            filter_input_mode: false,
            auto_scroll: true,
            list_state: ListState::default(),
            max_entries: 10_000,
        }
    }
}

impl BusLogState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, entry: BusLogEntry) {
        self.entries.push(entry);
        if self.entries.len() > self.max_entries {
            let overflow = self.entries.len() - self.max_entries;
            self.entries.drain(0..overflow);
            self.selected_index = self.selected_index.saturating_sub(overflow);
        }
        if self.auto_scroll {
            self.selected_index = self.visible_count().saturating_sub(1);
        }
    }

    pub fn ingest(&mut self, env: &BusEnvelope) {
        self.push(BusLogEntry::from_envelope(env));
    }

    pub fn filtered_entries(&self) -> Vec<&BusLogEntry> {
        if self.filter.trim().is_empty() {
            self.entries.iter().collect()
        } else {
            let needle = self.filter.to_lowercase();
            self.entries
                .iter()
                .filter(|entry| {
                    entry.topic.to_lowercase().contains(&needle)
                        || entry.sender_id.to_lowercase().contains(&needle)
                        || entry.kind.to_lowercase().contains(&needle)
                        || entry.summary.to_lowercase().contains(&needle)
                })
                .collect()
        }
    }

    pub fn visible_count(&self) -> usize {
        self.filtered_entries().len()
    }

    pub fn select_prev(&mut self) {
        self.auto_scroll = false;
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn select_next(&mut self) {
        self.auto_scroll = false;
        let max_index = self.visible_count().saturating_sub(1);
        if self.selected_index < max_index {
            self.selected_index += 1;
        }
    }

    pub fn enter_detail(&mut self) {
        self.detail_mode = true;
        self.detail_scroll = 0;
    }

    pub fn exit_detail(&mut self) {
        self.detail_mode = false;
        self.detail_scroll = 0;
    }

    pub fn detail_scroll_up(&mut self, amount: usize) {
        self.detail_scroll = self.detail_scroll.saturating_sub(amount);
    }

    pub fn detail_scroll_down(&mut self, amount: usize) {
        self.detail_scroll = self.detail_scroll.saturating_add(amount);
    }

    pub fn selected_entry(&self) -> Option<&BusLogEntry> {
        self.filtered_entries().get(self.selected_index).copied()
    }

    pub fn a2a_message_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.kind == "A2A•MSG")
            .count()
    }

    pub fn enter_filter_mode(&mut self) {
        self.filter_input_mode = true;
    }

    pub fn exit_filter_mode(&mut self) {
        self.filter_input_mode = false;
    }

    pub fn clear_filter(&mut self) {
        self.filter.clear();
        self.selected_index = self.visible_count().saturating_sub(1);
    }

    pub fn push_filter_char(&mut self, c: char) {
        self.filter.push(c);
        self.selected_index = 0;
    }

    pub fn pop_filter_char(&mut self) {
        self.filter.pop();
        self.selected_index = 0;
    }
}

pub fn render_bus_log(f: &mut Frame, state: &mut BusLogState, area: Rect) {
    render_bus_log_with_summary(f, state, area, None);
}

pub fn render_bus_log_with_summary(
    f: &mut Frame,
    state: &mut BusLogState,
    area: Rect,
    summary: Option<ProtocolSummary>,
) {
    if let Some(summary) = summary {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),
                Constraint::Min(8),
                Constraint::Length(2),
            ])
            .split(area);

        let worker_label = if summary.a2a_connected {
            "connected"
        } else {
            "offline"
        };
        let worker_color = if summary.a2a_connected {
            Color::Green
        } else {
            Color::Red
        };
        let processing_label = match summary.processing {
            Some(true) => "processing",
            Some(false) => "idle",
            None => "unknown",
        };
        let processing_color = match summary.processing {
            Some(true) => Color::Yellow,
            Some(false) => Color::Green,
            None => Color::DarkGray,
        };
        let worker_id = summary.worker_id.as_deref().unwrap_or("n/a");
        let worker_name = summary.worker_name.as_deref().unwrap_or("n/a");
        let peer_label = if summary.peer_endpoint_ready {
            "ready"
        } else {
            "off"
        };
        let peer_color = if summary.peer_endpoint_ready {
            Color::Cyan
        } else {
            Color::DarkGray
        };
        let a2a_count = state.a2a_message_count();
        let recent_task = summary
            .recent_task
            .unwrap_or_else(|| "No recent A2A tasks".to_string());
        let registered_agents = if summary.registered_agents.is_empty() {
            "none".to_string()
        } else {
            truncate(&summary.registered_agents.join(", "), 120)
        };
        let panel = Paragraph::new(vec![
            Line::from(vec![
                "A2A worker: ".dim(),
                Span::styled(worker_label, Style::default().fg(worker_color).bold()),
                "  •  ".dim(),
                Span::raw(worker_name).cyan(),
                "  •  ".dim(),
                Span::raw(worker_id).dim(),
            ]),
            Line::from(vec![
                "A2A peer: ".dim(),
                Span::styled(peer_label, Style::default().fg(peer_color).bold()),
                "  •  ".dim(),
                Span::raw(format!("{a2a_count} visible A2A msg(s)")),
            ]),
            Line::from(vec![
                "Heartbeat: ".dim(),
                Span::styled(
                    processing_label,
                    Style::default().fg(processing_color).bold(),
                ),
                "  •  ".dim(),
                Span::raw(format!("{} queued task(s)", summary.queued_tasks)),
            ]),
            Line::from(vec!["Agents: ".dim(), Span::raw(registered_agents)]),
            Line::from(vec!["Workspace: ".dim(), Span::raw(summary.cwd_display)]),
            Line::from(vec![
                "Recent task: ".dim(),
                Span::raw(truncate(&recent_task, 120)),
            ]),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Protocol Summary"),
        )
        .wrap(Wrap { trim: true });
        f.render_widget(panel, chunks[0]);
        render_bus_body(f, state, chunks[1], chunks[2]);
    } else {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(8), Constraint::Length(2)])
            .split(area);
        render_bus_body(f, state, chunks[0], chunks[1]);
    }
}

fn render_bus_body(f: &mut Frame, state: &mut BusLogState, main_area: Rect, footer_area: Rect) {
    if state.detail_mode {
        let detail = state
            .selected_entry()
            .map(|entry| entry.detail.clone())
            .unwrap_or_else(|| "No entry selected".to_string());
        let widget = Paragraph::new(detail)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Protocol Detail"),
            )
            .wrap(Wrap { trim: false })
            .scroll((state.detail_scroll.min(u16::MAX as usize) as u16, 0));
        f.render_widget(widget, main_area);
    } else {
        let filtered = state.filtered_entries();
        let filtered_len = filtered.len();
        let filter_title = if state.filter.is_empty() {
            format!("Protocol Bus Log ({filtered_len})")
        } else if state.filter_input_mode {
            format!("Protocol Bus Log [{}_] ({filtered_len})", state.filter)
        } else {
            format!("Protocol Bus Log [{}] ({filtered_len})", state.filter)
        };
        let items: Vec<ListItem<'_>> = filtered
            .iter()
            .enumerate()
            .map(|(idx, entry)| {
                let prefix = if idx == state.selected_index {
                    "▶ "
                } else {
                    "  "
                };
                ListItem::new(Line::from(vec![
                    Span::raw(prefix),
                    Span::styled(
                        format!("[{}] ", entry.kind),
                        Style::default()
                            .fg(entry.kind_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(format!(
                        "{} {} {}",
                        entry.timestamp, entry.sender_id, entry.summary
                    )),
                ]))
            })
            .collect();
        drop(filtered);
        state.list_state.select(Some(
            state.selected_index.min(filtered_len.saturating_sub(1)),
        ));
        let list =
            List::new(items).block(Block::default().borders(Borders::ALL).title(filter_title));
        f.render_stateful_widget(list, main_area, &mut state.list_state);
    }

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("↑↓", Style::default().fg(Color::Yellow)),
        Span::raw(": nav  "),
        Span::styled("Enter", Style::default().fg(Color::Yellow)),
        Span::raw(": detail/apply  "),
        Span::styled("/", Style::default().fg(Color::Yellow)),
        Span::raw(": filter  "),
        Span::styled("Backspace", Style::default().fg(Color::Yellow)),
        Span::raw(": edit  "),
        Span::styled("c", Style::default().fg(Color::Yellow)),
        Span::raw(": clear  "),
        Span::styled("Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": back/close filter"),
    ]));
    f.render_widget(footer, footer_area);
}

fn truncate(value: &str, max_chars: usize) -> String {
    let mut chars = value.chars();
    let truncated: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{truncated}…")
    } else {
        truncated
    }
}

#[cfg(test)]
mod tests {
    use super::{BusLogEntry, BusLogState};
    use ratatui::style::Color;

    fn entry(summary: &str, topic: &str) -> BusLogEntry {
        BusLogEntry {
            timestamp: "00:00:00.000".to_string(),
            topic: topic.to_string(),
            sender_id: "tester".to_string(),
            kind: "MSG".to_string(),
            summary: summary.to_string(),
            detail: summary.to_string(),
            kind_color: Color::Cyan,
        }
    }

    #[test]
    fn bus_filter_mode_can_be_entered_and_exited() {
        let mut state = BusLogState::new();
        assert!(!state.filter_input_mode);
        state.enter_filter_mode();
        assert!(state.filter_input_mode);
        state.exit_filter_mode();
        assert!(!state.filter_input_mode);
    }

    #[test]
    fn bus_filter_chars_update_visible_entries() {
        let mut state = BusLogState::new();
        state.push(entry("alpha event", "protocol.alpha"));
        state.push(entry("beta event", "protocol.beta"));

        assert_eq!(state.visible_count(), 2);
        state.push_filter_char('b');
        state.push_filter_char('e');

        assert_eq!(state.filter, "be");
        assert_eq!(state.visible_count(), 1);
        assert_eq!(
            state.selected_entry().map(|e| e.summary.as_str()),
            Some("beta event")
        );
    }

    #[test]
    fn bus_filter_backspace_and_clear_restore_entries() {
        let mut state = BusLogState::new();
        state.push(entry("alpha event", "protocol.alpha"));
        state.push(entry("beta event", "protocol.beta"));

        state.push_filter_char('a');
        state.push_filter_char('l');
        assert_eq!(state.visible_count(), 1);

        state.pop_filter_char();
        assert_eq!(state.filter, "a");
        assert_eq!(state.visible_count(), 2);

        state.clear_filter();
        assert!(state.filter.is_empty());
        assert_eq!(state.visible_count(), 2);
    }

    #[test]
    fn bus_detail_and_filter_modes_can_coexist_but_are_independently_cleared() {
        let mut state = BusLogState::new();
        state.push(entry("alpha event", "protocol.alpha"));

        state.enter_filter_mode();
        state.enter_detail();
        assert!(state.filter_input_mode);
        assert!(state.detail_mode);

        state.exit_filter_mode();
        assert!(!state.filter_input_mode);
        assert!(state.detail_mode);

        state.exit_detail();
        assert!(!state.detail_mode);
    }

    #[test]
    fn bus_filter_editing_resets_selection_to_first_filtered_match() {
        let mut state = BusLogState::new();
        state.push(entry("alpha event", "protocol.alpha"));
        state.push(entry("gamma event", "protocol.gamma"));

        state.selected_index = 1;
        state.push_filter_char('m');

        assert_eq!(state.selected_index, 0);
        assert_eq!(
            state.selected_entry().map(|e| e.summary.as_str()),
            Some("alpha event")
        );
    }
}
