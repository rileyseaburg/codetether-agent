//! Main TUI event loop

use super::*;

async fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::new();
    // Use paginated session loading - default 100, configurable via CODETETHER_SESSION_PICKER_LIMIT
    let limit = std::env::var("CODETETHER_SESSION_PICKER_LIMIT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(100);
    if let Ok(sessions) = list_sessions_paged(&app.workspace_dir, limit, 0).await {
        app.update_cached_sessions(sessions);
    }

    // Create agent bus and subscribe the TUI as an observer
    let bus = std::sync::Arc::new(crate::bus::AgentBus::new());
    let mut bus_handle = bus.handle("tui-observer");
    let (bus_tx, bus_rx) = mpsc::channel::<crate::bus::BusEnvelope>(512);
    app.bus_log_rx = Some(bus_rx);
    app.bus = Some(bus.clone());

    // Auto-start S3 sink if MinIO is configured (set MINIO_ENDPOINT to enable)
    crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());

    // Spawn a forwarder task: bus broadcast → mpsc channel for the TUI event loop
    tokio::spawn(async move {
        while let Some(env) = bus_handle.recv().await {
            if bus_tx.send(env).await.is_err() {
                break; // TUI closed
            }
        }
    });

    // Load configuration and theme
    let mut config = Config::load().await?;
    let mut theme = crate::tui::theme_utils::validate_theme(&config.load_theme());

    match TuiWorkerBridge::spawn(config.a2a.server_url.clone(), None, None, bus.clone()).await {
        Ok(Some(bridge)) => {
            tracing::info!(
                worker_id = %bridge.worker_id,
                worker_name = %bridge.worker_name,
                "TUI worker bridge connected"
            );
            app.worker_bridge = Some(bridge);
        }
        Ok(None) => {
            tracing::debug!("TUI worker bridge disabled (no server/token)");
        }
        Err(err) => {
            tracing::warn!(error = %err, "Failed to initialize TUI worker bridge");
        }
    }

    // Preload provider registry in the background so the UI stays responsive on first submit
    // (Vault/provider initialization can be slow, e.g. Vertex GLM service account parsing).
    let (provider_tx, mut provider_rx) =
        mpsc::channel::<Result<crate::provider::ProviderRegistry>>(1);
    {
        let config_for_providers = config.clone();
        tokio::spawn(async move {
            let registry = match crate::provider::ProviderRegistry::from_vault().await {
                Ok(registry) => Ok(registry),
                Err(vault_err) => {
                    tracing::warn!(
                        error = %vault_err,
                        "Provider registry from_vault failed; falling back to config/env"
                    );
                    crate::provider::ProviderRegistry::from_config(&config_for_providers).await
                }
            };
            let _ = provider_tx.send(registry).await;
        });
    }

    let secure_environment = is_secure_environment();
    app.secure_environment = secure_environment;

    match parse_chat_sync_config(secure_environment).await {
        Ok(Some(sync_config)) => {
            if let Some(archive_path) = app.chat_archive_path.clone() {
                let (chat_sync_tx, chat_sync_rx) = mpsc::channel::<ChatSyncUiEvent>(64);
                app.chat_sync_rx = Some(chat_sync_rx);
                app.chat_sync_status = Some("Starting remote archive sync worker…".to_string());
                tokio::spawn(async move {
                    run_chat_sync_worker(chat_sync_tx, archive_path, sync_config).await;
                });
            } else {
                let message = "Remote chat sync is enabled, but local archive path is unavailable.";
                if secure_environment {
                    return Err(anyhow::anyhow!(
                        "{message} Secure environment requires remote chat sync."
                    ));
                }
                app.messages.push(ChatMessage::new("system", message));
            }
        }
        Ok(None) => {}
        Err(err) => {
            if secure_environment {
                return Err(anyhow::anyhow!(
                    "Secure environment requires remote chat sync: {err}"
                ));
            }
            app.messages.push(ChatMessage::new(
                "system",
                format!("Remote chat sync disabled due to configuration error: {err}"),
            ));
        }
    }

    // Track last config modification time for hot-reloading
    let _config_paths = [
        std::path::PathBuf::from("./codetether.toml"),
        std::path::PathBuf::from("./.codetether/config.toml"),
    ];

    let _global_config_path = directories::ProjectDirs::from("com", "codetether", "codetether")
        .map(|dirs| dirs.config_dir().join("config.toml"));

    let mut last_check = Instant::now();
    let mut event_stream = EventStream::new();

    // Background session refresh — fires every 5s, sends results via channel
    let (session_tx, mut session_rx) = mpsc::channel::<Vec<crate::session::SessionSummary>>(1);
    {
        let workspace_dir = app.workspace_dir.clone();
        let session_limit = std::env::var("CODETETHER_SESSION_PICKER_LIMIT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            loop {
                interval.tick().await;
                if let Ok(sessions) = list_sessions_paged(&workspace_dir, session_limit, 0).await
                    && session_tx.send(sessions).await.is_err()
                {
                    break; // TUI closed
                }
            }
        });
    }

    // Check for an interrupted relay checkpoint and notify the user
    if let Some(checkpoint) = RelayCheckpoint::load_for_workspace(&app.workspace_dir).await {
        app.messages.push(ChatMessage::new(
            "system",
            format!(
                "Interrupted relay detected!\nTask: {}\nAgents: {}\nCompleted {} turns, was at round {}, index {}\n\nType /resume to continue the relay from where it left off.",
                truncate_with_ellipsis(&checkpoint.task, 120),
                checkpoint.ordered_agents.join(" → "),
                checkpoint.turns,
                checkpoint.round,
                checkpoint.idx,
            ),
        ));
    }

    loop {
        // --- Periodic background work (non-blocking) ---

        // Apply provider registry preload result (if ready).
        if app.provider_registry.is_none() {
            match provider_rx.try_recv() {
                Ok(Ok(registry)) => {
                    app.provider_registry = Some(std::sync::Arc::new(registry));
                    app.messages.push(ChatMessage::new(
                        "system",
                        "Providers loaded. Ready to chat.",
                    ));
                    app.scroll = SCROLL_BOTTOM;
                }
                Ok(Err(err)) => {
                    app.messages.push(ChatMessage::new(
                        "system",
                        format!("Failed to load providers: {err}"),
                    ));
                    app.scroll = SCROLL_BOTTOM;
                }
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {}
                Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {}
            }
        }

        // Receive session list updates from background task
        if let Ok(sessions) = session_rx.try_recv() {
            app.update_cached_sessions(sessions);
        }

        // Check for theme changes if hot-reload is enabled
        if config.ui.hot_reload && last_check.elapsed() > Duration::from_secs(2) {
            if let Ok(new_config) = Config::load().await
                && (new_config.ui.theme != config.ui.theme
                    || new_config.ui.custom_theme != config.ui.custom_theme)
            {
                theme = crate::tui::theme_utils::validate_theme(&new_config.load_theme());
                config = new_config;
            }
            last_check = Instant::now();
        }

        terminal.draw(|f| ui(f, &mut app, &theme))?;

        // Update max_scroll for scroll key handlers using the actual cached line count.
        // Previously this used `messages.len() * 4` (a rough estimate) which caused
        // scroll positions to be off.  The cache is populated by ui() on each draw,
        // so by the time we get here it reflects the true rendered height.
        let terminal_height = terminal.size()?.height.saturating_sub(6) as usize;
        let actual_lines = app.cached_message_lines.len();
        app.last_max_scroll = actual_lines.saturating_sub(terminal_height);

        // Drain all pending async responses
        if let Some(mut rx) = app.response_rx.take() {
            while let Ok(response) = rx.try_recv() {
                app.handle_response(response);
            }
            app.response_rx = Some(rx);
        }

        // Drain all pending swarm events
        if let Some(mut rx) = app.swarm_rx.take() {
            while let Ok(event) = rx.try_recv() {
                app.handle_swarm_event(event);
            }
            app.swarm_rx = Some(rx);
        }

        // Drain all pending ralph events
        if let Some(mut rx) = app.ralph_rx.take() {
            while let Ok(event) = rx.try_recv() {
                app.handle_ralph_event(event);
            }
            app.ralph_rx = Some(rx);
        }

        // Drain all pending bus log events
        if let Some(mut rx) = app.bus_log_rx.take() {
            while let Ok(env) = rx.try_recv() {
                app.bus_log_state.ingest(&env);
            }
            app.bus_log_rx = Some(rx);
        }

        // Drain all pending spawned-agent responses
        {
            let mut i = 0;
            while i < app.agent_response_rxs.len() {
                let mut done = false;
                while let Ok(event) = app.agent_response_rxs[i].1.try_recv() {
                    if matches!(event, SessionEvent::Done) {
                        done = true;
                    }
                    let name = app.agent_response_rxs[i].0.clone();
                    app.handle_agent_response(&name, event);
                }
                if done {
                    app.agent_response_rxs.swap_remove(i);
                } else {
                    i += 1;
                }
            }
        }

        // Drain all pending background autochat events
        if let Some(mut rx) = app.autochat_rx.take() {
            let mut completed = false;
            while let Ok(event) = rx.try_recv() {
                if app.handle_autochat_event(event) {
                    completed = true;
                }
            }

            if completed || rx.is_closed() {
                if !completed && app.autochat_running {
                    app.messages.push(ChatMessage::new(
                        "system",
                        "Autochat relay worker stopped unexpectedly.",
                    ));
                    app.scroll = SCROLL_BOTTOM;
                }
                app.autochat_running = false;
                app.autochat_started_at = None;
                app.autochat_status = None;
                app.autochat_rx = None;
            } else {
                app.autochat_rx = Some(rx);
            }
        }

        // Drain all pending background chat sync events
        if let Some(mut rx) = app.chat_sync_rx.take() {
            while let Ok(event) = rx.try_recv() {
                app.handle_chat_sync_event(event);
            }

            if rx.is_closed() {
                app.chat_sync_status = Some("Remote archive sync worker stopped.".to_string());
                app.chat_sync_rx = None;
                if app.secure_environment {
                    return Err(anyhow::anyhow!(
                        "Remote archive sync worker stopped in secure environment"
                    ));
                }
            } else {
                app.chat_sync_rx = Some(rx);
            }
        }

        app.maybe_watchdog_main_processing();

        let mut incoming_tasks = Vec::new();
        if let Some(bridge) = app.worker_bridge.as_mut() {
            while let Ok(task) = bridge.task_rx.try_recv() {
                incoming_tasks.push(task);
            }
        }
        for task in incoming_tasks {
            app.messages.push(ChatMessage::new(
                "system",
                format!(
                    "Incoming task {}{}",
                    task.task_id,
                    if task.message.is_empty() {
                        String::new()
                    } else {
                        format!(": {}", truncate_with_ellipsis(&task.message, 180))
                    }
                ),
            ));
            app.scroll = SCROLL_BOTTOM;
            app.worker_task_queue.push_back(task);
        }

        app.sync_worker_bridge_state();

        if app.worker_autorun_enabled
            && !app.is_processing
            && !app.autochat_running
            && app.pending_okr_approval.is_none()
            && app.provider_registry.is_some()
            && let Some(task) = app.worker_task_queue.pop_front()
        {
            let task_text = task.message.trim().to_string();
            if task_text.is_empty() {
                tracing::warn!(task_id = %task.task_id, "Skipping worker task with empty message");
            } else {
                let user = app.worker_policy_user();
                let resource = crate::server::policy::PolicyResource {
                    resource_type: Some("worker_task".to_string()),
                    id: Some(task.task_id.clone()),
                    owner_id: task.from_agent.clone(),
                    tenant_id: user.tenant_id.clone(),
                };
                let allowed =
                    crate::server::policy::check_policy(&user, "agent:execute", Some(&resource))
                        .await;
                if allowed {
                    app.messages.push(ChatMessage::new(
                        "system",
                        format!(
                            "Auto-running worker task {}{}",
                            task.task_id,
                            task.from_agent
                                .as_ref()
                                .map(|agent| format!(" from @{agent}"))
                                .unwrap_or_default()
                        ),
                    ));
                    app.scroll = SCROLL_BOTTOM;
                    app.input = task_text;
                    app.cursor_position = app.input.len();
                    app.submit_message(&config).await;
                } else {
                    app.messages.push(ChatMessage::new(
                        "system",
                        format!("OPA denied auto-run for worker task {}", task.task_id),
                    ));
                    app.scroll = SCROLL_BOTTOM;
                }
            }
        }

        // Persist any newly appended chat messages for durable post-hoc analysis.
        app.flush_chat_archive();

        // Wait for terminal events asynchronously (no blocking!)
        let ev = tokio::select! {
            maybe_event = event_stream.next() => {
                match maybe_event {
                    Some(Ok(ev)) => ev,
                    Some(Err(_)) => continue,
                    None => return Ok(()), // stream ended
                }
            }
            // Tick at 50ms to keep rendering responsive during streaming
            _ = tokio::time::sleep(Duration::from_millis(50)) => continue,
        };

        // Handle bracketed paste: insert entire clipboard text at cursor without submitting
        if let Event::Paste(text) = &ev {
            // Ensure cursor is at a valid char boundary before inserting
            let mut pos = app.cursor_position;
            while pos > 0 && !app.input.is_char_boundary(pos) {
                pos -= 1;
            }
            app.cursor_position = pos;

            for c in text.chars() {
                if c == '\n' || c == '\r' {
                    // Replace newlines with spaces to keep paste as single message
                    app.input.insert(app.cursor_position, ' ');
                } else {
                    app.input.insert(app.cursor_position, c);
                }
                app.cursor_position += c.len_utf8();
            }
            continue;
        }

        if let Event::Key(key) = ev {
            // Only handle key press events (not release or repeat).
            // Crossterm 0.29+ emits Press, Repeat, and Release events;
            // processing all of them causes double character entry.
            if !matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                continue;
            }

            // Help overlay
            if app.show_help {
                if matches!(key.code, KeyCode::Esc | KeyCode::Char('?')) {
                    app.show_help = false;
                }
                continue;
            }

            // Model picker overlay
            if app.view_mode == ViewMode::ModelPicker {
                match key.code {
                    KeyCode::Esc => {
                        app.view_mode = ViewMode::Chat;
                    }
                    KeyCode::Up | KeyCode::Char('k')
                        if !key.modifiers.contains(KeyModifiers::ALT) =>
                    {
                        if app.model_picker_selected > 0 {
                            app.model_picker_selected -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j')
                        if !key.modifiers.contains(KeyModifiers::ALT) =>
                    {
                        let filtered = app.filtered_models();
                        if app.model_picker_selected < filtered.len().saturating_sub(1) {
                            app.model_picker_selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        let filtered = app.filtered_models();
                        if let Some((_, (label, value, _name))) =
                            filtered.get(app.model_picker_selected)
                        {
                            let label = label.clone();
                            let value = value.clone();
                            app.active_model = Some(value.clone());
                            if let Some(session) = app.session.as_mut() {
                                session.metadata.model = Some(value.clone());
                            }
                            app.persist_active_session("model_picker_set").await;
                            app.messages.push(ChatMessage::new(
                                "system",
                                format!("Model set to: {}", label),
                            ));
                            app.view_mode = ViewMode::Chat;
                        }
                    }
                    KeyCode::Backspace => {
                        app.model_picker_filter.pop();
                        app.model_picker_selected = 0;
                    }
                    KeyCode::Char(c)
                        if !key.modifiers.contains(KeyModifiers::CONTROL)
                            && !key.modifiers.contains(KeyModifiers::ALT) =>
                    {
                        app.model_picker_filter.push(c);
                        app.model_picker_selected = 0;
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    _ => {}
                }
                continue;
            }

            // Session picker overlay - handle specially
            if app.view_mode == ViewMode::SessionPicker {
                match key.code {
                    KeyCode::Esc => {
                        if app.session_picker_confirm_delete {
                            app.session_picker_confirm_delete = false;
                        } else {
                            app.session_picker_filter.clear();
                            app.session_picker_offset = 0;
                            app.view_mode = ViewMode::Chat;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.session_picker_selected > 0 {
                            app.session_picker_selected -= 1;
                        }
                        app.session_picker_confirm_delete = false;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        let filtered_count = app.filtered_sessions().len();
                        if app.session_picker_selected < filtered_count.saturating_sub(1) {
                            app.session_picker_selected += 1;
                        }
                        app.session_picker_confirm_delete = false;
                    }
                    KeyCode::Char('d') if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                        if app.session_picker_confirm_delete {
                            // Second press: actually delete
                            let filtered = app.filtered_sessions();
                            if let Some((orig_idx, _)) = filtered.get(app.session_picker_selected) {
                                let session_id = app.session_picker_list[*orig_idx].id.clone();
                                let is_active = app
                                    .session
                                    .as_ref()
                                    .map(|s| s.id == session_id)
                                    .unwrap_or(false);
                                if !is_active {
                                    if let Err(e) = Session::delete(&session_id).await {
                                        app.messages.push(ChatMessage::new(
                                            "system",
                                            format!("Failed to delete session: {}", e),
                                        ));
                                    } else {
                                        app.session_picker_list.retain(|s| s.id != session_id);
                                        if app.session_picker_selected
                                            >= app.session_picker_list.len()
                                        {
                                            app.session_picker_selected =
                                                app.session_picker_list.len().saturating_sub(1);
                                        }
                                    }
                                }
                            }
                            app.session_picker_confirm_delete = false;
                        } else {
                            // First press: ask for confirmation
                            let filtered = app.filtered_sessions();
                            if let Some((orig_idx, _)) = filtered.get(app.session_picker_selected) {
                                let is_active = app
                                    .session
                                    .as_ref()
                                    .map(|s| s.id == app.session_picker_list[*orig_idx].id)
                                    .unwrap_or(false);
                                if !is_active {
                                    app.session_picker_confirm_delete = true;
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        app.session_picker_filter.pop();
                        app.session_picker_selected = 0;
                        app.session_picker_confirm_delete = false;
                    }
                    // Pagination: 'n' = next page, 'p' = previous page
                    KeyCode::Char('n') => {
                        let limit = std::env::var("CODETETHER_SESSION_PICKER_LIMIT")
                            .ok()
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(100);
                        let new_offset = app.session_picker_offset + limit;
                        app.session_picker_offset = new_offset;
                        match list_sessions_paged(&app.workspace_dir, limit, new_offset).await {
                            Ok(sessions) => {
                                app.update_cached_sessions(sessions);
                                app.session_picker_selected = 0;
                            }
                            Err(e) => {
                                app.messages.push(ChatMessage::new(
                                    "system",
                                    format!("Failed to load more sessions: {}", e),
                                ));
                            }
                        }
                    }
                    KeyCode::Char('p') => {
                        if app.session_picker_offset > 0 {
                            let limit = std::env::var("CODETETHER_SESSION_PICKER_LIMIT")
                                .ok()
                                .and_then(|v| v.parse().ok())
                                .unwrap_or(100);
                            let new_offset = app.session_picker_offset.saturating_sub(limit);
                            app.session_picker_offset = new_offset;
                            match list_sessions_paged(&app.workspace_dir, limit, new_offset).await {
                                Ok(sessions) => {
                                    app.update_cached_sessions(sessions);
                                    app.session_picker_selected = 0;
                                }
                                Err(e) => {
                                    app.messages.push(ChatMessage::new(
                                        "system",
                                        format!("Failed to load previous sessions: {}", e),
                                    ));
                                }
                            }
                        }
                    }
                    KeyCode::Char('/') => {
                        // Focus filter (no-op, just signals we're in filter mode)
                    }
                    KeyCode::Enter => {
                        app.session_picker_confirm_delete = false;
                        let filtered = app.filtered_sessions();
                        let session_id = filtered
                            .get(app.session_picker_selected)
                            .map(|(orig_idx, _)| app.session_picker_list[*orig_idx].id.clone());
                        if let Some(session_id) = session_id {
                            let load_result = Session::load(&session_id).await;
                            match load_result {
                                Ok(session) => {
                                    app.messages.clear();
                                    app.messages.push(ChatMessage::new(
                                        "system",
                                        format!(
                                            "Resumed session: {}\nCreated: {}\n{} messages loaded",
                                            session.title.as_deref().unwrap_or("(untitled)"),
                                            session.created_at.format("%Y-%m-%d %H:%M"),
                                            session.messages.len()
                                        ),
                                    ));

                                    for msg in &session.messages {
                                        let role_str = match msg.role {
                                            Role::System => "system",
                                            Role::User => "user",
                                            Role::Assistant => "assistant",
                                            Role::Tool => "tool",
                                        };

                                        // Process each content part separately
                                        for part in &msg.content {
                                            match part {
                                                ContentPart::Text { text } => {
                                                    if !text.is_empty() {
                                                        app.messages.push(ChatMessage::new(
                                                            role_str,
                                                            text.clone(),
                                                        ));
                                                    }
                                                }
                                                ContentPart::Image { url, mime_type } => {
                                                    app.messages.push(
                                                        ChatMessage::new(role_str, "")
                                                            .with_message_type(
                                                                MessageType::Image {
                                                                    url: url.clone(),
                                                                    mime_type: mime_type.clone(),
                                                                },
                                                            ),
                                                    );
                                                }
                                                ContentPart::ToolCall {
                                                    name, arguments, ..
                                                } => {
                                                    let (preview, truncated) =
                                                        build_tool_arguments_preview(
                                                            name,
                                                            arguments,
                                                            TOOL_ARGS_PREVIEW_MAX_LINES,
                                                            TOOL_ARGS_PREVIEW_MAX_BYTES,
                                                        );
                                                    app.messages.push(
                                                        ChatMessage::new(
                                                            role_str,
                                                            format!("🔧 {name}"),
                                                        )
                                                        .with_message_type(MessageType::ToolCall {
                                                            name: name.clone(),
                                                            arguments_preview: preview,
                                                            arguments_len: arguments.len(),
                                                            truncated,
                                                        }),
                                                    );
                                                }
                                                ContentPart::ToolResult { content, .. } => {
                                                    let truncated =
                                                        truncate_with_ellipsis(content, 500);
                                                    let (preview, preview_truncated) =
                                                        build_text_preview(
                                                            content,
                                                            TOOL_OUTPUT_PREVIEW_MAX_LINES,
                                                            TOOL_OUTPUT_PREVIEW_MAX_BYTES,
                                                        );
                                                    app.messages.push(
                                                        ChatMessage::new(
                                                            role_str,
                                                            format!("✅ Result\n{truncated}"),
                                                        )
                                                        .with_message_type(
                                                            MessageType::ToolResult {
                                                                name: "tool".to_string(),
                                                                output_preview: preview,
                                                                output_len: content.len(),
                                                                truncated: preview_truncated,
                                                                success: true,
                                                                duration_ms: None,
                                                            },
                                                        ),
                                                    );
                                                }
                                                ContentPart::File { path, mime_type } => {
                                                    app.messages.push(
                                                        ChatMessage::new(
                                                            role_str,
                                                            format!("📎 {path}"),
                                                        )
                                                        .with_message_type(MessageType::File {
                                                            path: path.clone(),
                                                            mime_type: mime_type.clone(),
                                                        }),
                                                    );
                                                }
                                                ContentPart::Thinking { text } => {
                                                    if !text.is_empty() {
                                                        app.messages.push(
                                                            ChatMessage::new(
                                                                role_str,
                                                                text.clone(),
                                                            )
                                                            .with_message_type(
                                                                MessageType::Thinking(text.clone()),
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    app.current_agent = session.agent.clone();
                                    app.session = Some(session);
                                    app.scroll = SCROLL_BOTTOM;
                                    app.view_mode = ViewMode::Chat;
                                }
                                Err(e) => {
                                    app.messages.push(ChatMessage::new(
                                        "system",
                                        format!("Failed to load session: {}", e),
                                    ));
                                    app.view_mode = ViewMode::Chat;
                                }
                            }
                        }
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char(c)
                        if !key.modifiers.contains(KeyModifiers::CONTROL)
                            && !key.modifiers.contains(KeyModifiers::ALT)
                            && c != 'j'
                            && c != 'k' =>
                    {
                        app.session_picker_filter.push(c);
                        app.session_picker_selected = 0;
                        app.session_picker_confirm_delete = false;
                    }
                    _ => {}
                }
                continue;
            }

            // Agent picker overlay
            if app.view_mode == ViewMode::AgentPicker {
                match key.code {
                    KeyCode::Esc => {
                        app.agent_picker_filter.clear();
                        app.view_mode = ViewMode::Chat;
                    }
                    KeyCode::Up | KeyCode::Char('k')
                        if !key.modifiers.contains(KeyModifiers::ALT) =>
                    {
                        if app.agent_picker_selected > 0 {
                            app.agent_picker_selected -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j')
                        if !key.modifiers.contains(KeyModifiers::ALT) =>
                    {
                        let filtered = app.filtered_spawned_agents();
                        if app.agent_picker_selected < filtered.len().saturating_sub(1) {
                            app.agent_picker_selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        let filtered = app.filtered_spawned_agents();
                        if let Some((name, _, _, _)) = filtered.get(app.agent_picker_selected) {
                            app.active_spawned_agent = Some(name.clone());
                            app.messages.push(ChatMessage::new(
                                "system",
                                format!(
                                    "Focused chat on @{name}. Type messages directly; use /agent main to exit focus."
                                ),
                            ));
                            app.view_mode = ViewMode::Chat;
                        }
                    }
                    KeyCode::Backspace => {
                        app.agent_picker_filter.pop();
                        app.agent_picker_selected = 0;
                    }
                    KeyCode::Char('m') if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.active_spawned_agent = None;
                        app.messages
                            .push(ChatMessage::new("system", "Returned to main chat mode."));
                        app.view_mode = ViewMode::Chat;
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char(c)
                        if !key.modifiers.contains(KeyModifiers::CONTROL)
                            && !key.modifiers.contains(KeyModifiers::ALT)
                            && c != 'j'
                            && c != 'k'
                            && c != 'm' =>
                    {
                        app.agent_picker_filter.push(c);
                        app.agent_picker_selected = 0;
                    }
                    _ => {}
                }
                continue;
            }

            // File picker overlay
            if app.view_mode == ViewMode::FilePicker {
                match key.code {
                    KeyCode::Esc => {
                        app.file_picker_filter.clear();
                        app.view_mode = ViewMode::Chat;
                    }
                    KeyCode::Up | KeyCode::Char('k')
                        if !key.modifiers.contains(KeyModifiers::ALT) =>
                    {
                        if app.file_picker_selected > 0 {
                            app.file_picker_selected -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j')
                        if !key.modifiers.contains(KeyModifiers::ALT) =>
                    {
                        let filtered = app.filtered_file_picker_entries();
                        if app.file_picker_selected < filtered.len().saturating_sub(1) {
                            app.file_picker_selected += 1;
                        }
                    }
                    KeyCode::Home => {
                        app.file_picker_selected = 0;
                    }
                    KeyCode::End => {
                        let filtered = app.filtered_file_picker_entries();
                        app.file_picker_selected = filtered.len().saturating_sub(1);
                    }
                    KeyCode::PageUp => {
                        app.file_picker_selected = app
                            .file_picker_selected
                            .saturating_sub(FILE_PICKER_PAGE_STEP);
                    }
                    KeyCode::PageDown => {
                        let filtered = app.filtered_file_picker_entries();
                        let max_index = filtered.len().saturating_sub(1);
                        app.file_picker_selected =
                            (app.file_picker_selected + FILE_PICKER_PAGE_STEP).min(max_index);
                    }
                    KeyCode::Enter | KeyCode::Right | KeyCode::Char('l')
                        if !key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        app.select_file_picker_entry();
                    }
                    KeyCode::Left | KeyCode::Char('h')
                        if !key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        app.file_picker_go_parent();
                    }
                    KeyCode::Backspace => {
                        if app.file_picker_filter.is_empty() {
                            app.file_picker_go_parent();
                        } else {
                            app.file_picker_filter.pop();
                            app.file_picker_selected = 0;
                        }
                    }
                    KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.file_picker_filter.clear();
                        app.file_picker_selected = 0;
                    }
                    KeyCode::Char('/') => {
                        // no-op: keeps parity with other pickers to signal filter mode
                    }
                    KeyCode::F(5) => {
                        if let Err(err) = app.reload_file_picker_entries() {
                            app.messages.push(ChatMessage::new(
                                "system",
                                format!("Failed to refresh file picker: {}", err),
                            ));
                            app.scroll = SCROLL_BOTTOM;
                        }
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char(c)
                        if !key.modifiers.contains(KeyModifiers::CONTROL)
                            && !key.modifiers.contains(KeyModifiers::ALT)
                            && c != 'j'
                            && c != 'k'
                            && c != 'h'
                            && c != 'l' =>
                    {
                        app.file_picker_filter.push(c);
                        app.file_picker_selected = 0;
                    }
                    _ => {}
                }
                app.refresh_file_picker_preview();
                continue;
            }

            // Swarm view key handling
            if app.view_mode == ViewMode::Swarm {
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Esc => {
                        if app.swarm_state.detail_mode {
                            app.swarm_state.exit_detail();
                        } else {
                            app.view_mode = ViewMode::Chat;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.swarm_state.detail_mode {
                            // In detail mode, Up/Down switch between agents
                            app.swarm_state.exit_detail();
                            app.swarm_state.select_prev();
                            app.swarm_state.enter_detail();
                        } else {
                            app.swarm_state.select_prev();
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.swarm_state.detail_mode {
                            app.swarm_state.exit_detail();
                            app.swarm_state.select_next();
                            app.swarm_state.enter_detail();
                        } else {
                            app.swarm_state.select_next();
                        }
                    }
                    KeyCode::Enter => {
                        if !app.swarm_state.detail_mode {
                            app.swarm_state.enter_detail();
                        }
                    }
                    KeyCode::PageDown => {
                        app.swarm_state.detail_scroll_down(10);
                    }
                    KeyCode::PageUp => {
                        app.swarm_state.detail_scroll_up(10);
                    }
                    KeyCode::Char('?') => {
                        app.show_help = true;
                    }
                    KeyCode::F(2) => {
                        app.view_mode = ViewMode::Chat;
                    }
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.view_mode = ViewMode::Chat;
                    }
                    _ => {}
                }
                continue;
            }

            // Ralph view key handling
            if app.view_mode == ViewMode::Ralph {
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Esc => {
                        if app.ralph_state.detail_mode {
                            app.ralph_state.exit_detail();
                        } else {
                            app.view_mode = ViewMode::Chat;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.ralph_state.detail_mode {
                            app.ralph_state.exit_detail();
                            app.ralph_state.select_prev();
                            app.ralph_state.enter_detail();
                        } else {
                            app.ralph_state.select_prev();
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.ralph_state.detail_mode {
                            app.ralph_state.exit_detail();
                            app.ralph_state.select_next();
                            app.ralph_state.enter_detail();
                        } else {
                            app.ralph_state.select_next();
                        }
                    }
                    KeyCode::Enter => {
                        if !app.ralph_state.detail_mode {
                            app.ralph_state.enter_detail();
                        }
                    }
                    KeyCode::PageDown => {
                        app.ralph_state.detail_scroll_down(10);
                    }
                    KeyCode::PageUp => {
                        app.ralph_state.detail_scroll_up(10);
                    }
                    KeyCode::Char('?') => {
                        app.show_help = true;
                    }
                    KeyCode::F(2) | KeyCode::Char('s')
                        if key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        app.view_mode = ViewMode::Chat;
                    }
                    _ => {}
                }
                continue;
            }

            // Bus log view key handling
            if app.view_mode == ViewMode::BusLog {
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Esc => {
                        if app.bus_log_state.detail_mode {
                            app.bus_log_state.exit_detail();
                        } else {
                            app.view_mode = ViewMode::Chat;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.bus_log_state.detail_mode {
                            app.bus_log_state.exit_detail();
                            app.bus_log_state.select_prev();
                            app.bus_log_state.enter_detail();
                        } else {
                            app.bus_log_state.select_prev();
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.bus_log_state.detail_mode {
                            app.bus_log_state.exit_detail();
                            app.bus_log_state.select_next();
                            app.bus_log_state.enter_detail();
                        } else {
                            app.bus_log_state.select_next();
                        }
                    }
                    KeyCode::Enter => {
                        if !app.bus_log_state.detail_mode {
                            app.bus_log_state.enter_detail();
                        }
                    }
                    KeyCode::PageDown => {
                        app.bus_log_state.detail_scroll_down(10);
                    }
                    KeyCode::PageUp => {
                        app.bus_log_state.detail_scroll_up(10);
                    }
                    // Clear all entries
                    KeyCode::Char('c') => {
                        app.bus_log_state.entries.clear();
                        app.bus_log_state.selected_index = 0;
                    }
                    // Jump to bottom (re-enable auto-scroll)
                    KeyCode::Char('g') => {
                        let len = app.bus_log_state.filtered_entries().len();
                        if len > 0 {
                            app.bus_log_state.selected_index = len - 1;
                            app.bus_log_state.list_state.select(Some(len - 1));
                        }
                        app.bus_log_state.auto_scroll = true;
                    }
                    KeyCode::Char('?') => {
                        app.show_help = true;
                    }
                    _ => {}
                }
                continue;
            }

            // Protocol registry view key handling
            if app.view_mode == ViewMode::Protocol {
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Esc => {
                        app.view_mode = ViewMode::Chat;
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.protocol_selected > 0 {
                            app.protocol_selected -= 1;
                        }
                        app.protocol_scroll = 0;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        let len = app.protocol_cards().len();
                        if app.protocol_selected < len.saturating_sub(1) {
                            app.protocol_selected += 1;
                        }
                        app.protocol_scroll = 0;
                    }
                    KeyCode::PageDown => {
                        app.protocol_scroll = app.protocol_scroll.saturating_add(10);
                    }
                    KeyCode::PageUp => {
                        app.protocol_scroll = app.protocol_scroll.saturating_sub(10);
                    }
                    KeyCode::Char('g') => {
                        app.protocol_scroll = 0;
                    }
                    KeyCode::Char('?') => {
                        app.show_help = true;
                    }
                    _ => {}
                }
                continue;
            }

            match key.code {
                // Quit
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(());
                }
                KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(());
                }

                // Help
                KeyCode::Char('?') => {
                    app.show_help = true;
                }

                // OKR approval gate: 'a' to approve, 'd' to deny
                KeyCode::Char('a')
                    if !key.modifiers.contains(KeyModifiers::CONTROL)
                        && app.pending_okr_approval.is_some() =>
                {
                    if let Some(pending) = app.pending_okr_approval.take() {
                        // Approve: save OKR and run, then execute via Ralph PRD loop
                        app.messages.push(ChatMessage::new(
                            "system",
                            "✅ OKR approved! Starting Ralph PRD execution...",
                        ));
                        app.scroll = SCROLL_BOTTOM;

                        let task = pending.task.clone();
                        let agent_count = pending.agent_count;
                        let config = config.clone();
                        let okr = pending.okr;
                        let mut run = pending.run;

                        // Resolve model for Ralph execution
                        let model = app
                            .active_model
                            .clone()
                            .or_else(|| config.default_model.clone())
                            .unwrap_or_else(|| GO_SWAP_MODEL_MINIMAX.to_string());

                        let bus = app.bus.clone();

                        // Save OKR to repository
                        let okr_id = okr.id;
                        let okr_run_id = run.id;
                        run.record_decision(crate::okr::ApprovalDecision::approve(
                            run.id,
                            "User approved via TUI go command",
                        ));
                        run.correlation_id = Some(format!("ralph-{}", Uuid::new_v4()));

                        let okr_for_save = okr.clone();
                        let run_for_save = run.clone();
                        tokio::spawn(async move {
                            if let Ok(repo) = OkrRepository::from_config().await {
                                let _ = repo.create_okr(okr_for_save).await;
                                let _ = repo.create_run(run_for_save).await;
                                tracing::info!(okr_id = %okr_id, okr_run_id = %okr_run_id, "OKR run approved and saved");
                            }
                        });

                        // Reuse autochat UI channel for Ralph progress reporting
                        let (tx, rx) = mpsc::channel(512);
                        app.autochat_rx = Some(rx);
                        app.autochat_running = true;
                        app.autochat_started_at = Some(Instant::now());
                        app.autochat_status = Some("Generating PRD from task…".to_string());

                        tokio::spawn(async move {
                            run_go_ralph_worker(tx, okr, run, task, model, bus, agent_count).await;
                        });

                        continue;
                    }
                }

                KeyCode::Char('d')
                    if !key.modifiers.contains(KeyModifiers::CONTROL)
                        && app.pending_okr_approval.is_some() =>
                {
                    if let Some(mut pending) = app.pending_okr_approval.take() {
                        // Deny: record decision and show denial message
                        pending.run.record_decision(ApprovalDecision::deny(
                            pending.run.id,
                            "User denied via TUI keypress",
                        ));
                        app.messages.push(ChatMessage::new(
                            "system",
                            "❌ OKR denied. Relay not started.\n\nUse /autochat --no-prd for tactical execution without OKR/PRD tracking.",
                        ));
                        app.scroll = SCROLL_BOTTOM;
                        continue;
                    }
                }

                // Toggle view mode (F2 or Ctrl+S)
                KeyCode::F(2) => {
                    app.view_mode = match app.view_mode {
                        ViewMode::Chat
                        | ViewMode::SessionPicker
                        | ViewMode::ModelPicker
                        | ViewMode::AgentPicker
                        | ViewMode::FilePicker
                        | ViewMode::Protocol
                        | ViewMode::BusLog => ViewMode::Swarm,
                        ViewMode::Swarm | ViewMode::Ralph => ViewMode::Chat,
                    };
                }
                KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.view_mode = match app.view_mode {
                        ViewMode::Chat
                        | ViewMode::SessionPicker
                        | ViewMode::ModelPicker
                        | ViewMode::AgentPicker
                        | ViewMode::FilePicker
                        | ViewMode::Protocol
                        | ViewMode::BusLog => ViewMode::Swarm,
                        ViewMode::Swarm | ViewMode::Ralph => ViewMode::Chat,
                    };
                }

                // Toggle inspector pane in webview layout
                KeyCode::F(3) => {
                    app.show_inspector = !app.show_inspector;
                }

                // Copy latest assistant message to clipboard (Ctrl+Y)
                KeyCode::Char('y') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    let msg = app
                        .messages
                        .iter()
                        .rev()
                        .find(|m| m.role == "assistant" && !m.content.trim().is_empty())
                        .or_else(|| {
                            app.messages
                                .iter()
                                .rev()
                                .find(|m| !m.content.trim().is_empty())
                        });

                    let Some(msg) = msg else {
                        app.messages
                            .push(ChatMessage::new("system", "Nothing to copy yet."));
                        app.scroll = SCROLL_BOTTOM;
                        continue;
                    };

                    let text = message_clipboard_text(msg);
                    match copy_text_to_clipboard_best_effort(&text) {
                        Ok(method) => {
                            app.messages.push(ChatMessage::new(
                                "system",
                                format!("Copied latest reply ({method})."),
                            ));
                            app.scroll = SCROLL_BOTTOM;
                        }
                        Err(err) => {
                            tracing::warn!(error = %err, "Copy to clipboard failed");
                            app.messages.push(ChatMessage::new(
                                "system",
                                "Could not copy to clipboard in this environment.",
                            ));
                            app.scroll = SCROLL_BOTTOM;
                        }
                    }
                }

                // Paste image from clipboard (Ctrl+V)
                KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    // Try to get an image from clipboard
                    if let Some(pending_img) = get_clipboard_image() {
                        let size_kb = pending_img.size_bytes / 1024;
                        let dims = format!("{}x{}", pending_img.width, pending_img.height);
                        app.pending_images.push(pending_img);
                        app.messages.push(ChatMessage::new(
                            "system",
                            format!(
                                "📷 Image attached ({}, ~{}KB). Type a message and press Enter to send with the image.",
                                dims, size_kb
                            ),
                        ));
                        app.scroll = SCROLL_BOTTOM;
                    } else {
                        // No image in clipboard - let the normal paste handling work
                        // (bracketed paste will handle text)
                        app.messages.push(ChatMessage::new(
                            "system",
                            "No image found in clipboard. Text paste uses terminal's native paste.",
                        ));
                        app.scroll = SCROLL_BOTTOM;
                    }
                }

                // Toggle chat layout (Ctrl+B)
                KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.chat_layout = match app.chat_layout {
                        ChatLayoutMode::Classic => ChatLayoutMode::Webview,
                        ChatLayoutMode::Webview => ChatLayoutMode::Classic,
                    };
                }

                // Escape - return to chat from swarm/picker view
                KeyCode::Esc => {
                    if app.view_mode == ViewMode::Swarm
                        || app.view_mode == ViewMode::Ralph
                        || app.view_mode == ViewMode::BusLog
                        || app.view_mode == ViewMode::Protocol
                        || app.view_mode == ViewMode::SessionPicker
                        || app.view_mode == ViewMode::ModelPicker
                        || app.view_mode == ViewMode::AgentPicker
                        || app.view_mode == ViewMode::FilePicker
                    {
                        app.view_mode = ViewMode::Chat;
                    }
                }

                // Model picker (Ctrl+M)
                KeyCode::Char('m') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.open_model_picker(&config).await;
                }

                // File picker (Ctrl+O)
                KeyCode::Char('o') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.open_file_picker();
                }

                // Agent picker (Ctrl+A)
                KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.open_agent_picker();
                }

                // Bus protocol log (Ctrl+L)
                KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.view_mode = ViewMode::BusLog;
                }
                KeyCode::F(4) => {
                    app.view_mode = ViewMode::BusLog;
                }

                // Protocol registry view (Ctrl+P)
                KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.open_protocol_view();
                }

                // Switch agent
                KeyCode::Tab => {
                    app.current_agent = if app.current_agent == "build" {
                        "plan".to_string()
                    } else {
                        "build".to_string()
                    };
                }

                // Submit message
                KeyCode::Enter => {
                    app.submit_message(&config).await;
                }

                // Vim-style scrolling (Alt + j/k)
                KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::ALT) => {
                    // Move down toward older content.
                    if app.scroll >= SCROLL_BOTTOM {
                        if app.last_max_scroll > 0 {
                            app.scroll = 1;
                        }
                    } else {
                        app.scroll = app.scroll.saturating_add(1).min(app.last_max_scroll);
                    }
                }
                KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::ALT) => {
                    // Move up toward newest content.
                    if app.scroll < SCROLL_BOTTOM {
                        let next = app.scroll.saturating_sub(1);
                        app.scroll = if next == 0 { SCROLL_BOTTOM } else { next };
                    } else {
                        // Already pinned to latest.
                    }
                }

                // Command history
                KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.search_history();
                }
                KeyCode::Up if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.navigate_history(-1);
                }
                KeyCode::Down if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.navigate_history(1);
                }

                // Additional Vim-style navigation (with modifiers to avoid conflicts)
                KeyCode::Char('g') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.scroll = 0; // Go to top
                }
                KeyCode::Char('G') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    // Re-enter follow-latest mode.
                    app.scroll = SCROLL_BOTTOM;
                }

                // Enhanced scrolling (with Alt to avoid conflicts)
                KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::ALT) => {
                    // Half page down toward older content.
                    if app.scroll >= SCROLL_BOTTOM {
                        if app.last_max_scroll > 0 {
                            app.scroll = 5.min(app.last_max_scroll);
                        }
                    } else {
                        app.scroll = app.scroll.saturating_add(5).min(app.last_max_scroll);
                    }
                }
                KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::ALT) => {
                    // Half page up toward newest content.
                    if app.scroll < SCROLL_BOTTOM {
                        let next = app.scroll.saturating_sub(5);
                        app.scroll = if next == 0 { SCROLL_BOTTOM } else { next };
                    } else {
                        // Already pinned to latest.
                    }
                }

                // Text input
                KeyCode::Char(c) => {
                    // Ensure cursor is at a valid char boundary
                    while app.cursor_position > 0
                        && !app.input.is_char_boundary(app.cursor_position)
                    {
                        app.cursor_position -= 1;
                    }
                    app.input.insert(app.cursor_position, c);
                    app.cursor_position += c.len_utf8();
                }
                KeyCode::Backspace => {
                    // Move back to previous char boundary
                    while app.cursor_position > 0
                        && !app.input.is_char_boundary(app.cursor_position)
                    {
                        app.cursor_position -= 1;
                    }
                    if app.cursor_position > 0 {
                        // Find start of previous char
                        let prev = app.input[..app.cursor_position].char_indices().next_back();
                        if let Some((idx, ch)) = prev {
                            app.input.replace_range(idx..idx + ch.len_utf8(), "");
                            app.cursor_position = idx;
                        }
                    }
                }
                KeyCode::Delete => {
                    // Ensure cursor is at a valid char boundary
                    while app.cursor_position > 0
                        && !app.input.is_char_boundary(app.cursor_position)
                    {
                        app.cursor_position -= 1;
                    }
                    if app.cursor_position < app.input.len() {
                        let ch = app.input[app.cursor_position..].chars().next();
                        if let Some(ch) = ch {
                            app.input.replace_range(
                                app.cursor_position..app.cursor_position + ch.len_utf8(),
                                "",
                            );
                        }
                    }
                }
                KeyCode::Left => {
                    // Move left by one character (not byte)
                    let prev = app.input[..app.cursor_position].char_indices().next_back();
                    if let Some((idx, _)) = prev {
                        app.cursor_position = idx;
                    }
                }
                KeyCode::Right => {
                    if app.cursor_position < app.input.len() {
                        let ch = app.input[app.cursor_position..].chars().next();
                        if let Some(ch) = ch {
                            app.cursor_position += ch.len_utf8();
                        }
                    }
                }
                KeyCode::Home => {
                    app.cursor_position = 0;
                }
                KeyCode::End => {
                    app.cursor_position = app.input.len();
                }

                // Scroll (normalize first to handle SCROLL_BOTTOM sentinel)
                //
                // Design:
                // - app.scroll == SCROLL_BOTTOM  →  follow-latest mode (latest shown at top)
                // - app.scroll < SCROLL_BOTTOM   →  manual position (0 = top)
                // - last_max_scroll is the max valid manual scroll position from the
                //   previous frame; it's a best-effort hint used only by key handlers.
                KeyCode::Up => {
                    // Move toward newest content.
                    if app.scroll < SCROLL_BOTTOM {
                        let next = app.scroll.saturating_sub(1);
                        app.scroll = if next == 0 { SCROLL_BOTTOM } else { next };
                    } else {
                        // Already pinned to latest.
                    }
                }
                KeyCode::Down => {
                    // Move toward older content.
                    if app.scroll >= SCROLL_BOTTOM {
                        if app.last_max_scroll > 0 {
                            app.scroll = 1;
                        }
                    } else {
                        app.scroll = app.scroll.saturating_add(1).min(app.last_max_scroll);
                    }
                }
                KeyCode::PageUp => {
                    // Page toward newest content.
                    if app.scroll < SCROLL_BOTTOM {
                        let next = app.scroll.saturating_sub(10);
                        app.scroll = if next == 0 { SCROLL_BOTTOM } else { next };
                    } else {
                        // Already pinned to latest.
                    }
                }
                KeyCode::PageDown => {
                    // Page toward older content.
                    if app.scroll >= SCROLL_BOTTOM {
                        if app.last_max_scroll > 0 {
                            app.scroll = 10.min(app.last_max_scroll);
                        }
                    } else {
                        app.scroll = app.scroll.saturating_add(10).min(app.last_max_scroll);
                    }
                }

                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App, theme: &Theme) {
