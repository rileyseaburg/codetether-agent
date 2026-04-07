//! Webview layout mode for TUI chat
//!
//! Alternative layout with header, sidebar, inspector panel.
//! Toggle between Classic and Webview layouts at runtime.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatLayoutMode {
    Classic,
    Webview,
}


fn render_webview_chat(f: &mut Frame, app: &mut App, theme: &Theme) -> bool {
    let area = f.area();
    if area.width < 90 || area.height < 18 {
        return false;
    }

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(1),    // Body
            Constraint::Length(3), // Input
            Constraint::Length(1), // Status
        ])
        .split(area);

    render_webview_header(f, app, theme, main_chunks[0]);

    let body_constraints = if app.show_inspector {
        vec![
            Constraint::Length(26),
            Constraint::Min(40),
            Constraint::Length(30),
        ]
    } else {
        vec![Constraint::Length(26), Constraint::Min(40)]
    };

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(body_constraints)
        .split(main_chunks[1]);

    render_webview_sidebar(f, app, theme, body_chunks[0]);
    // Build lines using cache before passing to the immutable render fn
    let center_area = body_chunks[1];
    let center_max_width = center_area.width.saturating_sub(4) as usize;
    let center_lines = app
        .get_or_build_message_lines(theme, center_max_width)
        .to_vec();
    let center_visible_lines = center_area.height.saturating_sub(2) as usize;
    app.last_max_scroll = center_lines.len().saturating_sub(center_visible_lines);
    render_webview_chat_center(f, app, theme, center_area, &center_lines);
    if app.show_inspector && body_chunks.len() > 2 {
        render_webview_inspector(f, app, theme, body_chunks[2]);
    }

    render_webview_input(f, app, theme, main_chunks[2]);

    let token_display = TokenDisplay::new();
    let mut status_line = token_display.create_status_bar(theme);
    let model_status = if let Some(ref active) = app.active_model {
        let (provider, model) = crate::provider::parse_model_string(active);
        format!(" {}:{} ", provider.unwrap_or("auto"), model)
    } else {
        " auto ".to_string()
    };
    status_line.spans.insert(
        0,
        Span::styled(
            "│ ",
            Style::default()
                .fg(theme.timestamp_color.to_color())
                .add_modifier(Modifier::DIM),
        ),
    );
    status_line.spans.insert(
        0,
        Span::styled(model_status, Style::default().fg(Color::Cyan)),
    );
    status_line
        .spans
        .insert(0, Span::styled("│ ", Style::default().fg(Color::DarkGray)));
    status_line.spans.insert(0, app.bus_status_badge_span());
    if let Some(autochat_status) = app.autochat_status_label() {
        status_line.spans.insert(
            0,
            Span::styled(
                format!(" {autochat_status} "),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        );
    }
    let status = Paragraph::new(status_line);
    f.render_widget(status, main_chunks[3]);

    true
}

fn render_protocol_registry(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let cards = app.protocol_cards();
    let selected = app.protocol_selected.min(cards.len().saturating_sub(1));

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(34), Constraint::Min(30)])
        .split(area);

    let list_block = Block::default()
        .borders(Borders::ALL)
        .title(" Registered Agents ")
        .border_style(Style::default().fg(theme.border_color.to_color()));

    let mut list_lines: Vec<Line> = Vec::new();
    if cards.is_empty() {
        list_lines.push(Line::styled(
            "No protocol-registered agents.",
            Style::default().fg(Color::DarkGray),
        ));
        list_lines.push(Line::styled(
            "Spawn an agent with /spawn.",
            Style::default().fg(Color::DarkGray),
        ));
    } else {
        for (idx, card) in cards.iter().enumerate() {
            let marker = if idx == selected { "▶" } else { " " };
            let style = if idx == selected {
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let transport = card.preferred_transport.as_deref().unwrap_or("JSONRPC");
            list_lines.push(Line::styled(format!(" {marker} {}", card.name), style));
            list_lines.push(Line::styled(
                format!(
                    "    {transport} • {}",
                    truncate_with_ellipsis(&card.url, 22)
                ),
                Style::default().fg(Color::DarkGray),
            ));
        }
    }

    let list = Paragraph::new(list_lines)
        .block(list_block)
        .wrap(Wrap { trim: false });
    f.render_widget(list, chunks[0]);

    let detail_block = Block::default()
        .borders(Borders::ALL)
        .title(" Agent Card Detail ")
        .border_style(Style::default().fg(theme.border_color.to_color()));

    let mut detail_lines: Vec<Line> = Vec::new();
    if let Some(card) = cards.get(selected) {
        let label_style = Style::default().fg(Color::DarkGray);
        detail_lines.push(Line::from(vec![
            Span::styled("Name: ", label_style),
            Span::styled(
                card.name.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]));
        detail_lines.push(Line::from(vec![
            Span::styled("Description: ", label_style),
            Span::raw(card.description.clone()),
        ]));
        detail_lines.push(Line::from(vec![
            Span::styled("URL: ", label_style),
            Span::styled(card.url.clone(), Style::default().fg(Color::Cyan)),
        ]));
        detail_lines.push(Line::from(vec![
            Span::styled("Version: ", label_style),
            Span::raw(format!(
                "{} (protocol {})",
                card.version, card.protocol_version
            )),
        ]));

        let preferred_transport = card.preferred_transport.as_deref().unwrap_or("JSONRPC");
        detail_lines.push(Line::from(vec![
            Span::styled("Transport: ", label_style),
            Span::raw(preferred_transport.to_string()),
        ]));
        if !card.additional_interfaces.is_empty() {
            detail_lines.push(Line::from(vec![
                Span::styled("Interfaces: ", label_style),
                Span::raw(format!("{} additional", card.additional_interfaces.len())),
            ]));
            for iface in &card.additional_interfaces {
                detail_lines.push(Line::styled(
                    format!("  • {} -> {}", iface.transport, iface.url),
                    Style::default().fg(Color::DarkGray),
                ));
            }
        }

        detail_lines.push(Line::from(""));
        detail_lines.push(Line::styled(
            "Capabilities",
            Style::default().add_modifier(Modifier::BOLD),
        ));
        detail_lines.push(Line::styled(
            format!(
                "  streaming={} push_notifications={} state_history={}",
                card.capabilities.streaming,
                card.capabilities.push_notifications,
                card.capabilities.state_transition_history
            ),
            Style::default().fg(Color::DarkGray),
        ));
        if !card.capabilities.extensions.is_empty() {
            detail_lines.push(Line::styled(
                format!(
                    "  extensions: {}",
                    card.capabilities
                        .extensions
                        .iter()
                        .map(|e| e.uri.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                Style::default().fg(Color::DarkGray),
            ));
        }

        detail_lines.push(Line::from(""));
        detail_lines.push(Line::styled(
            format!("Skills ({})", card.skills.len()),
            Style::default().add_modifier(Modifier::BOLD),
        ));
        if card.skills.is_empty() {
            detail_lines.push(Line::styled("  none", Style::default().fg(Color::DarkGray)));
        } else {
            for skill in &card.skills {
                let tags = if skill.tags.is_empty() {
                    "".to_string()
                } else {
                    format!(" [{}]", skill.tags.join(","))
                };
                detail_lines.push(Line::styled(
                    format!("  • {}{}", skill.name, tags),
                    Style::default().fg(Color::Green),
                ));
                if !skill.description.is_empty() {
                    detail_lines.push(Line::styled(
                        format!("    {}", skill.description),
                        Style::default().fg(Color::DarkGray),
                    ));
                }
            }
        }

        detail_lines.push(Line::from(""));
        detail_lines.push(Line::styled(
            "Security",
            Style::default().add_modifier(Modifier::BOLD),
        ));
        if card.security_schemes.is_empty() {
            detail_lines.push(Line::styled(
                "  schemes: none",
                Style::default().fg(Color::DarkGray),
            ));
        } else {
            let mut names = card.security_schemes.keys().cloned().collect::<Vec<_>>();
            names.sort();
            detail_lines.push(Line::styled(
                format!("  schemes: {}", names.join(", ")),
                Style::default().fg(Color::DarkGray),
            ));
        }
        detail_lines.push(Line::styled(
            format!("  requirements: {}", card.security.len()),
            Style::default().fg(Color::DarkGray),
        ));
        detail_lines.push(Line::styled(
            format!(
                "  authenticated_extended_card: {}",
                card.supports_authenticated_extended_card
            ),
            Style::default().fg(Color::DarkGray),
        ));
    } else {
        detail_lines.push(Line::styled(
            "No card selected.",
            Style::default().fg(Color::DarkGray),
        ));
    }

    let detail = Paragraph::new(detail_lines)
        .block(detail_block)
        .wrap(Wrap { trim: false })
        .scroll((app.protocol_scroll as u16, 0));
    f.render_widget(detail, chunks[1]);
}

fn render_webview_header(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let session_title = app
        .session
        .as_ref()
        .and_then(|s| s.title.clone())
        .unwrap_or_else(|| "Workspace Chat".to_string());
    let session_id = app
        .session
        .as_ref()
        .map(|s| s.id.chars().take(8).collect::<String>())
        .unwrap_or_else(|| "new".to_string());
    let model_label = app
        .session
        .as_ref()
        .and_then(|s| s.metadata.model.clone())
        .unwrap_or_else(|| "auto".to_string());
    let workspace_label = app.workspace.root_display.clone();
    let branch_label = app
        .workspace
        .git_branch
        .clone()
        .unwrap_or_else(|| "no-git".to_string());
    let dirty_label = if app.workspace.git_dirty_files > 0 {
        format!("{} dirty", app.workspace.git_dirty_files)
    } else {
        "clean".to_string()
    };
    let (bus_label, bus_color) = app.bus_status_label_and_color();

    let header_block = Block::default()
        .borders(Borders::ALL)
        .title(" CodeTether Webview ")
        .border_style(Style::default().fg(theme.border_color.to_color()));

    let header_lines = vec![
        Line::from(vec![
            Span::styled(session_title, Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled(
                format!("#{}", session_id),
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Workspace ",
                Style::default().fg(theme.timestamp_color.to_color()),
            ),
            Span::styled(workspace_label, Style::default()),
            Span::raw("  "),
            Span::styled(
                "Branch ",
                Style::default().fg(theme.timestamp_color.to_color()),
            ),
            Span::styled(
                branch_label,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                dirty_label,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                "Model ",
                Style::default().fg(theme.timestamp_color.to_color()),
            ),
            Span::styled(model_label, Style::default().fg(Color::Green)),
            Span::raw("  "),
            Span::styled(
                bus_label,
                Style::default().fg(bus_color).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let header = Paragraph::new(header_lines)
        .block(header_block)
        .wrap(Wrap { trim: true });
    f.render_widget(header, area);
}

fn render_webview_sidebar(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let sidebar_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Min(6)])
        .split(area);

    let workspace_block = Block::default()
        .borders(Borders::ALL)
        .title(" Workspace ")
        .border_style(Style::default().fg(theme.border_color.to_color()));

    let mut workspace_lines = Vec::new();
    workspace_lines.push(Line::from(vec![
        Span::styled(
            "Updated ",
            Style::default().fg(theme.timestamp_color.to_color()),
        ),
        Span::styled(
            app.workspace.captured_at.clone(),
            Style::default().fg(theme.timestamp_color.to_color()),
        ),
    ]));
    workspace_lines.push(Line::from(""));

    if app.workspace.entries.is_empty() {
        workspace_lines.push(Line::styled(
            "No entries found",
            Style::default().fg(Color::DarkGray),
        ));
    } else {
        for entry in app.workspace.entries.iter().take(12) {
            let icon = match entry.kind {
                WorkspaceEntryKind::Directory => "📁",
                WorkspaceEntryKind::File => "📄",
            };
            workspace_lines.push(Line::from(vec![
                Span::styled(icon, Style::default().fg(Color::Cyan)),
                Span::raw(" "),
                Span::styled(entry.name.clone(), Style::default()),
            ]));
        }
    }

    workspace_lines.push(Line::from(""));
    workspace_lines.push(Line::styled(
        "Use /refresh to rescan",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::DIM),
    ));

    let workspace_panel = Paragraph::new(workspace_lines)
        .block(workspace_block)
        .wrap(Wrap { trim: true });
    f.render_widget(workspace_panel, sidebar_chunks[0]);

    let sessions_block = Block::default()
        .borders(Borders::ALL)
        .title(" Recent Sessions ")
        .border_style(Style::default().fg(theme.border_color.to_color()));

    let mut session_lines = Vec::new();
    if app.session_picker_list.is_empty() {
        session_lines.push(Line::styled(
            "No sessions yet",
            Style::default().fg(Color::DarkGray),
        ));
    } else {
        for session in app.session_picker_list.iter().take(6) {
            let is_active = app
                .session
                .as_ref()
                .map(|s| s.id == session.id)
                .unwrap_or(false);
            let title = session.title.as_deref().unwrap_or("(untitled)");
            let indicator = if is_active { "●" } else { "○" };
            let line_style = if is_active {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            session_lines.push(Line::from(vec![
                Span::styled(indicator, line_style),
                Span::raw(" "),
                Span::styled(title, line_style),
            ]));
            session_lines.push(Line::styled(
                format!(
                    "  {} msgs • {}",
                    session.message_count,
                    session.updated_at.format("%m-%d %H:%M")
                ),
                Style::default().fg(Color::DarkGray),
            ));
        }
    }

    let sessions_panel = Paragraph::new(session_lines)
        .block(sessions_block)
        .wrap(Wrap { trim: true });
    f.render_widget(sessions_panel, sidebar_chunks[1]);
}

fn render_webview_chat_center(
    f: &mut Frame,
    app: &App,
    theme: &Theme,
    area: Rect,
    message_lines: &[ratatui::text::Line<'static>],
) {
    let messages_area = area;
    let focused_suffix = app
        .active_spawned_agent
        .as_ref()
        .map(|name| format!(" → @{name}"))
        .unwrap_or_default();
    let messages_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Chat [{}{}] ", app.current_agent, focused_suffix))
        .border_style(Style::default().fg(theme.border_color.to_color()));

    // message_lines is pre-built by the caller via the App cache.
    let total_lines = message_lines.len();
    let visible_lines = messages_area.height.saturating_sub(2) as usize;
    let max_scroll = total_lines.saturating_sub(visible_lines);
    let scroll = if app.scroll >= SCROLL_BOTTOM {
        0
    } else {
        app.scroll.min(max_scroll)
    };

    let messages_paragraph = Paragraph::new(
        message_lines[scroll..(scroll + visible_lines.min(total_lines)).min(total_lines)].to_vec(),
    )
    .block(messages_block.clone())
    .wrap(Wrap { trim: false });

    f.render_widget(messages_paragraph, messages_area);

    if total_lines > visible_lines {
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .symbols(ratatui::symbols::scrollbar::VERTICAL)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut scrollbar_state = ScrollbarState::new(max_scroll + 1).position(scroll);

        let scrollbar_area = Rect::new(
            messages_area.right() - 1,
            messages_area.top() + 1,
            1,
            messages_area.height - 2,
        );

        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }
}

fn render_webview_inspector(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Inspector ")
        .border_style(Style::default().fg(theme.border_color.to_color()));

    let status_label = if app.is_processing {
        "Processing"
    } else if app.autochat_running {
        "Autochat"
    } else {
        "Idle"
    };
    let status_style = if app.is_processing {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else if app.autochat_running {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };
    let tool_label = app
        .current_tool
        .clone()
        .unwrap_or_else(|| "none".to_string());
    let message_count = app.messages.len();
    let session_id = app
        .session
        .as_ref()
        .map(|s| s.id.chars().take(8).collect::<String>())
        .unwrap_or_else(|| "new".to_string());
    let model_label = app
        .active_model
        .as_deref()
        .or_else(|| {
            app.session
                .as_ref()
                .and_then(|s| s.metadata.model.as_deref())
        })
        .unwrap_or("auto");
    let conversation_depth = app.session.as_ref().map(|s| s.messages.len()).unwrap_or(0);

    let label_style = Style::default().fg(theme.timestamp_color.to_color());

    let mut lines = Vec::new();
    lines.push(Line::from(vec![
        Span::styled("Status: ", label_style),
        Span::styled(status_label, status_style),
    ]));

    // Show elapsed time when processing
    if let Some(started) = app.processing_started_at {
        let elapsed = started.elapsed();
        let elapsed_str = if elapsed.as_secs() >= 60 {
            format!("{}m{:02}s", elapsed.as_secs() / 60, elapsed.as_secs() % 60)
        } else {
            format!("{:.1}s", elapsed.as_secs_f64())
        };
        lines.push(Line::from(vec![
            Span::styled("Elapsed: ", label_style),
            Span::styled(
                elapsed_str,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    if app.autochat_running
        && let Some(status) = app.autochat_status_label()
    {
        lines.push(Line::from(vec![
            Span::styled("Relay: ", label_style),
            Span::styled(status, Style::default().fg(Color::Cyan)),
        ]));
    }

    lines.push(Line::from(vec![
        Span::styled("Tool: ", label_style),
        Span::styled(
            tool_label,
            if app.current_tool.is_some() {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            },
        ),
    ]));
    lines.push(Line::from(""));
    lines.push(Line::styled(
        "Session",
        Style::default().add_modifier(Modifier::BOLD),
    ));
    lines.push(Line::from(vec![
        Span::styled("ID: ", label_style),
        Span::styled(format!("#{}", session_id), Style::default().fg(Color::Cyan)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Model: ", label_style),
        Span::styled(model_label.to_string(), Style::default().fg(Color::Green)),
    ]));
    let agent_display = if let Some(target) = &app.active_spawned_agent {
        format!("{} → @{} (focused)", app.current_agent, target)
    } else {
        app.current_agent.clone()
    };
    lines.push(Line::from(vec![
        Span::styled("Agent: ", label_style),
        Span::styled(agent_display, Style::default()),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Messages: ", label_style),
        Span::styled(message_count.to_string(), Style::default()),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Context: ", label_style),
        Span::styled(format!("{} turns", conversation_depth), Style::default()),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Tools used: ", label_style),
        Span::styled(app.tool_call_count.to_string(), Style::default()),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Protocol: ", label_style),
        Span::styled(
            format!("{} registered", app.protocol_registered_count()),
            Style::default().fg(Color::Cyan),
        ),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Archive: ", label_style),
        Span::styled(
            format!("{} records", app.archived_message_count),
            Style::default(),
        ),
    ]));
    let sync_style = if app.chat_sync_last_error.is_some() {
        Style::default().fg(Color::Red)
    } else if app.chat_sync_rx.is_some() {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    lines.push(Line::from(vec![
        Span::styled("Remote sync: ", label_style),
        Span::styled(
            app.chat_sync_status
                .as_deref()
                .unwrap_or("disabled")
                .to_string(),
            sync_style,
        ),
    ]));
    lines.push(Line::from(""));
    lines.push(Line::styled(
        "Sub-agents",
        Style::default().add_modifier(Modifier::BOLD),
    ));
    if app.spawned_agents.is_empty() {
        lines.push(Line::styled(
            "None (use /spawn <name> <instructions>)",
            Style::default().fg(Color::DarkGray),
        ));
    } else {
        for (name, agent) in app.spawned_agents.iter().take(4) {
            let status = if agent.is_processing { "⚡" } else { "●" };
            let is_registered = app.is_agent_protocol_registered(name);
            let protocol = if is_registered { "🔗" } else { "⚠" };
            let focused = if app.active_spawned_agent.as_deref() == Some(name.as_str()) {
                " [focused]"
            } else {
                ""
            };
            lines.push(Line::styled(
                format!(
                    "{status} {protocol} {} @{name}{focused}",
                    agent_avatar(name)
                ),
                if focused.is_empty() {
                    Style::default().fg(Color::Magenta)
                } else {
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD)
                },
            ));
            let profile = agent_profile(name);
            lines.push(Line::styled(
                format!("   {} — {}", profile.codename, profile.profile),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::DIM),
            ));
            lines.push(Line::styled(
                format!("   {}", agent.instructions),
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM),
            ));
            if is_registered {
                lines.push(Line::styled(
                    format!("   bus://local/{name}"),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::DIM),
                ));
            }
        }
        if app.spawned_agents.len() > 4 {
            lines.push(Line::styled(
                format!("… and {} more", app.spawned_agents.len() - 4),
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM),
            ));
        }
    }
    lines.push(Line::from(""));
    lines.push(Line::styled(
        "Shortcuts",
        Style::default().add_modifier(Modifier::BOLD),
    ));
    lines.push(Line::from(vec![
        Span::styled("F3      ", Style::default().fg(Color::Yellow)),
        Span::styled("Inspector", Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Ctrl+B  ", Style::default().fg(Color::Yellow)),
        Span::styled("Layout", Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Ctrl+Y  ", Style::default().fg(Color::Yellow)),
        Span::styled("Copy", Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Ctrl+M  ", Style::default().fg(Color::Yellow)),
        Span::styled("Model", Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Ctrl+O  ", Style::default().fg(Color::Yellow)),
        Span::styled("File Picker", Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Ctrl+S  ", Style::default().fg(Color::Yellow)),
        Span::styled("Swarm", Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("?       ", Style::default().fg(Color::Yellow)),
        Span::styled("Help", Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(""));
    lines.push(Line::styled(
        "Views",
        Style::default().add_modifier(Modifier::BOLD),
    ));
    lines.push(Line::styled(
        view_mode_compact_summary(),
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::DIM),
    ));

    let panel = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
    f.render_widget(panel, area);
}

fn render_webview_input(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let title = if app.is_processing {
        if let Some(started) = app.processing_started_at {
            let elapsed = started.elapsed();
            format!(" Processing ({:.0}s)... ", elapsed.as_secs_f64())
        } else {
            " Message (Processing...) ".to_string()
        }
    } else if app.autochat_running {
        format!(
            " {} ",
            app.autochat_status_label()
                .unwrap_or_else(|| "Autochat running…".to_string())
        )
    } else if app.input.starts_with('/') {
        // Show matching slash commands as hints
        let hint = match_slash_command_hint(&app.input);
        format!(" {} ", hint)
    } else if let Some(target) = &app.active_spawned_agent {
        format!(" Message to @{target} (use /agent main to exit) ")
    } else {
        " Message (Enter to send, / for commands) ".to_string()
    };

    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(Style::default().fg(if app.is_processing {
            Color::Yellow
        } else if app.autochat_running {
            Color::Cyan
        } else if app.input.starts_with('/') {
            Color::Magenta
        } else {
            theme.input_border_color.to_color()
        }));

    let input = Paragraph::new(app.input.as_str())
        .block(input_block)
        .wrap(Wrap { trim: false });
    f.render_widget(input, area);

    // Cursor: convert byte offset to display column (char count).
    let cursor_col = app.input[..app.cursor_position.min(app.input.len())]
        .chars()
        .count() as u16;
    f.set_cursor_position((area.x + cursor_col + 1, area.y + 1));
}

