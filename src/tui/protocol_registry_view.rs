use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};

use crate::a2a::types::AgentCard;
use crate::tui::app::state::App;

pub fn render_protocol_registry(f: &mut Frame, app: &mut App, area: Rect) {
    let cards = collect_agent_cards(app);
    let chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Percentage(40),
            ratatui::layout::Constraint::Percentage(60),
        ])
        .split(area);
    f.render_widget(list_widget(&cards), chunks[0]);
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Agent Details");
    let inner = block.inner(chunks[1]);
    f.render_widget(block, chunks[1]);
    let body = detail_widget(app, &cards).unwrap_or_else(empty_widget);
    f.render_widget(body, inner);
}

fn collect_agent_cards(app: &App) -> Vec<AgentCard> {
    let mut names = app
        .state
        .worker_bridge_registered_agents
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    names.sort();
    names
        .into_iter()
        .map(|name| agent_card(app, name))
        .collect()
}

fn agent_card(app: &App, name: String) -> AgentCard {
    AgentCard {
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
    }
}

fn list_widget(cards: &[AgentCard]) -> List<'static> {
    let items = cards.iter().map(list_item).collect::<Vec<_>>();
    List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(list_title(cards)),
    )
}

fn list_title(cards: &[AgentCard]) -> String {
    format!("Agent Cards ({})", cards.len())
}

fn list_item(card: &AgentCard) -> ListItem<'static> {
    let dot = if card.capabilities.streaming {
        "● "
    } else {
        "○ "
    };
    let color = if card.capabilities.streaming {
        Color::Green
    } else {
        Color::Yellow
    };
    let line = Line::from(vec![
        Span::styled(dot, Style::default().fg(color)),
        Span::styled(
            card.name.clone(),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(card.url.clone(), Style::default().fg(Color::DarkGray)),
    ]);
    ListItem::new(line)
}

fn detail_widget(app: &App, cards: &[AgentCard]) -> Option<Paragraph<'static>> {
    let idx = app
        .state
        .protocol_selected
        .min(cards.len().saturating_sub(1));
    let card = cards.get(idx)?;
    Some(
        Paragraph::new(detail_lines(card))
            .wrap(Wrap { trim: false })
            .scroll((app.state.protocol_scroll as u16, 0)),
    )
}

fn detail_lines(card: &AgentCard) -> Vec<Line<'static>> {
    vec![
        field("Name", card.name.clone()),
        Line::from(""),
        field("URL", card.url.clone()),
        Line::from(""),
        field("Version", card.version.clone()),
        field("Protocol", card.protocol_version.clone()),
        Line::from(""),
        Line::from(Span::styled(
            "─ Capabilities ─",
            Style::default().fg(Color::Cyan),
        )),
        field("Streaming", yes_no(card.capabilities.streaming)),
        field("Push Notif", yes_no(card.capabilities.push_notifications)),
        field(
            "Transitions",
            yes_no(card.capabilities.state_transition_history),
        ),
    ]
}

fn field(label: &str, value: String) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            label.to_string(),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::raw(value),
    ])
}

fn yes_no(value: bool) -> String {
    if value { "yes" } else { "no" }.to_string()
}

fn empty_widget<'a>() -> Paragraph<'a> {
    Paragraph::new("No agent cards registered.\nConnect to an A2A server to see cards here.")
        .wrap(Wrap { trim: false })
}
