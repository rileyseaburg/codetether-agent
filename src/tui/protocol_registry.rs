use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::a2a::types::AgentCard;
use crate::tui::app::state::App;

/// Render the protocol registry master/detail view.
///
/// Left pane: list of AgentCards with name, URL, status.
/// Right pane: full metadata of the selected card.
pub fn render_protocol_registry(f: &mut Frame, app: &mut App, area: Rect) {
    let cards = collect_agent_cards(app);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    // ── Left pane: card list ────────────────────────────────────────────
    let list_title = format!("Agent Cards ({})", cards.len());

    let items: Vec<ListItem> = cards
        .iter()
        .map(|card| {
            let status_label = if card.capabilities.streaming {
                Span::styled("● ", Style::default().fg(Color::Green))
            } else {
                Span::styled("○ ", Style::default().fg(Color::Yellow))
            };
            let name = Span::styled(
                truncate_str(&card.name, 28),
                Style::default().add_modifier(Modifier::BOLD),
            );
            let url = Span::styled(
                truncate_str(&card.url, 40),
                Style::default().fg(Color::DarkGray),
            );
            ListItem::new(Line::from(vec![status_label, name, Span::raw("  "), url]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(list_title.as_str()),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    let mut list_state = ListState::default();
    if !cards.is_empty() {
        list_state.select(Some(app.state.protocol_selected.min(cards.len() - 1)));
    }

    f.render_stateful_widget(list, chunks[0], &mut list_state);

    // ── Right pane: detail view ─────────────────────────────────────────
    let detail_block = Block::default()
        .borders(Borders::ALL)
        .title("Agent Details");

    let inner = detail_block.inner(chunks[1]);
    f.render_widget(detail_block, chunks[1]);

    if cards.is_empty() {
        let empty = Paragraph::new(Line::from(vec![
            Span::styled("No agent cards registered.", Style::default().fg(Color::DarkGray)),
            Span::raw("\n"),
            Span::raw("Connect to an A2A server to see cards here."),
        ]))
        .wrap(Wrap { trim: false });
        f.render_widget(empty, inner);
        return;
    }

    let card = &cards[app.state.protocol_selected.min(cards.len() - 1)];
    let lines = build_detail_lines(card);

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false }).scroll((
        app.state.protocol_scroll as u16,
        0,
    ));

    f.render_widget(paragraph, inner);
}

/// Collect AgentCards from the worker bridge's registered agents.
///
/// Currently the TUI tracks agent names as `HashSet<String>` via
/// `worker_bridge_registered_agents`.  We synthesize minimal cards from
/// those names.  A future iteration can fetch full `AgentCard` structs from
/// the A2A client.
fn collect_agent_cards(app: &App) -> Vec<AgentCard> {
    let mut names: Vec<String> = app
        .state
        .worker_bridge_registered_agents
        .iter()
        .cloned()
        .collect();
    names.sort();

    names
        .into_iter()
        .map(|name| AgentCard {
            name,
            description: String::new(),
            url: app
                .state
                .worker_id
                .clone()
                .unwrap_or_else(|| "unknown".into()),
            version: "0.3.0".into(),
            protocol_version: "0.3.0".into(),
            preferred_transport: None,
            additional_interfaces: vec![],
            capabilities: Default::default(),
            skills: vec![],
            default_input_modes: vec![],
            default_output_modes: vec![],
            provider: None,
            icon_url: None,
            documentation_url: None,
            security_schemes: Default::default(),
            security: vec![],
            supports_authenticated_extended_card: false,
            signatures: vec![],
        })
        .collect()
}

fn build_detail_lines(card: &AgentCard) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    lines.push(Line::from(vec![
        Span::styled("Name", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::raw(card.name.clone()),
    ]));
    lines.push(Line::from(""));

    lines.push(Line::from(vec![
        Span::styled("URL", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("   "),
        Span::raw(card.url.clone()),
    ]));
    lines.push(Line::from(""));

    lines.push(Line::from(vec![
        Span::styled("Version", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("   "),
        Span::raw(card.version.clone()),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Protocol", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" "),
        Span::raw(card.protocol_version.clone()),
    ]));

    if let Some(ref transport) = card.preferred_transport {
        lines.push(Line::from(vec![
            Span::styled("Transport", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::raw(transport.clone()),
        ]));
    }
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(
        "─ Capabilities ─",
        Style::default().fg(Color::Cyan),
    )));
    lines.push(Line::from(vec![
        Span::raw("  Streaming:  "),
        Span::styled(
            if card.capabilities.streaming {
                "yes"
            } else {
                "no"
            },
            if card.capabilities.streaming {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            },
        ),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  Push Notif: "),
        Span::styled(
            if card.capabilities.push_notifications {
                "yes"
            } else {
                "no"
            },
            if card.capabilities.push_notifications {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            },
        ),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  Transitions:"),
        Span::styled(
            if card.capabilities.state_transition_history {
                "yes"
            } else {
                "no"
            },
            if card.capabilities.state_transition_history {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            },
        ),
    ]));
    if !card.capabilities.extensions.is_empty() {
        lines.push(Line::from(vec![
            Span::raw("  Extensions: "),
            Span::raw(
                card.capabilities
                    .extensions
                    .iter()
                    .map(|e| e.uri.clone())
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
        ]));
    }

    if !card.skills.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "─ Skills ─",
            Style::default().fg(Color::Cyan),
        )));
        for skill in &card.skills {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("  {} ", skill.name),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::styled(&skill.id, Style::default().fg(Color::DarkGray)),
            ]));
            if !skill.description.is_empty() {
                lines.push(Line::from(format!("    {}", skill.description)));
            }
            if !skill.tags.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!("    tags: {}", skill.tags.join(", ")),
                    Style::default().fg(Color::DarkGray),
                )));
            }
        }
    }

    if !card.default_input_modes.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Input Modes", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::raw(card.default_input_modes.join(", ")),
        ]));
    }

    if !card.default_output_modes.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("Output Modes", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::raw(card.default_output_modes.join(", ")),
        ]));
    }

    if let Some(ref provider) = card.provider {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Provider", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("    "),
            Span::raw(&provider.organization),
        ]));
        lines.push(Line::from(vec![
            Span::raw("             "),
            Span::raw(&provider.url),
        ]));
    }

    if let Some(ref doc_url) = card.documentation_url {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Docs", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("      "),
            Span::styled(doc_url.clone(), Style::default().fg(Color::Blue)),
        ]));
    }

    lines
}

fn truncate_str(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}…", &s[..max.saturating_sub(1)])
    } else {
        s.to_string()
    }
}
