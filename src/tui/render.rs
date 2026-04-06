//! TUI rendering: layout, chat view, webview, inspector, help overlay

use super::*;

fn ui(f: &mut Frame, app: &mut App, theme: &Theme) {
    // Check view mode
    if app.view_mode == ViewMode::Swarm {
        // Render swarm view
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Swarm view
                Constraint::Length(3), // Input
                Constraint::Length(1), // Status bar
            ])
            .split(f.area());

        // Swarm view
        render_swarm_view(f, &mut app.swarm_state, chunks[0]);

        // Input area (for returning to chat)
        let input_block = Block::default()
            .borders(Borders::ALL)
            .title(" Press Esc, Ctrl+S, or /view to return to chat ")
            .border_style(Style::default().fg(Color::Cyan));

        let input = Paragraph::new(app.input.as_str())
            .block(input_block)
            .wrap(Wrap { trim: false });
        f.render_widget(input, chunks[1]);

        // Status bar
        let mut status_line = if app.swarm_state.detail_mode {
            Line::from(vec![
                Span::styled(
                    " AGENT DETAIL ",
                    Style::default().fg(Color::Black).bg(Color::Cyan),
                ),
                Span::raw(" | "),
                Span::styled("Esc", Style::default().fg(Color::Yellow)),
                Span::raw(": Back to list | "),
                Span::styled("↑↓", Style::default().fg(Color::Yellow)),
                Span::raw(": Prev/Next agent | "),
                Span::styled("PgUp/PgDn", Style::default().fg(Color::Yellow)),
                Span::raw(": Scroll"),
            ])
        } else {
            Line::from(vec![
                Span::styled(
                    " SWARM MODE ",
                    Style::default().fg(Color::Black).bg(Color::Cyan),
                ),
                Span::raw(" | "),
                Span::styled("↑↓", Style::default().fg(Color::Yellow)),
                Span::raw(": Select | "),
                Span::styled("Enter", Style::default().fg(Color::Yellow)),
                Span::raw(": Detail | "),
                Span::styled("Esc", Style::default().fg(Color::Yellow)),
                Span::raw(": Back | "),
                Span::styled("Ctrl+S", Style::default().fg(Color::Yellow)),
                Span::raw(": Toggle view"),
            ])
        };
        app.append_bus_status_with_separator(&mut status_line.spans);
        let status = Paragraph::new(status_line);
        f.render_widget(status, chunks[2]);
        return;
    }

    // Ralph view
    if app.view_mode == ViewMode::Ralph {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Ralph view
                Constraint::Length(3), // Input
                Constraint::Length(1), // Status bar
            ])
            .split(f.area());

        render_ralph_view(f, &mut app.ralph_state, chunks[0]);

        let input_block = Block::default()
            .borders(Borders::ALL)
            .title(" Press Esc to return to chat ")
            .border_style(Style::default().fg(Color::Magenta));

        let input = Paragraph::new(app.input.as_str())
            .block(input_block)
            .wrap(Wrap { trim: false });
        f.render_widget(input, chunks[1]);

        let mut status_line = if app.ralph_state.detail_mode {
            Line::from(vec![
                Span::styled(
                    " STORY DETAIL ",
                    Style::default().fg(Color::Black).bg(Color::Magenta),
                ),
                Span::raw(" | "),
                Span::styled("Esc", Style::default().fg(Color::Yellow)),
                Span::raw(": Back to list | "),
                Span::styled("↑↓", Style::default().fg(Color::Yellow)),
                Span::raw(": Prev/Next story | "),
                Span::styled("PgUp/PgDn", Style::default().fg(Color::Yellow)),
                Span::raw(": Scroll"),
            ])
        } else {
            Line::from(vec![
                Span::styled(
                    " RALPH MODE ",
                    Style::default().fg(Color::Black).bg(Color::Magenta),
                ),
                Span::raw(" | "),
                Span::styled("↑↓", Style::default().fg(Color::Yellow)),
                Span::raw(": Select | "),
                Span::styled("Enter", Style::default().fg(Color::Yellow)),
                Span::raw(": Detail | "),
                Span::styled("Esc", Style::default().fg(Color::Yellow)),
                Span::raw(": Back"),
            ])
        };
        app.append_bus_status_with_separator(&mut status_line.spans);
        let status = Paragraph::new(status_line);
        f.render_widget(status, chunks[2]);
        return;
    }

    // Bus protocol log view
    if app.view_mode == ViewMode::BusLog {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Bus log view
                Constraint::Length(3), // Input
                Constraint::Length(1), // Status bar
            ])
            .split(f.area());

        render_bus_log(f, &mut app.bus_log_state, chunks[0]);

        let input_block = Block::default()
            .borders(Borders::ALL)
            .title(" Press Esc to return to chat ")
            .border_style(Style::default().fg(Color::Green));

        let input = Paragraph::new(app.input.as_str())
            .block(input_block)
            .wrap(Wrap { trim: false });
        f.render_widget(input, chunks[1]);

        let count_info = format!(
            " {}/{} ",
            app.bus_log_state.visible_count(),
            app.bus_log_state.total_count()
        );
        let mut status_line = Line::from(vec![
            Span::styled(
                " BUS LOG ",
                Style::default().fg(Color::Black).bg(Color::Green),
            ),
            Span::raw(count_info),
            Span::raw("| "),
            Span::styled("↑↓", Style::default().fg(Color::Yellow)),
            Span::raw(": Select | "),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(": Detail | "),
            Span::styled("c", Style::default().fg(Color::Yellow)),
            Span::raw(": Clear | "),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw(": Back"),
        ]);
        app.append_bus_status_with_separator(&mut status_line.spans);
        let status = Paragraph::new(status_line);
        f.render_widget(status, chunks[2]);
        return;
    }

    // Protocol registry view
    if app.view_mode == ViewMode::Protocol {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Protocol details
                Constraint::Length(3), // Input
                Constraint::Length(1), // Status bar
            ])
            .split(f.area());

        render_protocol_registry(f, app, theme, chunks[0]);

        let input_block = Block::default()
            .borders(Borders::ALL)
            .title(" Press Esc to return to chat ")
            .border_style(Style::default().fg(Color::Blue));

        let input = Paragraph::new(app.input.as_str())
            .block(input_block)
            .wrap(Wrap { trim: false });
        f.render_widget(input, chunks[1]);

        let cards = app.protocol_cards();
        let mut status_line = Line::from(vec![
            Span::styled(
                " PROTOCOL REGISTRY ",
                Style::default().fg(Color::Black).bg(Color::Blue),
            ),
            Span::raw(format!(" {} cards | ", cards.len())),
            Span::styled("↑↓", Style::default().fg(Color::Yellow)),
            Span::raw(": Select | "),
            Span::styled("PgUp/PgDn", Style::default().fg(Color::Yellow)),
            Span::raw(": Scroll detail | "),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw(": Back"),
        ]);
        app.append_bus_status_with_separator(&mut status_line.spans);
        let status = Paragraph::new(status_line);
        f.render_widget(status, chunks[2]);
        return;
    }

    // Model picker view
    if app.view_mode == ViewMode::ModelPicker {
        let area = centered_rect(70, 70, f.area());
        f.render_widget(Clear, area);

        let filter_display = if app.model_picker_filter.is_empty() {
            "type to filter".to_string()
        } else {
            format!("filter: {}", app.model_picker_filter)
        };

        let (bus_label, _) = app.bus_status_label_and_color();
        let picker_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(
                " Select Model (↑↓ navigate, Enter select, Esc cancel) [{}] [{}] ",
                filter_display, bus_label
            ))
            .border_style(Style::default().fg(Color::Magenta));

        let filtered = app.filtered_models();
        let mut list_lines: Vec<Line> = Vec::new();
        list_lines.push(Line::from(""));

        if let Some(ref active) = app.active_model {
            list_lines.push(Line::styled(
                format!("  Current: {}", active),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::DIM),
            ));
            list_lines.push(Line::from(""));
        }

        if filtered.is_empty() {
            list_lines.push(Line::styled(
                "  No models match filter",
                Style::default().fg(Color::DarkGray),
            ));
        } else {
            let mut current_provider = String::new();
            for (display_idx, (_, (label, _, human_name))) in filtered.iter().enumerate() {
                let provider = label.split('/').next().unwrap_or("");
                if provider != current_provider {
                    if !current_provider.is_empty() {
                        list_lines.push(Line::from(""));
                    }
                    list_lines.push(Line::styled(
                        format!("  ─── {} ───", provider),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ));
                    current_provider = provider.to_string();
                }

                let is_selected = display_idx == app.model_picker_selected;
                let is_active = app.active_model.as_deref() == Some(label.as_str());
                let marker = if is_selected { "▶" } else { " " };
                let active_marker = if is_active { " ✓" } else { "" };
                let model_id = label.split('/').skip(1).collect::<Vec<_>>().join("/");
                // Show human name if different from ID
                let display = if human_name != &model_id && !human_name.is_empty() {
                    format!("{} ({})", human_name, model_id)
                } else {
                    model_id
                };

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD)
                } else if is_active {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                };

                list_lines.push(Line::styled(
                    format!("  {} {}{}", marker, display, active_marker),
                    style,
                ));
            }
        }

        let list = Paragraph::new(list_lines)
            .block(picker_block)
            .wrap(Wrap { trim: false });
        f.render_widget(list, area);
        return;
    }

    // Session picker view
    if app.view_mode == ViewMode::SessionPicker {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Session list
                Constraint::Length(1), // Status bar
            ])
            .split(f.area());

        // Build title with filter display
        let filter_display = if app.session_picker_filter.is_empty() {
            String::new()
        } else {
            format!(" [filter: {}]", app.session_picker_filter)
        };

        let list_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(
                " Sessions (↑↓ navigate, Enter load, d delete, Esc cancel){} ",
                filter_display
            ))
            .border_style(Style::default().fg(Color::Cyan));

        let mut list_lines: Vec<Line> = Vec::new();
        list_lines.push(Line::from(""));

        let filtered = app.filtered_sessions();
        if filtered.is_empty() {
            if app.session_picker_filter.is_empty() {
                list_lines.push(Line::styled(
                    "  No sessions found.",
                    Style::default().fg(Color::DarkGray),
                ));
            } else {
                list_lines.push(Line::styled(
                    format!("  No sessions matching '{}'", app.session_picker_filter),
                    Style::default().fg(Color::DarkGray),
                ));
            }
        }

        for (display_idx, (_orig_idx, session)) in filtered.iter().enumerate() {
            let is_selected = display_idx == app.session_picker_selected;
            let is_active = app
                .session
                .as_ref()
                .map(|s| s.id == session.id)
                .unwrap_or(false);
            let title = session.title.as_deref().unwrap_or("(untitled)");
            let date = session.updated_at.format("%Y-%m-%d %H:%M");
            let active_marker = if is_active { " ●" } else { "" };
            let line_str = format!(
                " {} {}{} - {} ({} msgs)",
                if is_selected { "▶" } else { " " },
                title,
                active_marker,
                date,
                session.message_count
            );

            let style = if is_selected && app.session_picker_confirm_delete {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else if is_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else if is_active {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };

            list_lines.push(Line::styled(line_str, style));

            // Show details for selected item
            if is_selected {
                if app.session_picker_confirm_delete {
                    list_lines.push(Line::styled(
                        "   ⚠ Press d again to confirm delete, Esc to cancel",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ));
                } else {
                    list_lines.push(Line::styled(
                        format!("   Agent: {} | ID: {}", session.agent, session.id),
                        Style::default().fg(Color::DarkGray),
                    ));
                }
            }
        }

        let list = Paragraph::new(list_lines)
            .block(list_block)
            .wrap(Wrap { trim: false });
        f.render_widget(list, chunks[0]);

        // Status bar with more actions
        let mut status_spans = vec![
            Span::styled(
                " SESSION PICKER ",
                Style::default().fg(Color::Black).bg(Color::Cyan),
            ),
            Span::raw(" "),
            Span::styled("↑↓", Style::default().fg(Color::Yellow)),
            Span::raw(": Nav "),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(": Load "),
            Span::styled("d", Style::default().fg(Color::Yellow)),
            Span::raw(": Delete "),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw(": Cancel "),
        ];
        if !app.session_picker_filter.is_empty() || !app.session_picker_list.is_empty() {
            status_spans.push(Span::styled("Type", Style::default().fg(Color::Yellow)));
            status_spans.push(Span::raw(": Filter "));
        }
        let limit = std::env::var("CODETETHER_SESSION_PICKER_LIMIT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
        // Pagination info
        if app.session_picker_offset > 0 || app.session_picker_list.len() >= limit {
            status_spans.push(Span::styled("n", Style::default().fg(Color::Yellow)));
            status_spans.push(Span::raw(": Next "));
            if app.session_picker_offset > 0 {
                status_spans.push(Span::styled("p", Style::default().fg(Color::Yellow)));
                status_spans.push(Span::raw(": Prev "));
            }
        }
        let total = app.session_picker_list.len();
        let showing = filtered.len();
        let offset_display = if app.session_picker_offset > 0 {
            format!("+{}", app.session_picker_offset)
        } else {
            String::new()
        };
        if showing < total {
            status_spans.push(Span::styled(
                format!("{}{}/{}", offset_display, showing, total),
                Style::default().fg(Color::DarkGray),
            ));
        }

        app.append_bus_status_with_separator(&mut status_spans);
        let status = Paragraph::new(Line::from(status_spans));
        f.render_widget(status, chunks[1]);
        return;
    }

    // Agent picker view
    if app.view_mode == ViewMode::AgentPicker {
        let area = centered_rect(70, 70, f.area());
        f.render_widget(Clear, area);

        let filter_display = if app.agent_picker_filter.is_empty() {
            "type to filter".to_string()
        } else {
            format!("filter: {}", app.agent_picker_filter)
        };

        let (bus_label, _) = app.bus_status_label_and_color();
        let picker_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(
                " Select Agent (↑↓ navigate, Enter focus, m main chat, Esc cancel) [{}] [{}] ",
                filter_display, bus_label
            ))
            .border_style(Style::default().fg(Color::Magenta));

        let filtered = app.filtered_spawned_agents();
        let mut list_lines: Vec<Line> = Vec::new();
        list_lines.push(Line::from(""));

        if let Some(ref active) = app.active_spawned_agent {
            list_lines.push(Line::styled(
                format!("  Current focus: @{}", active),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::DIM),
            ));
            list_lines.push(Line::from(""));
        }

        if filtered.is_empty() {
            list_lines.push(Line::styled(
                "  No spawned agents match filter",
                Style::default().fg(Color::DarkGray),
            ));
        } else {
            for (display_idx, (name, instructions, is_processing, is_registered)) in
                filtered.iter().enumerate()
            {
                let is_selected = display_idx == app.agent_picker_selected;
                let is_focused = app.active_spawned_agent.as_deref() == Some(name.as_str());
                let marker = if is_selected { "▶" } else { " " };
                let focused_marker = if is_focused { " ✓" } else { "" };
                let status = if *is_processing { "⚡" } else { "●" };
                let protocol = if *is_registered { "🔗" } else { "⚠" };
                let avatar = agent_avatar(name);

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD)
                } else if is_focused {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                };

                list_lines.push(Line::styled(
                    format!("  {marker} {status} {protocol} {avatar} @{name}{focused_marker}"),
                    style,
                ));

                if is_selected {
                    let profile = agent_profile(name);
                    list_lines.push(Line::styled(
                        format!("     profile: {} — {}", profile.codename, profile.profile),
                        Style::default().fg(Color::Cyan),
                    ));
                    list_lines.push(Line::styled(
                        format!("     {}", instructions),
                        Style::default().fg(Color::DarkGray),
                    ));
                    list_lines.push(Line::styled(
                        format!(
                            "     protocol: {}",
                            if *is_registered {
                                "registered"
                            } else {
                                "not registered"
                            }
                        ),
                        if *is_registered {
                            Style::default().fg(Color::Green)
                        } else {
                            Style::default().fg(Color::Yellow)
                        },
                    ));
                }
            }
        }

        let list = Paragraph::new(list_lines)
            .block(picker_block)
            .wrap(Wrap { trim: false });
        f.render_widget(list, area);
        return;
    }

    // File picker view
    if app.view_mode == ViewMode::FilePicker {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // File list
                Constraint::Length(8), // Preview
                Constraint::Length(1), // Status bar
            ])
            .split(f.area());

        let filter_display = if app.file_picker_filter.is_empty() {
            "filter: (type to search)".to_string()
        } else {
            format!("filter: {}", app.file_picker_filter)
        };

        let current_dir = app.file_picker_dir.display().to_string();
        let filtered = app.filtered_file_picker_entries();
        let list_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(
                " File Picker [{}] ",
                truncate_with_ellipsis(&current_dir, 70)
            ))
            .border_style(Style::default().fg(Color::Yellow));

        let mut list_lines: Vec<Line> = Vec::new();
        list_lines.push(Line::styled(
            format!(
                "  {} shown / {} total • {}",
                filtered.len(),
                app.file_picker_entries.len(),
                filter_display
            ),
            Style::default().fg(Color::DarkGray),
        ));
        list_lines.push(Line::styled(
            "  Enter on file: attach to composer • Enter on folder: open folder",
            Style::default().fg(Color::DarkGray),
        ));
        list_lines.push(Line::from(""));

        if filtered.is_empty() {
            list_lines.push(Line::styled(
                "  No files match filter",
                Style::default().fg(Color::DarkGray),
            ));
        } else {
            for (display_idx, (_, entry)) in filtered.iter().enumerate() {
                let is_selected = display_idx == app.file_picker_selected;
                let marker = if is_selected { "▶" } else { " " };
                let (tag, detail) = match entry.kind {
                    FilePickerEntryKind::Parent => ("[..]", "parent".to_string()),
                    FilePickerEntryKind::Directory => ("[dir]", "folder".to_string()),
                    FilePickerEntryKind::File => (
                        "[file]",
                        entry
                            .size_bytes
                            .map(format_bytes)
                            .unwrap_or_else(|| "-".to_string()),
                    ),
                };

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else if entry.kind == FilePickerEntryKind::Directory {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                };

                list_lines.push(Line::styled(
                    format!("  {marker} {tag} {}  ({detail})", entry.name),
                    style,
                ));

                if is_selected {
                    list_lines.push(Line::styled(
                        format!(
                            "     {}",
                            display_path_for_workspace(&entry.path, &app.workspace_dir)
                        ),
                        Style::default().fg(Color::DarkGray),
                    ));
                }
            }
        }

        let list = Paragraph::new(list_lines)
            .block(list_block)
            .wrap(Wrap { trim: false });
        f.render_widget(list, chunks[0]);

        let preview_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(
                " {} ",
                truncate_with_ellipsis(&app.file_picker_preview_title, 90)
            ))
            .border_style(Style::default().fg(Color::Cyan));

        let preview_lines: Vec<Line> = if app.file_picker_preview_lines.is_empty() {
            vec![Line::styled(
                "  No preview available.",
                Style::default().fg(Color::DarkGray),
            )]
        } else {
            app.file_picker_preview_lines
                .iter()
                .map(|line| Line::styled(format!("  {line}"), Style::default()))
                .collect()
        };
        let preview = Paragraph::new(preview_lines)
            .block(preview_block)
            .wrap(Wrap { trim: false });
        f.render_widget(preview, chunks[1]);

        let mut status_line = Line::from(vec![
            Span::styled(
                " FILE PICKER ",
                Style::default().fg(Color::Black).bg(Color::Yellow),
            ),
            Span::raw(" "),
            Span::styled("↑↓/j/k", Style::default().fg(Color::Yellow)),
            Span::raw(": Nav "),
            Span::styled("PgUp/PgDn", Style::default().fg(Color::Yellow)),
            Span::raw(": Page "),
            Span::styled("Enter/l", Style::default().fg(Color::Yellow)),
            Span::raw(": Open folder / Attach file "),
            Span::styled("h/←", Style::default().fg(Color::Yellow)),
            Span::raw(": Parent "),
            Span::styled("Type", Style::default().fg(Color::Yellow)),
            Span::raw(": Filter "),
            Span::styled("Ctrl+U", Style::default().fg(Color::Yellow)),
            Span::raw(": Clear "),
            Span::styled("F5", Style::default().fg(Color::Yellow)),
            Span::raw(": Refresh "),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw(": Cancel"),
        ]);
        app.append_bus_status_with_separator(&mut status_line.spans);
        let status = Paragraph::new(status_line);
        f.render_widget(status, chunks[2]);
        return;
    }

    // Build message lines once here — shared by both webview and classic layouts.
    // This avoids the per-frame allocation cost of calling build_message_lines twice.
    // We use a conservative width estimate (terminal width - 8) for the cache key;
    // the exact width is refined per-layout below but the content rarely changes.
    let prefetch_width = f.area().width.saturating_sub(8) as usize;
    let _ = app.get_or_build_message_lines(theme, prefetch_width);

    if app.chat_layout == ChatLayoutMode::Webview && render_webview_chat(f, app, theme) {
        render_help_overlay_if_needed(f, app, theme);
        return;
    }

    // Chat view (default)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // Messages
            Constraint::Length(3), // Input
            Constraint::Length(1), // Status bar
        ])
        .split(f.area());

    // Messages area with theme-based styling
    let messages_area = chunks[0];
    let model_label = app.active_model.as_deref().unwrap_or("auto");
    let target_label = app
        .active_spawned_agent
        .as_ref()
        .map(|name| format!(" @{}", name))
        .unwrap_or_default();
    let messages_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(
            " CodeTether Agent [{}{}] model:{} ",
            app.current_agent, target_label, model_label
        ))
        .border_style(Style::default().fg(theme.border_color.to_color()));

    let max_width = messages_area.width.saturating_sub(4) as usize;
    // Use cached lines — only rebuilt when messages/state actually changes.
    let message_lines = app.get_or_build_message_lines(theme, max_width).to_vec();

    // Calculate scroll position
    let total_lines = message_lines.len();
    let visible_lines = messages_area.height.saturating_sub(2) as usize;
    let max_scroll = total_lines.saturating_sub(visible_lines);
    // Keep last_max_scroll in sync so key handlers are always accurate.
    app.last_max_scroll = max_scroll;
    // SCROLL_BOTTOM means "follow latest", otherwise clamp to max_scroll.
    // In newest-first mode, latest content is at the top.
    let scroll = if app.scroll >= SCROLL_BOTTOM {
        0
    } else {
        app.scroll.min(max_scroll)
    };

    // Render messages with scrolling
    let messages_paragraph = Paragraph::new(
        message_lines[scroll..(scroll + visible_lines.min(total_lines)).min(total_lines)].to_vec(),
    )
    .block(messages_block.clone())
    .wrap(Wrap { trim: false });

    f.render_widget(messages_paragraph, messages_area);

    // Render scrollbar if needed
    if total_lines > visible_lines {
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .symbols(ratatui::symbols::scrollbar::VERTICAL)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        // Scrollbar: content_length = max_scroll+1 so the thumb tracks the
        // scroll position correctly and reaches the very bottom.
        let mut scrollbar_state = ScrollbarState::new(max_scroll + 1).position(scroll);

        let scrollbar_area = Rect::new(
            messages_area.right() - 1,
            messages_area.top() + 1,
            1,
            messages_area.height - 2,
        );

        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }

    // Input area
    let input_title = if app.is_processing {
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
        let hint = match_slash_command_hint(&app.input);
        format!(" {} ", hint)
    } else if let Some(target) = &app.active_spawned_agent {
        format!(" Message to @{target} (use /agent main to exit) ")
    } else {
        " Message (Enter to send, / for commands) ".to_string()
    };
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(input_title)
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
    f.render_widget(input, chunks[1]);

    // Cursor: convert byte offset to display column (char count).
    // cursor_position is a byte index; rendering as a column directly
    // causes the cursor to appear at the wrong position for multi-byte chars.
    let cursor_col = app.input[..app.cursor_position.min(app.input.len())]
        .chars()
        .count() as u16;
    f.set_cursor_position((chunks[1].x + cursor_col + 1, chunks[1].y + 1));

    // Enhanced status bar with token display and model info
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
    f.render_widget(status, chunks[2]);

    render_help_overlay_if_needed(f, app, theme);
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

fn build_message_lines(app: &App, theme: &Theme, max_width: usize) -> Vec<Line<'static>> {
    let mut message_lines = Vec::new();
    let separator_width = max_width.min(60);

    for (idx, message) in app.messages.iter().rev().enumerate() {
        let role_style = theme.get_role_style(&message.role);

        // Add a thin separator between messages (not before the first)
        if idx > 0 {
            let sep_char = match message.role.as_str() {
                "tool" => "·",
                _ => "─",
            };
            message_lines.push(Line::from(Span::styled(
                sep_char.repeat(separator_width),
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            )));
        }

        // Role icons for better visual hierarchy
        let role_icon = match message.role.as_str() {
            "user" => "▸ ",
            "assistant" => "◆ ",
            "system" => "⚙ ",
            "tool" => "⚡",
            _ => "  ",
        };

        let header_line = {
            let mut spans = vec![
                Span::styled(
                    format!("[{}] ", message.timestamp),
                    Style::default()
                        .fg(theme.timestamp_color.to_color())
                        .add_modifier(Modifier::DIM),
                ),
                Span::styled(role_icon, role_style),
                Span::styled(message.role.clone(), role_style),
            ];
            if let Some(ref agent) = message.agent_name {
                let profile = agent_profile(agent);
                spans.push(Span::styled(
                    format!(" {} @{agent} ‹{}›", agent_avatar(agent), profile.codename),
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ));
            }
            Line::from(spans)
        };
        message_lines.push(header_line);

        match &message.message_type {
            MessageType::ToolCall {
                name,
                arguments_preview,
                arguments_len,
                truncated,
            } => {
                let tool_header = Line::from(vec![
                    Span::styled("  🔧 ", Style::default().fg(Color::Yellow)),
                    Span::styled(
                        format!("Tool: {}", name),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]);
                message_lines.push(tool_header);

                if arguments_preview.trim().is_empty() {
                    message_lines.push(Line::from(vec![
                        Span::styled("  │ ", Style::default().fg(Color::DarkGray)),
                        Span::styled(
                            "(no arguments)",
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::DIM),
                        ),
                    ]));
                } else {
                    for line in arguments_preview.lines() {
                        let args_line = Line::from(vec![
                            Span::styled("  │ ", Style::default().fg(Color::DarkGray)),
                            Span::styled(line.to_string(), Style::default().fg(Color::DarkGray)),
                        ]);
                        message_lines.push(args_line);
                    }
                }

                if *truncated {
                    let args_line = Line::from(vec![
                        Span::styled("  │ ", Style::default().fg(Color::DarkGray)),
                        Span::styled(
                            format!("... (truncated; {} bytes)", arguments_len),
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::DIM),
                        ),
                    ]);
                    message_lines.push(args_line);
                }
            }
            MessageType::ToolResult {
                name,
                output_preview,
                output_len,
                truncated,
                success,
                duration_ms,
            } => {
                let icon = if *success { "✅" } else { "❌" };
                let result_header = Line::from(vec![
                    Span::styled(
                        format!("  {icon} "),
                        Style::default().fg(if *success { Color::Green } else { Color::Red }),
                    ),
                    Span::styled(
                        format!("Result from {}", name),
                        Style::default()
                            .fg(if *success { Color::Green } else { Color::Red })
                            .add_modifier(Modifier::BOLD),
                    ),
                ]);
                message_lines.push(result_header);

                let status_line = format!(
                    "  │ status: {}{}",
                    if *success { "success" } else { "failure" },
                    duration_ms
                        .map(|ms| format!(" • {}", format_duration_ms(ms)))
                        .unwrap_or_default()
                );
                message_lines.push(Line::from(vec![
                    Span::styled("  │ ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        status_line.trim_start_matches("  │ ").to_string(),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]));

                if output_preview.trim().is_empty() {
                    message_lines.push(Line::from(vec![
                        Span::styled("  │ ", Style::default().fg(Color::DarkGray)),
                        Span::styled(
                            "(empty output)",
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::DIM),
                        ),
                    ]));
                } else {
                    for line in output_preview.lines() {
                        let output_line = Line::from(vec![
                            Span::styled("  │ ", Style::default().fg(Color::DarkGray)),
                            Span::styled(line.to_string(), Style::default().fg(Color::DarkGray)),
                        ]);
                        message_lines.push(output_line);
                    }
                }

                if *truncated {
                    message_lines.push(Line::from(vec![
                        Span::styled("  │ ", Style::default().fg(Color::DarkGray)),
                        Span::styled(
                            format!("... (truncated; {} bytes)", output_len),
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::DIM),
                        ),
                    ]));
                }
            }
            MessageType::Text(text) => {
                let formatter = MessageFormatter::new(max_width);
                let formatted_content = formatter.format_content(text, &message.role);
                message_lines.extend(formatted_content);
            }
            MessageType::Thinking(text) => {
                let thinking_style = Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM | Modifier::ITALIC);
                message_lines.push(Line::from(Span::styled(
                    "  💭 Thinking...",
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::DIM),
                )));
                // Show truncated thinking content
                let max_thinking_lines = 8;
                let mut iter = text.lines();
                let mut shown = 0usize;
                while shown < max_thinking_lines {
                    let Some(line) = iter.next() else { break };
                    message_lines.push(Line::from(vec![
                        Span::styled("  │ ", Style::default().fg(Color::DarkGray)),
                        Span::styled(line.to_string(), thinking_style),
                    ]));
                    shown += 1;
                }
                if iter.next().is_some() {
                    message_lines.push(Line::from(Span::styled(
                        "  │ ... (truncated)",
                        thinking_style,
                    )));
                }
            }
            MessageType::Image { url, mime_type } => {
                let formatter = MessageFormatter::new(max_width);
                let image_line = formatter.format_image(url, mime_type.as_deref());
                message_lines.push(image_line);
            }
            MessageType::File { path, mime_type } => {
                let mime_label = mime_type.as_deref().unwrap_or("unknown type");
                let file_header = Line::from(vec![
                    Span::styled("  📎 ", Style::default().fg(Color::Cyan)),
                    Span::styled(
                        format!("File: {}", path),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!(" ({})", mime_label),
                        Style::default()
                            .fg(Color::DarkGray)
                            .add_modifier(Modifier::DIM),
                    ),
                ]);
                message_lines.push(file_header);
            }
        }

        // Show usage indicator after assistant messages
        if message.role == "assistant"
            && let Some(ref meta) = message.usage_meta
        {
            let duration_str = if meta.duration_ms >= 60_000 {
                format!(
                    "{}m{:02}.{}s",
                    meta.duration_ms / 60_000,
                    (meta.duration_ms % 60_000) / 1000,
                    (meta.duration_ms % 1000) / 100
                )
            } else {
                format!(
                    "{}.{}s",
                    meta.duration_ms / 1000,
                    (meta.duration_ms % 1000) / 100
                )
            };
            let tokens_str = format!("{}→{} tokens", meta.prompt_tokens, meta.completion_tokens);
            let cost_str = match meta.cost_usd {
                Some(c) if c < 0.01 => format!("${:.4}", c),
                Some(c) => format!("${:.2}", c),
                None => String::new(),
            };
            let dim_style = Style::default()
                .fg(theme.timestamp_color.to_color())
                .add_modifier(Modifier::DIM);
            let mut spans = vec![Span::styled(
                format!("  ⏱ {} │ 📊 {}", duration_str, tokens_str),
                dim_style,
            )];
            if !cost_str.is_empty() {
                spans.push(Span::styled(format!(" │ 💰 {}", cost_str), dim_style));
            }
            message_lines.push(Line::from(spans));
        }

        message_lines.push(Line::from(""));
    }

    // Show streaming text preview (text arriving before TextComplete finalizes it)
    if let Some(ref streaming) = app.streaming_text
        && !streaming.is_empty()
    {
        message_lines.push(Line::from(Span::styled(
            "─".repeat(separator_width),
            Style::default()
                .fg(theme.timestamp_color.to_color())
                .add_modifier(Modifier::DIM),
        )));
        message_lines.push(Line::from(vec![
            Span::styled(
                format!("[{}] ", chrono::Local::now().format("%H:%M")),
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            ),
            Span::styled("◆ ", theme.get_role_style("assistant")),
            Span::styled("assistant", theme.get_role_style("assistant")),
            Span::styled(
                " (streaming...)",
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            ),
        ]));
        let formatter = MessageFormatter::new(max_width);
        let formatted = formatter.format_content(streaming, "assistant");
        message_lines.extend(formatted);
        message_lines.push(Line::from(""));
    }

    let mut agent_streams = app.streaming_agent_texts.iter().collect::<Vec<_>>();
    agent_streams.sort_by(|(a, _), (b, _)| a.to_lowercase().cmp(&b.to_lowercase()));
    for (agent, streaming) in agent_streams {
        if streaming.is_empty() {
            continue;
        }

        let profile = agent_profile(agent);

        message_lines.push(Line::from(Span::styled(
            "─".repeat(separator_width),
            Style::default()
                .fg(theme.timestamp_color.to_color())
                .add_modifier(Modifier::DIM),
        )));
        message_lines.push(Line::from(vec![
            Span::styled(
                format!("[{}] ", chrono::Local::now().format("%H:%M")),
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            ),
            Span::styled("◆ ", theme.get_role_style("assistant")),
            Span::styled("assistant", theme.get_role_style("assistant")),
            Span::styled(
                format!(" {} @{} ‹{}›", agent_avatar(agent), agent, profile.codename),
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " (streaming...)",
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            ),
        ]));

        let formatter = MessageFormatter::new(max_width);
        let formatted = formatter.format_content(streaming, "assistant");
        message_lines.extend(formatted);
        message_lines.push(Line::from(""));
    }

    if app.is_processing {
        let spinner = current_spinner_frame();

        // Elapsed time display
        let elapsed_str = if let Some(started) = app.processing_started_at {
            let elapsed = started.elapsed();
            if elapsed.as_secs() >= 60 {
                format!(" {}m{:02}s", elapsed.as_secs() / 60, elapsed.as_secs() % 60)
            } else {
                format!(" {:.1}s", elapsed.as_secs_f64())
            }
        } else {
            String::new()
        };

        let processing_line = Line::from(vec![
            Span::styled(
                format!("[{}] ", chrono::Local::now().format("%H:%M")),
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            ),
            Span::styled("◆ ", theme.get_role_style("assistant")),
            Span::styled("assistant", theme.get_role_style("assistant")),
            Span::styled(
                elapsed_str,
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            ),
        ]);
        message_lines.push(processing_line);

        let (status_text, status_color) = if let Some(ref tool) = app.current_tool {
            (format!("  {spinner} Running: {}", tool), Color::Cyan)
        } else {
            (
                format!(
                    "  {} {}",
                    spinner,
                    app.processing_message.as_deref().unwrap_or("Thinking...")
                ),
                Color::Yellow,
            )
        };

        let indicator_line = Line::from(vec![Span::styled(
            status_text,
            Style::default()
                .fg(status_color)
                .add_modifier(Modifier::BOLD),
        )]);
        message_lines.push(indicator_line);
        message_lines.push(Line::from(""));
    }

    if app.autochat_running {
        let status_text = app
            .autochat_status_label()
            .unwrap_or_else(|| "Autochat running…".to_string());
        message_lines.push(Line::from(Span::styled(
            "─".repeat(separator_width),
            Style::default()
                .fg(theme.timestamp_color.to_color())
                .add_modifier(Modifier::DIM),
        )));
        message_lines.push(Line::from(vec![
            Span::styled(
                format!("[{}] ", chrono::Local::now().format("%H:%M")),
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            ),
            Span::styled("⚙ ", theme.get_role_style("system")),
            Span::styled(
                status_text,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        message_lines.push(Line::from(""));
    }

    message_lines
}

fn match_slash_command_hint(input: &str) -> String {

fn message_clipboard_text(message: &ChatMessage) -> String {
    let mut prefix = String::new();
    if let Some(agent) = &message.agent_name {
        prefix = format!("@{agent}\n");
    }

    match &message.message_type {
        MessageType::Text(text) => format!("{prefix}{text}"),
        MessageType::Thinking(text) => format!("{prefix}{text}"),
        MessageType::Image { url, .. } => format!("{prefix}{url}"),
        MessageType::File { path, .. } => format!("{prefix}{path}"),
        MessageType::ToolCall {
            name,
            arguments_preview,
            ..
        } => format!("{prefix}Tool call: {name}\n{arguments_preview}"),
        MessageType::ToolResult {
            name,
            output_preview,
            ..
        } => format!("{prefix}Tool result: {name}\n{output_preview}"),
    }
}

fn copy_text_to_clipboard_best_effort(text: &str) -> Result<&'static str, String> {
    if text.trim().is_empty() {
        return Err("empty text".to_string());
    }

    // 1) Try system clipboard first (works locally when a clipboard provider is available)
    match arboard::Clipboard::new().and_then(|mut clipboard| clipboard.set_text(text.to_string())) {
        Ok(()) => return Ok("system clipboard"),
        Err(e) => {
            tracing::debug!(error = %e, "System clipboard unavailable; falling back to OSC52");
        }
    }

    // 2) Fallback: OSC52 (works in many terminals, including remote SSH sessions)
    osc52_copy(text).map_err(|e| format!("osc52 copy failed: {e}"))?;
    Ok("OSC52")
}

fn osc52_copy(text: &str) -> std::io::Result<()> {
