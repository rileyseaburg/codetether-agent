//! App struct implementation: session management, message handling, tool execution

use super::*;

impl ChatMessage {
    fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        let content = content.into();
        Self {
            role: role.into(),
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
            message_type: MessageType::Text(content.clone()),
            content,
            usage_meta: None,
            agent_name: None,
        }
    }

    fn with_message_type(mut self, message_type: MessageType) -> Self {
        self.message_type = message_type;
        self
    }

    fn with_usage_meta(mut self, meta: UsageMeta) -> Self {
        self.usage_meta = Some(meta);
        self
    }

    fn with_agent_name(mut self, name: impl Into<String>) -> Self {
        self.agent_name = Some(name.into());
        self
    }
}

/// Pending OKR approval gate state for PRD-gated relay commands.

impl App {
    fn new() -> Self {
        let workspace_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let chat_archive_path =
            crate::config::Config::data_dir().map(|dir| dir.join("chat_events.jsonl"));

        Self {
            input: String::new(),
            cursor_position: 0,
            messages: vec![
                ChatMessage::new("system", "Welcome to CodeTether Agent! Press ? for help."),
                ChatMessage::new(
                    "assistant",
                    "Quick start (easy mode):\n• Type a message to chat with the AI\n• /go <task> - OKR+PRD gated relay (requires approval, tracks outcomes)\n• /autochat <task> - PRD-gated relay by default (use --no-prd for tactical)\n• /autochat-local <task> - PRD-gated local relay (use --no-prd for tactical)\n• /local - switch active model to local CUDA\n• /file - open file picker and attach file content to your next message\n• /add <name> - create a helper teammate\n• /talk <name> <message> - message a teammate\n• /list - show teammates\n• /remove <name> - remove teammate\n• /home - return to main chat\n• /help - open help\n• CLI autonomy: codetether forage --top 5\n\nPower user mode: /spawn, /agent, /swarm, /ralph, /protocol",
                ),
            ],
            current_agent: "build".to_string(),
            scroll: 0,
            show_help: false,
            command_history: Vec::new(),
            history_index: None,
            session: None,
            is_processing: false,
            processing_message: None,
            current_tool: None,
            current_tool_started_at: None,
            processing_started_at: None,
            streaming_text: None,
            streaming_agent_texts: HashMap::new(),
            tool_call_count: 0,
            response_rx: None,
            provider_registry: None,
            workspace_dir: workspace_root.clone(),
            view_mode: ViewMode::Chat,
            chat_layout: ChatLayoutMode::Webview,
            show_inspector: true,
            workspace: WorkspaceSnapshot::capture(&workspace_root, 18),
            swarm_state: SwarmViewState::new(),
            swarm_rx: None,
            ralph_state: RalphViewState::new(),
            ralph_rx: None,
            bus_log_state: BusLogState::new(),
            bus_log_rx: None,
            bus: None,
            session_picker_list: Vec::new(),
            session_picker_selected: 0,
            session_picker_filter: String::new(),
            session_picker_confirm_delete: false,
            session_picker_offset: 0,
            model_picker_list: Vec::new(),
            model_picker_selected: 0,
            model_picker_filter: String::new(),
            agent_picker_selected: 0,
            agent_picker_filter: String::new(),
            file_picker_dir: workspace_root.clone(),
            file_picker_entries: Vec::new(),
            file_picker_selected: 0,
            file_picker_filter: String::new(),
            file_picker_preview_title: "No selection".to_string(),
            file_picker_preview_lines: vec![
                "Use ↑/↓ to pick a file.".to_string(),
                "Press Enter to open a folder or attach a file.".to_string(),
            ],
            protocol_selected: 0,
            protocol_scroll: 0,
            active_model: None,
            active_spawned_agent: None,
            spawned_agents: HashMap::new(),
            agent_response_rxs: Vec::new(),
            agent_tool_started_at: HashMap::new(),
            autochat_rx: None,
            autochat_running: false,
            autochat_started_at: None,
            autochat_status: None,
            chat_archive_path,
            archived_message_count: 0,
            chat_sync_rx: None,
            chat_sync_status: None,
            chat_sync_last_success: None,
            chat_sync_last_error: None,
            chat_sync_uploaded_bytes: 0,
            chat_sync_uploaded_batches: 0,
            secure_environment: false,
            pending_okr_approval: None,
            okr_repository: None,
            last_max_scroll: 0,
            cached_message_lines: Vec::new(),
            cached_messages_len: 0,
            cached_max_width: 0,
            cached_streaming_snapshot: None,
            cached_processing: false,
            cached_autochat_running: false,
            pending_images: Vec::new(),
            main_inflight_prompt: None,
            main_watchdog_root_prompt: None,
            main_last_event_at: None,
            main_watchdog_restart_count: 0,
            worker_bridge: None,
            worker_bridge_registered_agents: HashSet::new(),
            worker_bridge_processing_state: None,
            worker_task_queue: VecDeque::new(),
            worker_autorun_enabled: std::env::var("CODETETHER_WORKER_AUTORUN")
                .map(|value| {
                    let v = value.trim().to_ascii_lowercase();
                    matches!(v.as_str(), "1" | "true" | "yes" | "on")
                })
                .unwrap_or(true),
            smart_switch_retry_count: 0,
            smart_switch_attempted_models: HashSet::new(),
            pending_smart_switch_retry: None,
        }
    }

    fn refresh_workspace(&mut self) {
        let workspace_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        self.workspace = WorkspaceSnapshot::capture(&workspace_root, 18);
    }

    fn update_cached_sessions(&mut self, sessions: Vec<SessionSummary>) {
        // Default to 100 sessions, configurable via CODETETHER_SESSION_PICKER_LIMIT env var
        let limit = std::env::var("CODETETHER_SESSION_PICKER_LIMIT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
        self.session_picker_list = sessions.into_iter().take(limit).collect();
        if self.session_picker_selected >= self.session_picker_list.len() {
            self.session_picker_selected = self.session_picker_list.len().saturating_sub(1);
        }
    }

    async fn persist_active_session(&mut self, context: &'static str) {
        if let Some(session) = self.session.as_mut()
            && let Err(err) = session.save().await
        {
            tracing::warn!(
                context,
                error = %err,
                session_id = %session.id,
                "Failed to persist active session"
            );
        }
    }

    fn is_agent_protocol_registered(&self, agent_name: &str) -> bool {
        self.bus
            .as_ref()
            .is_some_and(|bus| bus.registry.get(agent_name).is_some())
    }

    fn protocol_registered_count(&self) -> usize {
        self.bus.as_ref().map_or(0, |bus| bus.registry.len())
    }

    fn bus_status_label_and_color(&self) -> (String, Color) {
        let Some(bus) = &self.bus else {
            return ("BUS off".to_string(), Color::DarkGray);
        };

        let event_count = self.bus_log_state.total_count();
        let agent_count = self.protocol_registered_count();
        let receiver_count = bus.receiver_count();

        if event_count > 0 || agent_count > 0 || receiver_count > 1 {
            (
                format!("BUS active ({event_count}ev/{agent_count}ag/{receiver_count}rx)"),
                Color::Green,
            )
        } else {
            ("BUS idle".to_string(), Color::Yellow)
        }
    }

    fn bus_status_badge_span(&self) -> Span<'static> {
        let (label, color) = self.bus_status_label_and_color();
        Span::styled(
            format!(" {label} "),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )
    }

    fn append_bus_status_with_separator(&self, spans: &mut Vec<Span<'static>>) {
        spans.push(Span::raw(" | "));
        spans.push(self.bus_status_badge_span());
    }

    fn send_worker_bridge_cmd(&self, cmd: WorkerBridgeCmd) -> bool {
        let Some(bridge) = self.worker_bridge.as_ref() else {
            return false;
        };
        if let Err(err) = bridge.cmd_tx.try_send(cmd) {
            tracing::debug!(error = %err, "Failed to send worker bridge command");
            return false;
        }
        true
    }

    fn sync_worker_bridge_agents(&mut self) {
        if self.worker_bridge.is_none() {
            return;
        }

        let current_agents: HashSet<String> = self.spawned_agents.keys().cloned().collect();
        let mut next_registered = self.worker_bridge_registered_agents.clone();

        for name in current_agents
            .difference(&self.worker_bridge_registered_agents)
            .cloned()
        {
            if let Some(agent) = self.spawned_agents.get(&name) {
                if self.send_worker_bridge_cmd(WorkerBridgeCmd::RegisterAgent {
                    name: name.clone(),
                    instructions: agent.instructions.clone(),
                }) {
                    next_registered.insert(name);
                }
            }
        }

        for name in self
            .worker_bridge_registered_agents
            .difference(&current_agents)
            .cloned()
        {
            if self.send_worker_bridge_cmd(WorkerBridgeCmd::DeregisterAgent { name: name.clone() })
            {
                next_registered.remove(&name);
            }
        }

        self.worker_bridge_registered_agents = next_registered;
    }

    fn sync_worker_bridge_processing(&mut self) {
        if self.worker_bridge.is_none() {
            return;
        }

        let processing = self.is_processing
            || self.autochat_running
            || self
                .spawned_agents
                .values()
                .any(|agent| agent.is_processing);

        if self.worker_bridge_processing_state == Some(processing) {
            return;
        }

        self.worker_bridge_processing_state = Some(processing);
        let _ = self.send_worker_bridge_cmd(WorkerBridgeCmd::SetProcessing(processing));
    }

    fn sync_worker_bridge_state(&mut self) {
        self.sync_worker_bridge_agents();
        self.sync_worker_bridge_processing();
    }

    fn reset_smart_switch_state(&mut self) {
        self.smart_switch_retry_count = 0;
        self.smart_switch_attempted_models.clear();
        self.pending_smart_switch_retry = None;
    }

    fn smart_switch_max_retries() -> u8 {
        std::env::var("CODETETHER_SMART_SWITCH_MAX_RETRIES")
            .ok()
            .and_then(|v| v.parse::<u8>().ok())
            .map(|v| v.clamp(1, 10))
            .unwrap_or(SMART_SWITCH_MAX_RETRIES)
    }

    fn current_main_model_ref(&self) -> Option<String> {
        self.session
            .as_ref()
            .and_then(|session| session.metadata.model.clone())
            .or_else(|| self.active_model.clone())
    }

    fn smart_switch_candidates(&self, current_model: Option<&str>) -> Vec<String> {
        let Some(registry) = self.provider_registry.as_ref() else {
            return Vec::new();
        };

        let available: HashSet<String> = registry
            .list()
            .into_iter()
            .map(normalize_provider_alias)
            .map(ToString::to_string)
            .collect();

        let current_provider = current_model
            .and_then(|model_ref| crate::provider::parse_model_string(model_ref).0)
            .map(normalize_provider_alias)
            .map(ToString::to_string);

        let mut candidates = Vec::new();

        if let Some(provider_name) = current_provider.as_deref()
            && available.contains(provider_name)
        {
            for model_id in smart_switch_preferred_models(provider_name) {
                candidates.push(format!("{provider_name}/{model_id}"));
            }
        }

        for provider_name in SMART_SWITCH_PROVIDER_PRIORITY {
            let normalized_provider = normalize_provider_alias(provider_name);
            if Some(normalized_provider) == current_provider.as_deref() {
                continue;
            }
            if !available.contains(normalized_provider) {
                continue;
            }
            if let Some(model_id) = smart_switch_preferred_models(normalized_provider).first() {
                candidates.push(format!("{normalized_provider}/{model_id}"));
            }
        }

        let mut extra_providers: Vec<String> = available
            .into_iter()
            .filter(|provider_name| {
                if Some(provider_name.as_str()) == current_provider.as_deref() {
                    return false;
                }
                !SMART_SWITCH_PROVIDER_PRIORITY
                    .iter()
                    .any(|priority| normalize_provider_alias(priority) == provider_name)
            })
            .collect();
        extra_providers.sort();

        for provider_name in extra_providers {
            if let Some(model_id) = smart_switch_preferred_models(&provider_name).first() {
                candidates.push(format!("{provider_name}/{model_id}"));
            }
        }

        candidates
    }

    fn maybe_schedule_smart_switch_retry(&mut self, err: &str) -> bool {
        if self.pending_smart_switch_retry.is_some() || !is_retryable_provider_error(err) {
            return false;
        }

        let max_retries = Self::smart_switch_max_retries();
        if self.smart_switch_retry_count >= max_retries {
            return false;
        }

        let base_prompt = self
            .main_watchdog_root_prompt
            .clone()
            .or_else(|| self.main_inflight_prompt.clone());
        let Some(prompt) = base_prompt else {
            return false;
        };

        if let Some(current_model) = self.current_main_model_ref() {
            self.smart_switch_attempted_models
                .insert(smart_switch_model_key(&current_model));
        }

        let target_model = self
            .smart_switch_candidates(self.current_main_model_ref().as_deref())
            .into_iter()
            .find(|candidate| {
                !self
                    .smart_switch_attempted_models
                    .contains(&smart_switch_model_key(candidate))
            });

        let Some(target_model) = target_model else {
            return false;
        };

        self.smart_switch_retry_count = self.smart_switch_retry_count.saturating_add(1);
        self.smart_switch_attempted_models
            .insert(smart_switch_model_key(&target_model));
        self.pending_smart_switch_retry = Some(PendingSmartSwitchRetry {
            prompt,
            target_model: target_model.clone(),
        });

        self.messages.push(ChatMessage::new(
            "system",
            format!(
                "Smart switcher: transient provider failure detected. \
Retrying with {target_model} (attempt {}/{}).",
                self.smart_switch_retry_count, max_retries
            ),
        ));
        self.scroll = SCROLL_BOTTOM;
        true
    }

    fn apply_pending_smart_switch_retry(&mut self) -> bool {
        let Some(retry) = self.pending_smart_switch_retry.take() else {
            return false;
        };

        self.response_rx = None;
        self.streaming_text = None;
        self.current_tool = None;
        self.current_tool_started_at = None;
        self.processing_started_at = Some(Instant::now());
        self.processing_message = Some(format!("Retrying via {}...", retry.target_model));
        self.main_last_event_at = Some(Instant::now());

        self.active_model = Some(retry.target_model.clone());
        if let Some(session) = self.session.as_mut() {
            session.metadata.model = Some(retry.target_model.clone());
        }
        if self.main_watchdog_root_prompt.is_none() {
            self.main_watchdog_root_prompt = Some(retry.prompt.clone());
        }

        self.spawn_main_processing_task(retry.prompt, Vec::new());
        true
    }

    fn reset_main_processing_state(&mut self) {
        self.is_processing = false;
        self.processing_message = None;
        self.current_tool = None;
        self.current_tool_started_at = None;
        self.processing_started_at = None;
        self.streaming_text = None;
        self.response_rx = None;
        self.main_inflight_prompt = None;
        self.main_watchdog_root_prompt = None;
        self.main_last_event_at = None;
        self.main_watchdog_restart_count = 0;
        self.reset_smart_switch_state();
    }

    fn main_watchdog_timeout_secs() -> u64 {
        std::env::var("CODETETHER_MAIN_WATCHDOG_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .map(|v| v.clamp(60, 900))
            .unwrap_or(MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS)
    }

    fn build_main_watchdog_recovery_prompt(base_prompt: &str, attempt: u8) -> String {
        format!(
            "Watchdog recovery attempt {attempt}: previous run stalled. \
Resume and complete the user request. If blocked, self-delegate to available helper agents and continue.\n\n\
Original request:\n{base_prompt}"
        )
    }

    fn spawn_main_processing_task(
        &mut self,
        message: String,
        pending_images: Vec<crate::session::ImageAttachment>,
    ) {
        let Some(registry) = self.provider_registry.clone() else {
            self.messages.push(ChatMessage::new(
                "system",
                "Providers are still loading; cannot send message yet.",
            ));
            self.scroll = SCROLL_BOTTOM;
            self.reset_main_processing_state();
            return;
        };

        let Some(session_clone) = self.session.clone() else {
            self.messages.push(ChatMessage::new(
                "assistant",
                "Error: session not initialized",
            ));
            self.scroll = SCROLL_BOTTOM;
            self.reset_main_processing_state();
            return;
        };

        self.is_processing = true;
        self.processing_message = Some("Thinking...".to_string());
        self.current_tool = None;
        self.current_tool_started_at = None;
        self.processing_started_at = Some(Instant::now());
        self.streaming_text = None;
        self.main_inflight_prompt = Some(message.clone());
        self.main_last_event_at = Some(Instant::now());

        let (tx, rx) = mpsc::channel(100);
        self.response_rx = Some(rx);

        tokio::spawn(async move {
            let mut session = session_clone;
            let result = if pending_images.is_empty() {
                session
                    .prompt_with_events(&message, tx.clone(), registry)
                    .await
            } else {
                session
                    .prompt_with_events_and_images(&message, pending_images, tx.clone(), registry)
                    .await
            };

            if let Err(err) = result {
                tracing::error!(error = %err, "Agent processing failed");
                session.add_message(crate::provider::Message {
                    role: crate::provider::Role::Assistant,
                    content: vec![crate::provider::ContentPart::Text {
                        text: format!("Error: {err:#}"),
                    }],
                });
                if let Err(save_err) = session.save().await {
                    tracing::warn!(
                        error = %save_err,
                        session_id = %session.id,
                        "Failed to save session after processing error"
                    );
                }
                let _ = tx.send(SessionEvent::SessionSync(Box::new(session))).await;
                let _ = tx.send(SessionEvent::Error(format!("{err:#}"))).await;
                let _ = tx.send(SessionEvent::Done).await;
            }
        });
    }

    fn maybe_watchdog_main_processing(&mut self) {
        if !self.is_processing {
            return;
        }

        let timeout_secs = Self::main_watchdog_timeout_secs();
        let channel_closed = self
            .response_rx
            .as_ref()
            .is_none_or(mpsc::Receiver::is_closed);
        let stalled_by_time = self
            .main_last_event_at
            .map(|t| t.elapsed() >= Duration::from_secs(timeout_secs))
            .unwrap_or(false);

        if !channel_closed && !stalled_by_time {
            return;
        }

        let reason = if channel_closed {
            "response channel closed unexpectedly"
        } else {
            "no progress events received"
        };

        let base_prompt = self
            .main_watchdog_root_prompt
            .clone()
            .or_else(|| self.main_inflight_prompt.clone());

        // Never give up - always retry with recovery prompt
        if let Some(base_prompt) = base_prompt {
            self.main_watchdog_restart_count = self.main_watchdog_restart_count.saturating_add(1);
            let attempt = self.main_watchdog_restart_count;
            let recovery_prompt = Self::build_main_watchdog_recovery_prompt(&base_prompt, attempt);
            self.messages.push(ChatMessage::new(
                "system",
                format!("Watchdog: main request stalled ({reason}); restarting attempt {attempt}."),
            ));
            self.scroll = SCROLL_BOTTOM;
            self.current_tool = None;
            self.current_tool_started_at = None;
            self.processing_message = Some("Recovering stalled request...".to_string());
            self.streaming_text = None;
            // Drop old receiver so stale worker updates do not continue mutating UI state.
            self.response_rx = None;
            self.main_last_event_at = Some(Instant::now());

            if attempt >= 2 {
                self.watchdog_nudge_helper_agent(&base_prompt, attempt);
            }

            self.spawn_main_processing_task(recovery_prompt, Vec::new());
        }
    }

    fn watchdog_nudge_helper_agent(&mut self, base_prompt: &str, attempt: u8) {
        let helper_name = self
            .spawned_agents
            .iter()
            .find_map(|(name, agent)| (!agent.is_processing).then(|| name.clone()));

        let Some(helper_name) = helper_name else {
            return;
        };

        let helper_prompt = format!(
            "Watchdog assist request (attempt {attempt}). The main agent stalled while handling:\n\
{base_prompt}\n\n\
Please take one concrete step to unblock progress and report results."
        );

        self.messages.push(ChatMessage::new(
            "system",
            format!(
                "Watchdog: asking @{helper_name} to assist recovery for the stalled main request."
            ),
        ));
        self.scroll = SCROLL_BOTTOM;
        self.dispatch_to_agent_internal(&helper_name, &helper_prompt);
    }

    fn protocol_cards(&self) -> Vec<crate::a2a::types::AgentCard> {
        let Some(bus) = &self.bus else {
            return Vec::new();
        };

        let mut ids = bus.registry.agent_ids();
        ids.sort_by_key(|id| id.to_lowercase());

        ids.into_iter()
            .filter_map(|id| bus.registry.get(&id))
            .collect()
    }

    fn open_protocol_view(&mut self) {
        self.protocol_selected = 0;
        self.protocol_scroll = 0;
        self.view_mode = ViewMode::Protocol;
    }

    fn unique_spawned_name(&self, base: &str) -> String {
        if !self.spawned_agents.contains_key(base) {
            return base.to_string();
        }

        let mut suffix = 2usize;
        loop {
            let candidate = format!("{base}-{suffix}");
            if !self.spawned_agents.contains_key(&candidate) {
                return candidate;
            }
            suffix += 1;
        }
    }

    fn build_autochat_profiles(&self, count: usize) -> Vec<(String, String, Vec<String>)> {
        let mut profiles = Vec::with_capacity(count);
        for idx in 0..count {
            let base = format!("auto-agent-{}", idx + 1);
            let name = self.unique_spawned_name(&base);
            let instructions = format!(
                "You are @{name}.\n\
                 Role policy: self-organize from task context and current handoff instead of assuming a fixed persona.\n\
                 Mission: advance the relay with concrete, high-signal next actions and clear ownership boundaries.\n\n\
                 This is a protocol-first relay conversation. Treat the incoming handoff as the authoritative context.\n\
                 Keep your response concise, concrete, and useful for the next specialist.\n\
                 Include one clear recommendation for what the next agent should do.\n\
                 If the task scope is too large, explicitly call out missing specialties and handoff boundaries.",
            );
            let capabilities = vec![
                "generalist".to_string(),
                "self-organizing".to_string(),
                "relay".to_string(),
                "context-handoff".to_string(),
                "rlm-aware".to_string(),
                "autochat".to_string(),
            ];

            profiles.push((name, instructions, capabilities));
        }

        profiles
    }
    async fn start_autochat_execution(
        &mut self,
        agent_count: usize,
        task: String,
        config: &Config,
        okr_id: Option<Uuid>,
        okr_run_id: Option<Uuid>,
        model_override: Option<String>,
    ) {
        if !(2..=AUTOCHAT_MAX_AGENTS).contains(&agent_count) {
            self.messages.push(ChatMessage::new(
                "system",
                format!(
                    "Usage: /autochat <count> <task>\ncount must be between 2 and {AUTOCHAT_MAX_AGENTS}."
                ),
            ));
            return;
        }

        if self.autochat_running {
            self.messages.push(ChatMessage::new(
                "system",
                "Autochat relay already running. Wait for it to finish before starting another.",
            ));
            return;
        }

        let model_ref = model_override
            .or_else(|| self.active_model.clone())
            .or_else(|| config.default_model.clone())
            .unwrap_or_else(|| "zai/glm-5".to_string());

        let status_msg = if okr_id.is_some() {
            format!(
                "Preparing OKR-gated relay with {agent_count} agents…\nModel: {model_ref}\nTask: {}\n(Approval-granted execution)",
                truncate_with_ellipsis(&task, 160),
            )
        } else {
            format!(
                "Preparing relay with {agent_count} agents…\nModel: {model_ref}\nTask: {}\n(Compact mode: live agent streaming here, detailed relay envelopes in /buslog)",
                truncate_with_ellipsis(&task, 180),
            )
        };

        self.messages.push(ChatMessage::new(
            "user",
            format!("/autochat {agent_count} {task}"),
        ));
        self.messages.push(ChatMessage::new("system", status_msg));
        self.scroll = SCROLL_BOTTOM;

        let Some(bus) = self.bus.clone() else {
            self.messages.push(ChatMessage::new(
                "system",
                "Protocol bus unavailable; cannot start /autochat relay.",
            ));
            return;
        };

        let profiles = self.build_autochat_profiles(agent_count);
        if profiles.is_empty() {
            self.messages.push(ChatMessage::new(
                "system",
                "No relay profiles could be created.",
            ));
            return;
        }

        let (tx, rx) = mpsc::channel(512);
        self.autochat_rx = Some(rx);
        self.autochat_running = true;
        self.autochat_started_at = Some(Instant::now());
        self.autochat_status = Some("Preparing relay…".to_string());
        self.active_spawned_agent = None;

        tokio::spawn(async move {
            run_autochat_worker(tx, bus, profiles, task, model_ref, okr_id, okr_run_id).await;
        });
    }

    /// Resume an interrupted autochat relay from a checkpoint.
    async fn resume_autochat_relay(&mut self, checkpoint: RelayCheckpoint) {
        if self.autochat_running {
            self.messages.push(ChatMessage::new(
                "system",
                "Autochat relay already running. Wait for it to finish before resuming.",
            ));
            return;
        }

        let Some(bus) = self.bus.clone() else {
            self.messages.push(ChatMessage::new(
                "system",
                "Protocol bus unavailable; cannot resume relay.",
            ));
            return;
        };

        self.messages.push(ChatMessage::new(
            "system",
            format!(
                "Resuming interrupted relay…\nTask: {}\nAgents: {}\nResuming from round {}, agent index {}\nTurns completed: {}",
                truncate_with_ellipsis(&checkpoint.task, 120),
                checkpoint.ordered_agents.join(" → "),
                checkpoint.round,
                checkpoint.idx,
                checkpoint.turns,
            ),
        ));
        self.scroll = SCROLL_BOTTOM;

        let (tx, rx) = mpsc::channel(512);
        self.autochat_rx = Some(rx);
        self.autochat_running = true;
        self.autochat_started_at = Some(Instant::now());
        self.autochat_status = Some("Resuming relay…".to_string());
        self.active_spawned_agent = None;

        tokio::spawn(async move {
            resume_autochat_worker(tx, bus, checkpoint).await;
        });
    }

    async fn submit_message(&mut self, config: &Config) {
        if self.input.is_empty() {
            return;
        }

        let mut message = std::mem::take(&mut self.input);
        let easy_go_requested = is_easy_go_command(&message);
        self.cursor_position = 0;

        // Check for pending OKR approval gate response FIRST
        // This must be before any command normalization
        if let Some(pending) = self.pending_okr_approval.take() {
            let response = message.trim().to_lowercase();
            let approved = matches!(
                response.as_str(),
                "a" | "approve" | "y" | "yes" | "A" | "Approve" | "Y" | "Yes"
            );
            let denied = matches!(
                response.as_str(),
                "d" | "deny" | "n" | "no" | "D" | "Deny" | "N" | "No"
            );

            if approved {
                // User approved - save OKR and run, then execute
                let okr_id = pending.okr.id;
                let run_id = pending.run.id;
                let task = pending.task.clone();
                let agent_count = pending.agent_count;
                let _model = pending.model.clone();

                // Update run status to approved
                let mut approved_run = pending.run;
                approved_run.record_decision(ApprovalDecision::approve(
                    approved_run.id,
                    "User approved via TUI",
                ));

                // Save to repository if available
                if let Some(ref repo) = self.okr_repository {
                    let repo = std::sync::Arc::clone(repo);
                    let okr_to_save = pending.okr;
                    let run_to_save = approved_run;
                    tokio::spawn(async move {
                        if let Err(e) = repo.create_okr(okr_to_save).await {
                            tracing::error!(error = %e, "Failed to save approved OKR");
                        }
                        if let Err(e) = repo.create_run(run_to_save).await {
                            tracing::error!(error = %e, "Failed to save approved OKR run");
                        }
                    });
                }

                self.messages.push(ChatMessage::new(
                    "system",
                    format!(
                        "✅ OKR approved. Starting OKR-gated relay (ID: {})...",
                        okr_id
                    ),
                ));
                self.scroll = SCROLL_BOTTOM;

                // Start execution with OKR IDs
                self.start_autochat_execution(
                    agent_count,
                    task,
                    config,
                    Some(okr_id),
                    Some(run_id),
                    None,
                )
                .await;
                return;
            } else if denied {
                // User denied - record decision and cancel
                let mut denied_run = pending.run;
                denied_run
                    .record_decision(ApprovalDecision::deny(denied_run.id, "User denied via TUI"));
                self.messages.push(ChatMessage::new(
                    "system",
                    "❌ OKR denied. Relay cancelled.",
                ));
                self.scroll = SCROLL_BOTTOM;
                return;
            } else {
                // Invalid response - re-prompt
                self.messages.push(ChatMessage::new(
                    "system",
                    format!(
                        "Invalid response. {}\n\nPress [A] to approve or [D] to deny.",
                        pending.approval_prompt()
                    ),
                ));
                self.scroll = SCROLL_BOTTOM;
                // Put the pending approval back
                self.pending_okr_approval = Some(pending);
                // Put the input back so user can try again
                self.input = message;
                return;
            }
        }

        // Save to command history
        if !message.trim().is_empty() {
            self.command_history.push(message.clone());
            self.history_index = None;
        }

        // Easy-mode slash aliases (/go, /add, /talk, /list, ...)
        message = normalize_easy_command(&message);
        self.sync_spawned_agents_from_tool_store();

        if message.trim() == "/help" {
            self.show_help = true;
            return;
        }

        // Backward-compatible /agent command aliases
        if message.trim().starts_with("/agent") {
            let rest = message.trim().strip_prefix("/agent").unwrap_or("").trim();

            if rest.is_empty() {
                self.open_agent_picker();
                return;
            }

            if rest == "pick" || rest == "picker" || rest == "select" {
                self.open_agent_picker();
                return;
            }

            if rest == "main" || rest == "off" {
                if let Some(target) = self.active_spawned_agent.take() {
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!("Exited focused sub-agent chat (@{target})."),
                    ));
                } else {
                    self.messages
                        .push(ChatMessage::new("system", "Already in main chat mode."));
                }
                return;
            }

            if rest == "build" || rest == "plan" {
                self.current_agent = rest.to_string();
                self.active_spawned_agent = None;
                self.messages.push(ChatMessage::new(
                    "system",
                    format!("Switched main agent to '{rest}'. (Tab also works.)"),
                ));
                return;
            }

            if rest == "list" || rest == "ls" {
                message = "/agents".to_string();
            } else if let Some(args) = rest
                .strip_prefix("spawn ")
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                message = format!("/spawn {args}");
            } else if let Some(name) = rest
                .strip_prefix("kill ")
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                message = format!("/kill {name}");
            } else if !rest.contains(' ') {
                let target = rest.trim_start_matches('@');
                if self.spawned_agents.contains_key(target) {
                    self.active_spawned_agent = Some(target.to_string());
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!(
                            "Focused chat on @{target}. Type messages directly; use /agent main to exit focus."
                        ),
                    ));
                } else {
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!(
                            "No agent named @{target}. Use /agents to list, or /spawn <name> <instructions> to create one."
                        ),
                    ));
                }
                return;
            } else if let Some((name, content)) = rest.split_once(' ') {
                let target = name.trim().trim_start_matches('@');
                let content = content.trim();
                if target.is_empty() || content.is_empty() {
                    self.messages
                        .push(ChatMessage::new("system", "Usage: /agent <name> <message>"));
                    return;
                }
                message = format!("@{target} {content}");
            } else {
                self.messages.push(ChatMessage::new(
                    "system",
                    "Unknown /agent usage. Try /agent, /agent <name>, /agent <name> <message>, or /agent list.",
                ));
                return;
            }
        }

        if let Some(rest) = command_with_optional_args(&message, "/autochat-local") {
            let Some(parsed) = crate::autochat::parse_autochat_request(
                rest,
                AUTOCHAT_DEFAULT_AGENTS,
                AUTOCHAT_QUICK_DEMO_TASK,
            ) else {
                self.messages.push(ChatMessage::new(
                    "system",
                    format!(
                        "Usage: /autochat-local [count] [--no-prd] <task>\nExamples:\n  /autochat-local implement protocol-first relay with tests\n  /autochat-local 4 implement protocol-first relay with tests\ncount range: 2-{} (default: {})",
                        AUTOCHAT_MAX_AGENTS,
                        AUTOCHAT_DEFAULT_AGENTS,
                    ),
                ));
                return;
            };

            let count = parsed.agent_count;
            let task = parsed.task;
            let current_model = self
                .active_model
                .as_deref()
                .or(config.default_model.as_deref());
            let local_model = resolve_local_loop_model(current_model);
            let require_prd = !parsed.bypass_prd;
            if require_prd {
                let pending = PendingOkrApproval::propose(task, count, local_model).await;
                self.messages
                    .push(ChatMessage::new("system", pending.approval_prompt()));
                self.scroll = SCROLL_BOTTOM;
                self.pending_okr_approval = Some(pending);
            } else {
                self.start_autochat_execution(count, task, config, None, None, Some(local_model))
                    .await;
            }
            return;
        }

        // Check for /autochat command
        if let Some(rest) = command_with_optional_args(&message, "/autochat") {
            let Some(parsed) = crate::autochat::parse_autochat_request(
                rest,
                AUTOCHAT_DEFAULT_AGENTS,
                AUTOCHAT_QUICK_DEMO_TASK,
            ) else {
                self.messages.push(ChatMessage::new(
                    "system",
                    format!(
                        "Usage: /autochat [count] [--no-prd] <task>\nEasy mode: /go <task>\nExamples:\n  /autochat implement protocol-first relay with tests\n  /autochat 4 implement protocol-first relay with tests\ncount range: 2-{} (default: {})",
                        AUTOCHAT_MAX_AGENTS,
                        AUTOCHAT_DEFAULT_AGENTS,
                    ),
                ));
                return;
            };
            let count = parsed.agent_count;
            let task = parsed.task;
            let require_prd = easy_go_requested || !parsed.bypass_prd;

            if require_prd {
                let current_model = self
                    .active_model
                    .as_deref()
                    .or(config.default_model.as_deref());
                let model = if easy_go_requested {
                    let next_model = next_go_model(current_model);
                    self.active_model = Some(next_model.clone());
                    if let Some(session) = self.session.as_mut() {
                        session.metadata.model = Some(next_model.clone());
                    }
                    self.persist_active_session("go_model_swap").await;
                    next_model
                } else {
                    current_model
                        .filter(|value| !value.trim().is_empty())
                        .map(str::to_string)
                        .unwrap_or_else(|| GO_SWAP_MODEL_MINIMAX.to_string())
                };

                // Initialize OKR repository if not already done
                if self.okr_repository.is_none()
                    && let Ok(repo) = OkrRepository::from_config().await
                {
                    self.okr_repository = Some(std::sync::Arc::new(repo));
                }

                // Create pending OKR approval gate.
                // For /go, default to max concurrency unless an explicit count is provided.
                let go_count = if easy_go_requested && !parsed.explicit_count {
                    AUTOCHAT_MAX_AGENTS
                } else {
                    count
                };
                let pending = PendingOkrApproval::propose(task.to_string(), go_count, model).await;

                self.messages
                    .push(ChatMessage::new("system", pending.approval_prompt()));
                self.scroll = SCROLL_BOTTOM;

                // Store pending approval and wait for user input
                self.pending_okr_approval = Some(pending);
                return;
            }

            self.start_autochat_execution(count, task, config, None, None, None)
                .await;
            return;
        }

        // Check for /swarm command
        if let Some(task) = command_with_optional_args(&message, "/swarm") {
            if task.is_empty() {
                self.messages.push(ChatMessage::new(
                    "system",
                    "Usage: /swarm <task description>",
                ));
                return;
            }
            self.start_swarm_execution(task.to_string(), config).await;
            return;
        }

        // Check for /ralph command
        if message.trim().starts_with("/ralph") {
            let prd_path = message
                .trim()
                .strip_prefix("/ralph")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .unwrap_or("prd.json")
                .to_string();
            self.start_ralph_execution(prd_path, config).await;
            return;
        }

        if message.trim() == "/webview" {
            self.chat_layout = ChatLayoutMode::Webview;
            self.messages.push(ChatMessage::new(
                "system",
                "Switched to webview layout. Use /classic to return to single-pane chat.",
            ));
            return;
        }

        if message.trim() == "/classic" {
            self.chat_layout = ChatLayoutMode::Classic;
            self.messages.push(ChatMessage::new(
                "system",
                "Switched to classic layout. Use /webview for dashboard-style panes.",
            ));
            return;
        }

        if message.trim() == "/inspector" {
            self.show_inspector = !self.show_inspector;
            let state = if self.show_inspector {
                "enabled"
            } else {
                "disabled"
            };
            self.messages.push(ChatMessage::new(
                "system",
                format!("Inspector pane {}. Press F3 to toggle quickly.", state),
            ));
            return;
        }

        if message.trim() == "/refresh" {
            self.refresh_workspace();
            let limit = std::env::var("CODETETHER_SESSION_PICKER_LIMIT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100);
            // Reset offset on refresh
            self.session_picker_offset = 0;
            match list_sessions_paged(&self.workspace_dir, limit, 0).await {
                Ok(sessions) => self.update_cached_sessions(sessions),
                Err(err) => self.messages.push(ChatMessage::new(
                    "system",
                    format!(
                        "Workspace refreshed, but failed to refresh sessions: {}",
                        err
                    ),
                )),
            }
            self.messages.push(ChatMessage::new(
                "system",
                "Workspace and session cache refreshed.",
            ));
            return;
        }

        // /file command: open picker or attach a specific file path to the composer
        if let Some(rest) = command_with_optional_args(&message, "/file") {
            let cleaned = rest.trim().trim_matches(|c| c == '"' || c == '\'');
            if cleaned.is_empty() {
                self.open_file_picker();
            } else {
                self.attach_file_to_input(Path::new(cleaned));
            }
            return;
        }

        if message.trim() == "/archive" {
            let details = if let Some(path) = &self.chat_archive_path {
                format!(
                    "Chat archive: {}\nCaptured records in this run: {}\n{}",
                    path.display(),
                    self.archived_message_count,
                    self.chat_sync_summary(),
                )
            } else {
                format!(
                    "Chat archive path unavailable in this environment.\n{}",
                    self.chat_sync_summary()
                )
            };
            self.messages.push(ChatMessage::new("system", details));
            self.scroll = SCROLL_BOTTOM;
            return;
        }

        // Check for /view command to toggle views
        if message.trim() == "/view" {
            self.view_mode = match self.view_mode {
                ViewMode::Chat
                | ViewMode::SessionPicker
                | ViewMode::ModelPicker
                | ViewMode::AgentPicker
                | ViewMode::FilePicker
                | ViewMode::BusLog
                | ViewMode::Protocol => ViewMode::Swarm,
                ViewMode::Swarm | ViewMode::Ralph => ViewMode::Chat,
            };
            return;
        }

        // Check for /buslog command to open protocol bus log
        if message.trim() == "/buslog" || message.trim() == "/bus" {
            self.view_mode = ViewMode::BusLog;
            return;
        }

        // Check for /protocol command to inspect registered AgentCards
        if message.trim() == "/protocol" || message.trim() == "/registry" {
            self.open_protocol_view();
            return;
        }

        // Check for /spawn command - create a named sub-agent
        if let Some(rest) = command_with_optional_args(&message, "/spawn") {
            let default_instructions = |agent_name: &str| {
                let profile = agent_profile(agent_name);
                format!(
                    "You are @{agent_name}, codename {codename}.\n\
                     Profile: {profile_line}.\n\
                     Personality: {personality}.\n\
                     Collaboration style: {style}.\n\
                     Signature move: {signature}.\n\
                     Be a helpful teammate: explain in simple words, short steps, and a friendly tone.",
                    codename = profile.codename,
                    profile_line = profile.profile,
                    personality = profile.personality,
                    style = profile.collaboration_style,
                    signature = profile.signature_move,
                )
            };

            let (name, instructions, used_default_instructions) = if rest.is_empty() {
                self.messages.push(ChatMessage::new(
                    "system",
                    "Usage: /spawn <name> [instructions]\nEasy mode: /add <name>\nExample: /spawn planner You are a planning agent. Break tasks into steps.",
                ));
                return;
            } else {
                let mut parts = rest.splitn(2, char::is_whitespace);
                let raw_name = parts.next().unwrap_or("").trim();
                if raw_name.is_empty() {
                    self.messages.push(ChatMessage::new(
                        "system",
                        "Usage: /spawn <name> [instructions]\nEasy mode: /add <name>",
                    ));
                    return;
                }
                let name = sanitize_spawned_agent_name(raw_name);
                if name.is_empty() {
                    self.messages.push(ChatMessage::new(
                        "system",
                        "Agent name must contain at least one letter or number.",
                    ));
                    return;
                }
                if name != raw_name {
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!("Normalized agent name to @{name} (from `{raw_name}`)."),
                    ));
                }

                let instructions = parts.next().map(str::trim).filter(|s| !s.is_empty());
                match instructions {
                    Some(custom) => (name.to_string(), custom.to_string(), false),
                    None => (name.to_string(), default_instructions(&name), true),
                }
            };

            if self.spawned_agents.contains_key(&name) {
                self.messages.push(ChatMessage::new(
                    "system",
                    format!("Agent @{name} already exists. Use /kill {name} first."),
                ));
                return;
            }

            match Session::new().await {
                Ok(mut session) => {
                    // Use the same model as the main chat
                    session.metadata.model = self
                        .active_model
                        .clone()
                        .or_else(|| config.default_model.clone());
                    session.agent = name.clone();
                    if let Some(ref b) = self.bus {
                        session.bus = Some(b.clone());
                    }

                    // Add system message with the agent's instructions
                    session.add_message(crate::provider::Message {
                        role: Role::System,
                        content: vec![ContentPart::Text {
                            text: format!(
                                "You are @{name}, a specialized sub-agent. {instructions}\n\n\
                                 When you receive a message from another agent (prefixed with their name), \
                                 respond helpfully. Keep responses concise and focused on your specialty.\n\
                                 If you edit code, keep changes minimal and scoped to the explicit request. \
                                 Avoid unrelated refactors or formatting churn."
                            ),
                        }],
                    });

                    // Announce on bus
                    let mut protocol_registered = false;
                    if let Some(ref bus) = self.bus {
                        let handle = bus.handle(&name);
                        handle.announce_ready(vec!["sub-agent".to_string(), name.clone()]);
                        protocol_registered = bus.registry.get(&name).is_some();
                    }

                    let agent = SpawnedAgent {
                        name: name.clone(),
                        instructions: instructions.clone(),
                        session,
                        is_processing: false,
                    };
                    self.spawned_agents.insert(name.clone(), agent);
                    self.active_spawned_agent = Some(name.clone());

                    let protocol_line = if protocol_registered {
                        format!("Protocol registration: ✅ bus://local/{name}")
                    } else {
                        "Protocol registration: ⚠ unavailable (bus not connected)".to_string()
                    };
                    let profile_summary = format_agent_profile_summary(&name);

                    self.messages.push(ChatMessage::new(
                        "system",
                        format!(
                            "Spawned agent {}\nProfile: {}\nInstructions: {instructions}\nFocused chat on @{name}. Type directly, or use @{name} <message>.\n{protocol_line}{}",
                            format_agent_identity(&name),
                            profile_summary,
                            if used_default_instructions {
                                "\nTip: I used friendly default instructions. You can customize with /add <name> <instructions>."
                            } else {
                                ""
                            }
                        ),
                    ));
                }
                Err(e) => {
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!("Failed to spawn agent: {e}"),
                    ));
                }
            }
            return;
        }

        // Check for /agents command - list spawned agents
        if message.trim() == "/agents" {
            if self.spawned_agents.is_empty() {
                self.messages.push(ChatMessage::new(
                    "system",
                    "No agents spawned. Use /spawn <name> <instructions> to create one.",
                ));
            } else {
                let mut lines = vec![format!(
                    "Active agents: {} (protocol registered: {})",
                    self.spawned_agents.len(),
                    self.protocol_registered_count()
                )];

                let mut agents = self.spawned_agents.iter().collect::<Vec<_>>();
                agents.sort_by(|(a, _), (b, _)| a.to_lowercase().cmp(&b.to_lowercase()));

                for (name, agent) in agents {
                    let status = if agent.is_processing {
                        "⚡ working"
                    } else {
                        "● idle"
                    };
                    let protocol_status = if self.is_agent_protocol_registered(name) {
                        "🔗 protocol"
                    } else {
                        "⚠ protocol-pending"
                    };
                    let focused = if self.active_spawned_agent.as_deref() == Some(name.as_str()) {
                        " [focused]"
                    } else {
                        ""
                    };
                    let profile_summary = format_agent_profile_summary(name);
                    lines.push(format!(
                        "  {} @{name} [{status}] {protocol_status}{focused} — {} | {}",
                        agent_avatar(name),
                        profile_summary,
                        agent.instructions
                    ));
                }
                self.messages
                    .push(ChatMessage::new("system", lines.join("\n")));
                self.messages.push(ChatMessage::new(
                    "system",
                    "Tip: use /agent to open the picker, /agent <name> to focus, or Ctrl+A.",
                ));
            }
            return;
        }

        // Check for /kill command - remove a spawned agent
        if let Some(name) = command_with_optional_args(&message, "/kill") {
            if name.is_empty() {
                self.messages
                    .push(ChatMessage::new("system", "Usage: /kill <name>"));
                return;
            }

            let name = name.to_string();
            let removed_local = self.spawned_agents.remove(&name).is_some();
            let removed_tool_store = crate::tool::agent::remove_agent(&name);
            if removed_local || removed_tool_store {
                // Remove its response channels
                self.agent_response_rxs.retain(|(n, _)| n != &name);
                self.streaming_agent_texts.remove(&name);
                if self.active_spawned_agent.as_deref() == Some(name.as_str()) {
                    self.active_spawned_agent = None;
                }
                // Announce shutdown on bus
                if let Some(ref bus) = self.bus {
                    let handle = bus.handle(&name);
                    handle.announce_shutdown();
                }
                self.messages.push(ChatMessage::new(
                    "system",
                    format!("Agent @{name} removed."),
                ));
            } else {
                self.messages.push(ChatMessage::new(
                    "system",
                    format!("No agent named @{name}. Use /agents to list."),
                ));
            }
            return;
        }

        // Check for @mention - route message to a specific spawned agent
        if message.trim().starts_with('@') {
            let trimmed = message.trim();
            let (target, content) = match trimmed.split_once(' ') {
                Some((mention, rest)) => (
                    mention.strip_prefix('@').unwrap_or(mention).to_string(),
                    rest.to_string(),
                ),
                None => {
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!(
                            "Usage: @agent_name your message\nAvailable: {}",
                            if self.spawned_agents.is_empty() {
                                "none (use /spawn first)".to_string()
                            } else {
                                self.spawned_agents
                                    .keys()
                                    .map(|n| format!("@{n}"))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            }
                        ),
                    ));
                    return;
                }
            };

            if !self.spawned_agents.contains_key(&target) {
                self.messages.push(ChatMessage::new(
                    "system",
                    format!(
                        "No agent named @{target}. Available: {}",
                        if self.spawned_agents.is_empty() {
                            "none (use /spawn first)".to_string()
                        } else {
                            self.spawned_agents
                                .keys()
                                .map(|n| format!("@{n}"))
                                .collect::<Vec<_>>()
                                .join(", ")
                        }
                    ),
                ));
                return;
            }

            // Show the user's @mention message in chat
            self.messages
                .push(ChatMessage::new("user", format!("@{target} {content}")));
            self.scroll = SCROLL_BOTTOM;

            // Send the message over the bus
            if let Some(ref bus) = self.bus {
                let handle = bus.handle("user");
                handle.send_to_agent(
                    &target,
                    vec![crate::a2a::types::Part::Text {
                        text: content.clone(),
                    }],
                );
            }

            // Send the message to the target agent's session
            self.send_to_agent(&target, &content, config).await;
            return;
        }

        // If a spawned agent is focused, route plain messages there automatically.
        if !message.trim().starts_with('/')
            && let Some(target) = self.active_spawned_agent.clone()
        {
            if !self.spawned_agents.contains_key(&target) {
                self.active_spawned_agent = None;
                self.messages.push(ChatMessage::new(
                    "system",
                    format!(
                        "Focused agent @{target} is no longer available. Use /agents or /spawn to continue."
                    ),
                ));
                return;
            }

            let content = message.trim().to_string();
            if content.is_empty() {
                return;
            }

            self.messages
                .push(ChatMessage::new("user", format!("@{target} {content}")));
            self.scroll = SCROLL_BOTTOM;

            if let Some(ref bus) = self.bus {
                let handle = bus.handle("user");
                handle.send_to_agent(
                    &target,
                    vec![crate::a2a::types::Part::Text {
                        text: content.clone(),
                    }],
                );
            }

            self.send_to_agent(&target, &content, config).await;
            return;
        }

        // Check for /sessions command - open session picker
        if message.trim() == "/sessions" {
            let limit = std::env::var("CODETETHER_SESSION_PICKER_LIMIT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100);
            // Reset offset when opening session picker
            self.session_picker_offset = 0;
            match list_sessions_paged(&self.workspace_dir, limit, 0).await {
                Ok(sessions) => {
                    if sessions.is_empty() {
                        self.messages
                            .push(ChatMessage::new("system", "No saved sessions found."));
                    } else {
                        self.update_cached_sessions(sessions);
                        self.session_picker_selected = 0;
                        self.view_mode = ViewMode::SessionPicker;
                    }
                }
                Err(e) => {
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!("Failed to list sessions: {}", e),
                    ));
                }
            }
            return;
        }

        // Check for /resume command to load a session or resume an interrupted relay
        if message.trim() == "/resume" || message.trim().starts_with("/resume ") {
            let session_id = message
                .trim()
                .strip_prefix("/resume")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty());

            // If no specific session ID, check for an interrupted relay checkpoint first
            if session_id.is_none()
                && let Some(checkpoint) =
                    RelayCheckpoint::load_for_workspace(&self.workspace_dir).await
            {
                self.messages.push(ChatMessage::new("user", "/resume"));
                self.resume_autochat_relay(checkpoint).await;
                return;
            }

            let loaded = if let Some(id) = session_id {
                Session::load(id).await
            } else {
                Session::last_for_directory(Some(&self.workspace_dir)).await
            };

            match loaded {
                Ok(session) => {
                    // Convert session messages to chat messages
                    self.messages.clear();
                    self.messages.push(ChatMessage::new(
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
                                        self.messages
                                            .push(ChatMessage::new(role_str, text.clone()));
                                    }
                                }
                                ContentPart::Image { url, mime_type } => {
                                    self.messages.push(
                                        ChatMessage::new(role_str, "").with_message_type(
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
                                    let (preview, truncated) = build_tool_arguments_preview(
                                        name,
                                        arguments,
                                        TOOL_ARGS_PREVIEW_MAX_LINES,
                                        TOOL_ARGS_PREVIEW_MAX_BYTES,
                                    );
                                    self.messages.push(
                                        ChatMessage::new(role_str, format!("🔧 {name}"))
                                            .with_message_type(MessageType::ToolCall {
                                                name: name.clone(),
                                                arguments_preview: preview,
                                                arguments_len: arguments.len(),
                                                truncated,
                                            }),
                                    );
                                }
                                ContentPart::ToolResult { content, .. } => {
                                    let truncated = truncate_with_ellipsis(content, 500);
                                    let (preview, preview_truncated) = build_text_preview(
                                        content,
                                        TOOL_OUTPUT_PREVIEW_MAX_LINES,
                                        TOOL_OUTPUT_PREVIEW_MAX_BYTES,
                                    );
                                    self.messages.push(
                                        ChatMessage::new(
                                            role_str,
                                            format!("✅ Result\n{truncated}"),
                                        )
                                        .with_message_type(MessageType::ToolResult {
                                            name: "tool".to_string(),
                                            output_preview: preview,
                                            output_len: content.len(),
                                            truncated: preview_truncated,
                                            success: true,
                                            duration_ms: None,
                                        }),
                                    );
                                }
                                ContentPart::File { path, mime_type } => {
                                    self.messages.push(
                                        ChatMessage::new(role_str, format!("📎 {}", path))
                                            .with_message_type(MessageType::File {
                                                path: path.clone(),
                                                mime_type: mime_type.clone(),
                                            }),
                                    );
                                }
                                ContentPart::Thinking { text } => {
                                    if !text.is_empty() {
                                        self.messages.push(
                                            ChatMessage::new(role_str, text.clone())
                                                .with_message_type(MessageType::Thinking(
                                                    text.clone(),
                                                )),
                                        );
                                    }
                                }
                            }
                        }
                    }

                    self.current_agent = session.agent.clone();
                    self.session = Some(session);
                    self.scroll = SCROLL_BOTTOM;
                }
                Err(e) => {
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!("Failed to load session: {}", e),
                    ));
                }
            }
            return;
        }

        // Check for /model command - open model picker
        if message.trim() == "/model" || message.trim().starts_with("/model ") {
            let direct_model = message
                .trim()
                .strip_prefix("/model")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty());

            if let Some(model_str) = direct_model {
                // Direct set: /model provider/model-name
                self.active_model = Some(model_str.to_string());
                if let Some(session) = self.session.as_mut() {
                    session.metadata.model = Some(model_str.to_string());
                }
                self.persist_active_session("direct_model_set").await;
                self.messages.push(ChatMessage::new(
                    "system",
                    format!("Model set to: {}", model_str),
                ));
            } else {
                // Open model picker
                self.open_model_picker(config).await;
            }
            return;
        }

        if message.trim() == "/local" || message.trim().starts_with("/local ") {
            if self.provider_registry.is_none() {
                match crate::provider::ProviderRegistry::from_vault().await {
                    Ok(registry) => {
                        self.provider_registry = Some(std::sync::Arc::new(registry));
                    }
                    Err(vault_err) => {
                        tracing::warn!(
                            error = %vault_err,
                            "Provider registry from_vault failed during /local; falling back to config/env"
                        );
                        match crate::provider::ProviderRegistry::from_config(config).await {
                            Ok(registry) => {
                                self.provider_registry = Some(std::sync::Arc::new(registry));
                            }
                            Err(config_err) => {
                                self.messages.push(ChatMessage::new(
                                    "system",
                                    format!(
                                        "Failed to load providers for local mode.\nVault error: {vault_err}\nConfig/env fallback error: {config_err}"
                                    ),
                                ));
                                self.scroll = SCROLL_BOTTOM;
                                return;
                            }
                        }
                    }
                }
            }

            if self
                .provider_registry
                .as_ref()
                .and_then(|registry| registry.get("local_cuda"))
                .is_none()
            {
                // Try one forced refresh before refusing local mode. This helps when
                // provider preload finished before local env vars were fully available.
                match crate::provider::ProviderRegistry::from_config(config).await {
                    Ok(registry) => {
                        self.provider_registry = Some(std::sync::Arc::new(registry));
                    }
                    Err(err) => {
                        tracing::warn!(
                            error = %err,
                            "Provider registry config/env refresh failed during /local"
                        );
                    }
                }
            }

            let Some(registry) = self.provider_registry.as_ref() else {
                self.messages.push(ChatMessage::new(
                    "system",
                    "Providers are still loading; cannot enable local mode yet.",
                ));
                self.scroll = SCROLL_BOTTOM;
                return;
            };

            if registry.get("local_cuda").is_none() {
                let available = registry.list().join(", ");
                self.messages.push(ChatMessage::new(
                    "system",
                    format!(
                        "Local mode unavailable: provider `local_cuda` is not configured.\nAvailable providers: {available}\nHint: set CODETETHER_LOCAL_CUDA=1 and LOCAL_CUDA_MODEL_PATH/LOCAL_CUDA_TOKENIZER_PATH (or configure local-cuda in Vault)."
                    ),
                ));
                self.scroll = SCROLL_BOTTOM;
                return;
            }

            let requested = message
                .trim()
                .strip_prefix("/local")
                .map(str::trim)
                .filter(|s| !s.is_empty());
            let current_model = self
                .active_model
                .as_deref()
                .or(config.default_model.as_deref());
            let local_model = requested
                .map(normalize_local_model_ref)
                .unwrap_or_else(|| resolve_local_loop_model(current_model));

            self.active_model = Some(local_model.clone());
            if let Some(session) = self.session.as_mut() {
                session.metadata.model = Some(local_model.clone());
            }
            self.persist_active_session("direct_local_model_set").await;
            self.messages.push(ChatMessage::new(
                "system",
                format!(
                    "Local mode enabled: {local_model}\nUse /autochat-local <task> for forced local relay runs."
                ),
            ));
            return;
        }

        // Check for /undo command - remove last user turn and response
        if message.trim() == "/undo" {
            // Remove from TUI messages: walk backwards and remove everything
            // until we've removed the last "user" message (inclusive)
            let mut found_user = false;
            while let Some(msg) = self.messages.last() {
                if msg.role == "user" {
                    if found_user {
                        break; // hit the previous user turn, stop
                    }
                    found_user = true;
                }
                // Skip system messages that aren't part of the turn
                if msg.role == "system" && !found_user {
                    break;
                }
                self.messages.pop();
            }

            if !found_user {
                self.messages
                    .push(ChatMessage::new("system", "Nothing to undo."));
                return;
            }

            // Remove from session: walk backwards removing the last user message
            // and all assistant/tool messages after it
            if let Some(session) = self.session.as_mut() {
                let mut found_session_user = false;
                while let Some(msg) = session.messages.last() {
                    if msg.role == crate::provider::Role::User {
                        if found_session_user {
                            break;
                        }
                        found_session_user = true;
                    }
                    if msg.role == crate::provider::Role::System && !found_session_user {
                        break;
                    }
                    session.messages.pop();
                }
                if let Err(e) = session.save().await {
                    tracing::warn!(error = %e, "Failed to save session after undo");
                }
            }

            self.messages.push(ChatMessage::new(
                "system",
                "Undid last message and response.",
            ));
            self.scroll = SCROLL_BOTTOM;
            return;
        }

        // Check for /new command to start a fresh session
        if message.trim() == "/new" {
            self.persist_active_session("new_session_command").await;
            self.session = None;
            self.messages.clear();
            self.messages.push(ChatMessage::new(
                "system",
                "Started a new session. Previous session was saved.",
            ));
            return;
        }

        // Normal chat messages require providers. Avoid blocking the TUI event loop by awaiting
        // Vault/provider initialization here; instead ask the user to retry once the background
        // preload completes.
        if self.provider_registry.is_none() {
            self.messages.push(ChatMessage::new(
                "system",
                "Providers are still loading. Please wait a moment and press Enter again.",
            ));
            self.scroll = SCROLL_BOTTOM;
            self.input = message;
            self.cursor_position = self.input.len();
            return;
        }

        // Add user message
        self.messages
            .push(ChatMessage::new("user", message.clone()));

        // Auto-scroll to bottom when user sends a message
        self.scroll = SCROLL_BOTTOM;

        let current_agent = self.current_agent.clone();
        let model = self
            .active_model
            .clone()
            .or_else(|| {
                config
                    .agents
                    .get(&current_agent)
                    .and_then(|agent| agent.model.clone())
            })
            .or_else(|| config.default_model.clone())
            .or_else(|| Some("zai/glm-5".to_string()));

        // Initialize session if needed
        if self.session.is_none() {
            match Session::new().await {
                Ok(mut session) => {
                    if let Some(ref b) = self.bus {
                        session.bus = Some(b.clone());
                    }
                    self.session = Some(session);
                }
                Err(err) => {
                    tracing::error!(error = %err, "Failed to create session");
                    self.messages
                        .push(ChatMessage::new("assistant", format!("Error: {err}")));
                    return;
                }
            }
        }

        let selected_model_ref = {
            let session = match self.session.as_mut() {
                Some(session) => session,
                None => {
                    self.messages.push(ChatMessage::new(
                        "assistant",
                        "Error: session not initialized",
                    ));
                    return;
                }
            };

            if let Some(model) = model {
                session.metadata.model = Some(model);
            }

            session.agent = current_agent;
            session.metadata.model.clone()
        };
        self.reset_smart_switch_state();
        if let Some(model_ref) = selected_model_ref.as_deref() {
            self.smart_switch_attempted_models
                .insert(smart_switch_model_key(model_ref));
        }

        // Take any pending images to send with this message
        let pending_images: Vec<crate::session::ImageAttachment> =
            std::mem::take(&mut self.pending_images)
                .into_iter()
                .map(|img| crate::session::ImageAttachment {
                    data_url: img.data_url,
                    mime_type: Some("image/png".to_string()),
                })
                .collect();
        self.main_watchdog_root_prompt = Some(message.clone());
        self.main_watchdog_restart_count = 0;
        self.spawn_main_processing_task(message.clone(), pending_images);
    }

    fn handle_response(&mut self, event: SessionEvent) {
        // Auto-scroll to bottom when new content arrives
        self.scroll = SCROLL_BOTTOM;
        self.main_last_event_at = Some(Instant::now());

        match event {
            SessionEvent::Thinking => {
                self.processing_message = Some("Thinking...".to_string());
                self.current_tool = None;
                self.current_tool_started_at = None;
                if self.processing_started_at.is_none() {
                    self.processing_started_at = Some(Instant::now());
                }
            }
            SessionEvent::ToolCallStart { name, arguments } => {
                // Flush any streaming text before showing tool call
                if let Some(text) = self.streaming_text.take()
                    && !text.is_empty()
                {
                    self.messages.push(ChatMessage::new("assistant", text));
                }
                self.processing_message = Some(format!("Running {}...", name));
                self.current_tool = Some(name.clone());
                self.current_tool_started_at = Some(Instant::now());
                self.tool_call_count += 1;

                let (preview, truncated) = build_tool_arguments_preview(
                    &name,
                    &arguments,
                    TOOL_ARGS_PREVIEW_MAX_LINES,
                    TOOL_ARGS_PREVIEW_MAX_BYTES,
                );
                self.messages.push(
                    ChatMessage::new("tool", format!("🔧 {}", name)).with_message_type(
                        MessageType::ToolCall {
                            name,
                            arguments_preview: preview,
                            arguments_len: arguments.len(),
                            truncated,
                        },
                    ),
                );
            }
            SessionEvent::ToolCallComplete {
                name,
                output,
                success,
            } => {
                let icon = if success { "✓" } else { "✗" };
                let duration_ms = self
                    .current_tool_started_at
                    .take()
                    .map(|started| started.elapsed().as_millis() as u64);

                let (preview, truncated) = build_text_preview(
                    &output,
                    TOOL_OUTPUT_PREVIEW_MAX_LINES,
                    TOOL_OUTPUT_PREVIEW_MAX_BYTES,
                );
                self.messages.push(
                    ChatMessage::new("tool", format!("{} {}", icon, name)).with_message_type(
                        MessageType::ToolResult {
                            name,
                            output_preview: preview,
                            output_len: output.len(),
                            truncated,
                            success,
                            duration_ms,
                        },
                    ),
                );
                self.current_tool = None;
                self.processing_message = Some("Thinking...".to_string());
            }
            SessionEvent::TextChunk(text) => {
                // Show streaming text as it arrives (before TextComplete finalizes)
                self.streaming_text = Some(text);
            }
            SessionEvent::ThinkingComplete(text) => {
                if !text.is_empty() {
                    self.messages.push(
                        ChatMessage::new("assistant", &text)
                            .with_message_type(MessageType::Thinking(text)),
                    );
                }
            }
            SessionEvent::TextComplete(text) => {
                // Clear streaming preview and add the final message
                self.streaming_text = None;
                if !text.is_empty() {
                    self.messages.push(ChatMessage::new("assistant", text));
                }
            }
            SessionEvent::UsageReport {
                prompt_tokens,
                completion_tokens,
                duration_ms,
                model,
            } => {
                let cost_usd = estimate_cost(&model, prompt_tokens, completion_tokens);
                let meta = UsageMeta {
                    prompt_tokens,
                    completion_tokens,
                    duration_ms,
                    cost_usd,
                };
                // Attach to the most recent assistant message
                if let Some(msg) = self
                    .messages
                    .iter_mut()
                    .rev()
                    .find(|m| m.role == "assistant")
                {
                    *msg = msg.clone().with_usage_meta(meta);
                }
            }
            SessionEvent::SessionSync(session) => {
                // Sync the updated session (with full conversation history) back
                // so subsequent messages include prior context.
                self.session = Some(*session);
            }
            SessionEvent::Error(err) => {
                self.current_tool_started_at = None;
                self.maybe_schedule_smart_switch_retry(&err);
                self.messages
                    .push(ChatMessage::new("assistant", format!("Error: {}", err)));
            }
            SessionEvent::Done => {
                if !self.apply_pending_smart_switch_retry() {
                    self.reset_main_processing_state();
                }
            }
        }
    }

    /// Dispatch a message to a specific spawned agent without awaiting.
    fn dispatch_to_agent_internal(&mut self, agent_name: &str, message: &str) {
        let Some(registry) = self.provider_registry.clone() else {
            self.messages.push(ChatMessage::new(
                "system",
                "Providers are still loading; cannot message sub-agents yet.",
            ));
            self.scroll = SCROLL_BOTTOM;
            return;
        };

        let agent = match self.spawned_agents.get_mut(agent_name) {
            Some(a) => a,
            None => return,
        };

        agent.is_processing = true;
        self.streaming_agent_texts.remove(agent_name);
        let session_clone = agent.session.clone();
        let msg_clone = message.to_string();
        let agent_name_owned = agent_name.to_string();
        let bus_arc = self.bus.clone();

        let (tx, rx) = mpsc::channel(100);
        self.agent_response_rxs.push((agent_name.to_string(), rx));

        tokio::spawn(async move {
            let mut session = session_clone;
            if let Err(err) = session
                .prompt_with_events(&msg_clone, tx.clone(), registry)
                .await
            {
                tracing::error!(agent = %agent_name_owned, error = %err, "Spawned agent failed");
                session.add_message(crate::provider::Message {
                    role: crate::provider::Role::Assistant,
                    content: vec![crate::provider::ContentPart::Text {
                        text: format!("Error: {err:#}"),
                    }],
                });
                if let Err(save_err) = session.save().await {
                    tracing::warn!(
                        agent = %agent_name_owned,
                        error = %save_err,
                        session_id = %session.id,
                        "Failed to save spawned-agent session after processing error"
                    );
                }
                let _ = tx.send(SessionEvent::SessionSync(Box::new(session))).await;
                let _ = tx.send(SessionEvent::Error(format!("{err:#}"))).await;
                let _ = tx.send(SessionEvent::Done).await;
            }

            // Send the agent's response over the bus
            if let Some(ref bus) = bus_arc {
                let handle = bus.handle(&agent_name_owned);
                handle.send(
                    format!("agent.{agent_name_owned}.events"),
                    crate::bus::BusMessage::AgentMessage {
                        from: agent_name_owned.clone(),
                        to: "user".to_string(),
                        parts: vec![crate::a2a::types::Part::Text {
                            text: "(response complete)".to_string(),
                        }],
                    },
                );
            }
        });
    }

    /// Send a message to a specific spawned agent
    async fn send_to_agent(&mut self, agent_name: &str, message: &str, _config: &Config) {
        self.dispatch_to_agent_internal(agent_name, message);
    }

    fn worker_policy_user(&self) -> crate::server::policy::PolicyUser {
        let roles = std::env::var("CODETETHER_POLICY_ROLES")
            .ok()
            .map(|raw| {
                raw.split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            })
            .filter(|roles| !roles.is_empty())
            .unwrap_or_else(|| vec!["editor".to_string()]);

        crate::server::policy::PolicyUser {
            user_id: std::env::var("CODETETHER_ACTOR_ID")
                .unwrap_or_else(|_| "tui-autonomous".to_string()),
            roles,
            tenant_id: std::env::var("CODETETHER_TENANT_ID").ok(),
            scopes: Vec::new(),
            auth_source: "keycloak".to_string(),
        }
    }

    /// Handle an event from a spawned agent
    fn handle_agent_response(&mut self, agent_name: &str, event: SessionEvent) {
        self.scroll = SCROLL_BOTTOM;

        match event {
            SessionEvent::Thinking => {
                // Show thinking indicator for this agent
                if let Some(agent) = self.spawned_agents.get_mut(agent_name) {
                    agent.is_processing = true;
                }
            }
            SessionEvent::ToolCallStart { name, arguments } => {
                self.streaming_agent_texts.remove(agent_name);
                self.agent_tool_started_at
                    .insert(agent_name.to_string(), Instant::now());
                let (preview, truncated) = build_tool_arguments_preview(
                    &name,
                    &arguments,
                    TOOL_ARGS_PREVIEW_MAX_LINES,
                    TOOL_ARGS_PREVIEW_MAX_BYTES,
                );
                self.messages.push(
                    ChatMessage::new(
                        "tool",
                        format!("🔧 {} → {name}", format_agent_identity(agent_name)),
                    )
                    .with_message_type(MessageType::ToolCall {
                        name,
                        arguments_preview: preview,
                        arguments_len: arguments.len(),
                        truncated,
                    })
                    .with_agent_name(agent_name),
                );
            }
            SessionEvent::ToolCallComplete {
                name,
                output,
                success,
            } => {
                self.streaming_agent_texts.remove(agent_name);
                let icon = if success { "✓" } else { "✗" };
                let duration_ms = self
                    .agent_tool_started_at
                    .remove(agent_name)
                    .map(|started| started.elapsed().as_millis() as u64);
                let (preview, truncated) = build_text_preview(
                    &output,
                    TOOL_OUTPUT_PREVIEW_MAX_LINES,
                    TOOL_OUTPUT_PREVIEW_MAX_BYTES,
                );
                self.messages.push(
                    ChatMessage::new(
                        "tool",
                        format!("{icon} {} → {name}", format_agent_identity(agent_name)),
                    )
                    .with_message_type(MessageType::ToolResult {
                        name,
                        output_preview: preview,
                        output_len: output.len(),
                        truncated,
                        success,
                        duration_ms,
                    })
                    .with_agent_name(agent_name),
                );
            }
            SessionEvent::TextChunk(text) => {
                if text.is_empty() {
                    self.streaming_agent_texts.remove(agent_name);
                } else {
                    self.streaming_agent_texts
                        .insert(agent_name.to_string(), text);
                }
            }
            SessionEvent::ThinkingComplete(text) => {
                self.streaming_agent_texts.remove(agent_name);
                if !text.is_empty() {
                    self.messages.push(
                        ChatMessage::new("assistant", &text)
                            .with_message_type(MessageType::Thinking(text))
                            .with_agent_name(agent_name),
                    );
                }
            }
            SessionEvent::TextComplete(text) => {
                self.streaming_agent_texts.remove(agent_name);
                if !text.is_empty() {
                    self.messages
                        .push(ChatMessage::new("assistant", &text).with_agent_name(agent_name));
                }
            }
            SessionEvent::UsageReport {
                prompt_tokens,
                completion_tokens,
                duration_ms,
                model,
            } => {
                let cost_usd = estimate_cost(&model, prompt_tokens, completion_tokens);
                let meta = UsageMeta {
                    prompt_tokens,
                    completion_tokens,
                    duration_ms,
                    cost_usd,
                };
                if let Some(msg) =
                    self.messages.iter_mut().rev().find(|m| {
                        m.role == "assistant" && m.agent_name.as_deref() == Some(agent_name)
                    })
                {
                    *msg = msg.clone().with_usage_meta(meta);
                }
            }
            SessionEvent::SessionSync(session) => {
                if let Some(agent) = self.spawned_agents.get_mut(agent_name) {
                    agent.session = *session;
                }
            }
            SessionEvent::Error(err) => {
                self.streaming_agent_texts.remove(agent_name);
                self.agent_tool_started_at.remove(agent_name);
                self.messages.push(
                    ChatMessage::new("assistant", format!("Error: {err}"))
                        .with_agent_name(agent_name),
                );
            }
            SessionEvent::Done => {
                self.streaming_agent_texts.remove(agent_name);
                self.agent_tool_started_at.remove(agent_name);
                if let Some(agent) = self.spawned_agents.get_mut(agent_name) {
                    agent.is_processing = false;
                }
            }
        }
    }

    fn handle_autochat_event(&mut self, event: AutochatUiEvent) -> bool {
        match event {
            AutochatUiEvent::Progress(status) => {
                self.autochat_status = Some(status);
                false
            }
            AutochatUiEvent::SystemMessage(text) => {
                self.autochat_status = Some(
                    text.lines()
                        .next()
                        .unwrap_or("Relay update")
                        .trim()
                        .to_string(),
                );
                self.messages.push(ChatMessage::new("system", text));
                self.scroll = SCROLL_BOTTOM;
                false
            }
            AutochatUiEvent::AgentEvent { agent_name, event } => {
                self.autochat_status = Some(format!("Streaming from @{agent_name}…"));
                self.handle_agent_response(&agent_name, *event);
                false
            }
            AutochatUiEvent::Completed {
                summary,
                okr_id,
                okr_run_id,
                relay_id,
            } => {
                self.autochat_status = Some("Completed".to_string());

                // Add OKR correlation info to the completion message if present
                let mut full_summary = summary.clone();
                if let (Some(okr_id), Some(okr_run_id)) = (&okr_id, &okr_run_id) {
                    full_summary.push_str(&format!(
                        "\n\n📊 OKR Tracking: okr_id={} run_id={}",
                        &okr_id[..8.min(okr_id.len())],
                        &okr_run_id[..8.min(okr_run_id.len())]
                    ));
                }
                if let Some(rid) = &relay_id {
                    full_summary.push_str(&format!("\n🔗 Relay: {}", rid));
                }

                self.messages
                    .push(ChatMessage::new("assistant", full_summary));
                self.scroll = SCROLL_BOTTOM;
                true
            }
        }
    }

    /// Handle a swarm event
    fn handle_swarm_event(&mut self, event: SwarmEvent) {
        self.swarm_state.handle_event(event.clone());

        // When swarm completes, switch back to chat view with summary
        if let SwarmEvent::Complete { success, ref stats } = event {
            self.view_mode = ViewMode::Chat;
            let summary = if success {
                format!(
                    "Swarm completed successfully.\n\
                     Subtasks: {} completed, {} failed\n\
                     Total tool calls: {}\n\
                     Time: {:.1}s (speedup: {:.1}x)",
                    stats.subagents_completed,
                    stats.subagents_failed,
                    stats.total_tool_calls,
                    stats.execution_time_ms as f64 / 1000.0,
                    stats.speedup_factor
                )
            } else {
                format!(
                    "Swarm completed with failures.\n\
                     Subtasks: {} completed, {} failed\n\
                     Check the subtask results for details.",
                    stats.subagents_completed, stats.subagents_failed
                )
            };
            self.messages.push(ChatMessage::new("system", &summary));
            self.swarm_rx = None;
        }

        if let SwarmEvent::Error(ref err) = event {
            self.messages
                .push(ChatMessage::new("system", format!("Swarm error: {}", err)));
        }
    }

    /// Handle a Ralph event
    fn handle_ralph_event(&mut self, event: RalphEvent) {
        self.ralph_state.handle_event(event.clone());

        // When Ralph completes, switch back to chat view with summary
        if let RalphEvent::Complete {
            ref status,
            passed,
            total,
        } = event
        {
            self.view_mode = ViewMode::Chat;
            let summary = format!(
                "Ralph loop finished: {}\n\
                 Stories: {}/{} passed",
                status, passed, total
            );
            self.messages.push(ChatMessage::new("system", &summary));
            self.ralph_rx = None;
        }

        if let RalphEvent::Error(ref err) = event {
            self.messages
                .push(ChatMessage::new("system", format!("Ralph error: {}", err)));
        }
    }

    /// Start Ralph execution for a PRD
    async fn start_ralph_execution(&mut self, prd_path: String, config: &Config) {
        // Add user message
        self.messages
            .push(ChatMessage::new("user", format!("/ralph {}", prd_path)));

        // Get model from config
        let model = self
            .active_model
            .clone()
            .or_else(|| config.default_model.clone())
            .or_else(|| Some("zai/glm-5".to_string()));

        let model = match model {
            Some(m) => m,
            None => {
                self.messages.push(ChatMessage::new(
                    "system",
                    "No model configured. Use /model to select one first.",
                ));
                return;
            }
        };

        // Check PRD exists
        let prd_file = std::path::PathBuf::from(&prd_path);
        if !prd_file.exists() {
            self.messages.push(ChatMessage::new(
                "system",
                format!("PRD file not found: {}", prd_path),
            ));
            return;
        }

        // Create channel for ralph events
        let (tx, rx) = mpsc::channel(200);
        self.ralph_rx = Some(rx);

        // Switch to Ralph view
        self.view_mode = ViewMode::Ralph;
        self.ralph_state = RalphViewState::new();

        // Build Ralph config
        let ralph_config = RalphConfig {
            prd_path: prd_path.clone(),
            max_iterations: 10,
            progress_path: "progress.txt".to_string(),
            quality_checks_enabled: true,
            auto_commit: true,
            model: Some(model.clone()),
            use_rlm: true,
            parallel_enabled: true,
            max_concurrent_stories: 100,
            worktree_enabled: true,
            story_timeout_secs: 300,
            conflict_timeout_secs: 120,
            relay_enabled: true,
            relay_max_agents: 8,
            relay_max_rounds: 3,
            max_steps_per_story: 30,
        };

        // Parse provider/model from the model string
        let (provider_name, model_name) = if let Some(pos) = model.find('/') {
            (model[..pos].to_string(), model[pos + 1..].to_string())
        } else {
            (model.clone(), model.clone())
        };

        let prd_path_clone = prd_path.clone();
        let tx_clone = tx.clone();

        // Spawn Ralph execution
        tokio::spawn(async move {
            // Get provider from registry
            let provider = match crate::provider::ProviderRegistry::from_vault().await {
                Ok(registry) => match registry.get(&provider_name) {
                    Some(p) => p,
                    None => {
                        let _ = tx_clone
                            .send(RalphEvent::Error(format!(
                                "Provider '{}' not found",
                                provider_name
                            )))
                            .await;
                        return;
                    }
                },
                Err(e) => {
                    let _ = tx_clone
                        .send(RalphEvent::Error(format!(
                            "Failed to load providers: {}",
                            e
                        )))
                        .await;
                    return;
                }
            };

            let prd_path_buf = std::path::PathBuf::from(&prd_path_clone);
            match RalphLoop::new(prd_path_buf, provider, model_name, ralph_config).await {
                Ok(ralph) => {
                    let mut ralph = ralph.with_event_tx(tx_clone.clone());
                    match ralph.run().await {
                        Ok(_state) => {
                            // Complete event already emitted by run()
                        }
                        Err(e) => {
                            let _ = tx_clone.send(RalphEvent::Error(e.to_string())).await;
                        }
                    }
                }
                Err(e) => {
                    let _ = tx_clone
                        .send(RalphEvent::Error(format!(
                            "Failed to initialize Ralph: {}",
                            e
                        )))
                        .await;
                }
            }
        });

        self.messages.push(ChatMessage::new(
            "system",
            format!("Starting Ralph loop with PRD: {}", prd_path),
        ));
    }

    /// Start swarm execution for a task
    async fn start_swarm_execution(&mut self, task: String, config: &Config) {
        // Add user message
        self.messages
            .push(ChatMessage::new("user", format!("/swarm {}", task)));

        // Get model from config
        let model = config.default_model.clone();

        // Configure swarm
        let swarm_config = SwarmConfig {
            model,
            max_subagents: 10,
            max_steps_per_subagent: 50,
            worktree_enabled: true,
            worktree_auto_merge: true,
            working_dir: Some(
                std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string()),
            ),
            ..Default::default()
        };

        // Create channel for swarm events
        let (tx, rx) = mpsc::channel(100);
        self.swarm_rx = Some(rx);

        // Switch to swarm view
        self.view_mode = ViewMode::Swarm;
        self.swarm_state = SwarmViewState::new();

        // Send initial event
        let _ = tx
            .send(SwarmEvent::Started {
                task: task.clone(),
                total_subtasks: 0,
            })
            .await;

        // Spawn swarm execution — executor emits all events via event_tx
        let task_clone = task;
        let bus_arc = self.bus.clone();
        tokio::spawn(async move {
            // Create executor with event channel — it handles decomposition + execution
            let mut executor = SwarmExecutor::new(swarm_config).with_event_tx(tx.clone());
            if let Some(bus) = bus_arc {
                executor = executor.with_bus(bus);
            }
            let result = executor
                .execute(&task_clone, DecompositionStrategy::Automatic)
                .await;

            match result {
                Ok(swarm_result) => {
                    let _ = tx
                        .send(SwarmEvent::Complete {
                            success: swarm_result.success,
                            stats: swarm_result.stats,
                        })
                        .await;
                }
                Err(e) => {
                    let _ = tx.send(SwarmEvent::Error(e.to_string())).await;
                }
            }
        });
    }

    /// Populate and open the model picker overlay
    async fn open_model_picker(&mut self, config: &Config) {
        let mut models: Vec<(String, String, String)> = Vec::new();

        // Prefer cached provider registry to avoid blocking the UI on Vault calls.
        if let Some(registry) = self.provider_registry.as_ref() {
            for provider_name in registry.list() {
                if let Some(provider) = registry.get(provider_name) {
                    match provider.list_models().await {
                        Ok(model_list) => {
                            for m in model_list {
                                let label = format!("{}/{}", provider_name, m.id);
                                let value = format!("{}/{}", provider_name, m.id);
                                let name = m.name.clone();
                                models.push((label, value, name));
                            }
                        }
                        Err(e) => {
                            tracing::warn!("Failed to list models for {}: {}", provider_name, e);
                        }
                    }
                }
            }
        }

        // Fallback: also try from config
        if models.is_empty()
            && let Ok(registry) = crate::provider::ProviderRegistry::from_config(config).await
        {
            for provider_name in registry.list() {
                if let Some(provider) = registry.get(provider_name)
                    && let Ok(model_list) = provider.list_models().await
                {
                    for m in model_list {
                        let label = format!("{}/{}", provider_name, m.id);
                        let value = format!("{}/{}", provider_name, m.id);
                        let name = m.name.clone();
                        models.push((label, value, name));
                    }
                }
            }
        }

        if models.is_empty() {
            self.messages.push(ChatMessage::new(
                "system",
                "No models found yet. Providers may still be loading; try again in a moment.",
            ));
        } else {
            // Sort models by provider then name
            models.sort_by(|a, b| a.0.cmp(&b.0));
            self.model_picker_list = models;
            self.model_picker_selected = 0;
            self.model_picker_filter.clear();
            self.view_mode = ViewMode::ModelPicker;
        }
    }

    /// Get filtered session list for the session picker
    fn filtered_sessions(&self) -> Vec<(usize, &SessionSummary)> {
        if self.session_picker_filter.is_empty() {
            self.session_picker_list.iter().enumerate().collect()
        } else {
            let filter = self.session_picker_filter.to_lowercase();
            self.session_picker_list
                .iter()
                .enumerate()
                .filter(|(_, s)| {
                    s.title
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&filter)
                        || s.agent.to_lowercase().contains(&filter)
                        || s.id.to_lowercase().contains(&filter)
                })
                .collect()
        }
    }

    /// Get filtered model list
    fn filtered_models(&self) -> Vec<(usize, &(String, String, String))> {
        if self.model_picker_filter.is_empty() {
            self.model_picker_list.iter().enumerate().collect()
        } else {
            let filter = self.model_picker_filter.to_lowercase();
            self.model_picker_list
                .iter()
                .enumerate()
                .filter(|(_, (label, _, name))| {
                    label.to_lowercase().contains(&filter) || name.to_lowercase().contains(&filter)
                })
                .collect()
        }
    }

    /// Get filtered spawned agents list (sorted by name)
    fn filtered_spawned_agents(&self) -> Vec<(String, String, bool, bool)> {
        let mut agents: Vec<(String, String, bool, bool)> = self
            .spawned_agents
            .iter()
            .map(|(name, agent)| {
                let protocol_registered = self.is_agent_protocol_registered(name);
                (
                    name.clone(),
                    agent.instructions.clone(),
                    agent.is_processing,
                    protocol_registered,
                )
            })
            .collect();

        agents.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

        if self.agent_picker_filter.is_empty() {
            agents
        } else {
            let filter = self.agent_picker_filter.to_lowercase();
            agents
                .into_iter()
                .filter(|(name, instructions, _, _)| {
                    name.to_lowercase().contains(&filter)
                        || instructions.to_lowercase().contains(&filter)
                })
                .collect()
        }
    }

    /// Import sub-agents created via the `agent` tool into the TUI-local map.
    ///
    /// The tool runtime keeps a global store, while TUI commands/picker use
    /// `self.spawned_agents`.
    fn sync_spawned_agents_from_tool_store(&mut self) {
        let bus = self.bus.clone();
        for snapshot in crate::tool::agent::list_agent_snapshots() {
            let agent_name = snapshot.name.clone();
            let entry = self
                .spawned_agents
                .entry(agent_name.clone())
                .or_insert(SpawnedAgent {
                    name: snapshot.name,
                    instructions: snapshot.instructions,
                    session: snapshot.session,
                    is_processing: false,
                });

            if let Some(ref b) = bus {
                if entry.session.bus.is_none() {
                    entry.session.bus = Some(b.clone());
                }
                if b.registry.get(&agent_name).is_none() {
                    let handle = b.handle(&agent_name);
                    handle.announce_ready(vec!["sub-agent".to_string(), agent_name.clone()]);
                    tracing::info!(agent = %agent_name, "Auto-registered spawned agent on protocol bus");
                }
            }
        }
    }

    /// Open picker for choosing a spawned sub-agent to focus
    fn open_agent_picker(&mut self) {
        self.sync_spawned_agents_from_tool_store();
        if self.spawned_agents.is_empty() {
            self.messages.push(ChatMessage::new(
                "system",
                "No agents spawned yet. Use /spawn <name> <instructions> first.",
            ));
            return;
        }

        self.agent_picker_filter.clear();
        let filtered = self.filtered_spawned_agents();
        self.agent_picker_selected = if let Some(active) = &self.active_spawned_agent {
            filtered
                .iter()
                .position(|(name, _, _, _)| name == active)
                .unwrap_or(0)
        } else {
            0
        };
        self.view_mode = ViewMode::AgentPicker;
    }

    fn open_file_picker(&mut self) {
        if !self.file_picker_dir.exists() {
            self.file_picker_dir = self.workspace_dir.clone();
        }
        self.file_picker_filter.clear();
        self.file_picker_selected = 0;

        if let Err(err) = self.reload_file_picker_entries() {
            self.messages.push(ChatMessage::new(
                "system",
                format!(
                    "Failed to open file picker in {}: {}",
                    self.file_picker_dir.display(),
                    err
                ),
            ));
            self.scroll = SCROLL_BOTTOM;
            return;
        }

        self.view_mode = ViewMode::FilePicker;
        self.refresh_file_picker_preview();
    }

    fn reload_file_picker_entries(&mut self) -> Result<()> {
        let mut entries = Vec::new();

        if let Some(parent) = self.file_picker_dir.parent() {
            entries.push(FilePickerEntry {
                name: "..".to_string(),
                path: parent.to_path_buf(),
                kind: FilePickerEntryKind::Parent,
                size_bytes: None,
            });
        }

        let read_dir = std::fs::read_dir(&self.file_picker_dir)?;
        for entry in read_dir.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if name.is_empty() || should_skip_workspace_entry(&name) {
                continue;
            }

            let file_type = match entry.file_type() {
                Ok(ft) => ft,
                Err(_) => continue,
            };

            let (kind, size_bytes) = if file_type.is_dir() {
                (FilePickerEntryKind::Directory, None)
            } else if file_type.is_file() {
                let size = entry.metadata().ok().map(|m| m.len());
                (FilePickerEntryKind::File, size)
            } else {
                continue;
            };

            entries.push(FilePickerEntry {
                name,
                path,
                kind,
                size_bytes,
            });
        }

        entries.sort_by(|a, b| {
            let rank = |kind: FilePickerEntryKind| match kind {
                FilePickerEntryKind::Parent => 0_u8,
                FilePickerEntryKind::Directory => 1,
                FilePickerEntryKind::File => 2,
            };
            rank(a.kind)
                .cmp(&rank(b.kind))
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });
        entries.truncate(FILE_PICKER_MAX_ENTRIES);

        self.file_picker_entries = entries;
        if self.file_picker_selected >= self.file_picker_entries.len() {
            self.file_picker_selected = self.file_picker_entries.len().saturating_sub(1);
        }
        self.refresh_file_picker_preview();

        Ok(())
    }

    fn filtered_file_picker_entries(&self) -> Vec<(usize, &FilePickerEntry)> {
        if self.file_picker_filter.is_empty() {
            return self.file_picker_entries.iter().enumerate().collect();
        }

        let filter = self.file_picker_filter.to_lowercase();
        self.file_picker_entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| {
                entry.kind == FilePickerEntryKind::Parent
                    || entry.name.to_lowercase().contains(&filter)
            })
            .collect()
    }

    fn file_picker_go_parent(&mut self) {
        if let Some(parent) = self.file_picker_dir.parent() {
            self.file_picker_dir = parent.to_path_buf();
            self.file_picker_filter.clear();
            self.file_picker_selected = 0;
            if let Err(err) = self.reload_file_picker_entries() {
                self.messages.push(ChatMessage::new(
                    "system",
                    format!("Failed to navigate to parent directory: {}", err),
                ));
                self.scroll = SCROLL_BOTTOM;
            }
        }
    }

    fn refresh_file_picker_preview(&mut self) {
        let filtered = self.filtered_file_picker_entries();
        if filtered.is_empty() {
            self.file_picker_preview_title = "No selection".to_string();
            self.file_picker_preview_lines = vec![
                "No files match the current filter.".to_string(),
                "Backspace clears the filter.".to_string(),
            ];
            return;
        }

        let selected_index = self
            .file_picker_selected
            .min(filtered.len().saturating_sub(1));

        let selected = filtered
            .get(selected_index)
            .map(|(_, entry)| (*entry).clone());
        drop(filtered);
        self.file_picker_selected = selected_index;

        let Some(entry) = selected else {
            self.file_picker_preview_title = "No selection".to_string();
            self.file_picker_preview_lines = vec!["Use ↑/↓ to pick a file.".to_string()];
            return;
        };

        match entry.kind {
            FilePickerEntryKind::Parent => {
                self.file_picker_preview_title = "Parent directory".to_string();
                self.file_picker_preview_lines = vec![
                    format!(
                        "Open: {}",
                        display_path_for_workspace(&entry.path, &self.workspace_dir)
                    ),
                    "Press Enter to navigate up.".to_string(),
                ];
            }
            FilePickerEntryKind::Directory => {
                self.file_picker_preview_title = format!("Directory: {}", entry.name);
                let mut preview = vec![format!(
                    "Path: {}",
                    display_path_for_workspace(&entry.path, &self.workspace_dir)
                )];

                match std::fs::read_dir(&entry.path) {
                    Ok(read_dir) => {
                        let mut names = Vec::new();
                        let mut total = 0_usize;
                        for child in read_dir.flatten() {
                            let child_name = child.file_name().to_string_lossy().to_string();
                            if child_name.is_empty() || should_skip_workspace_entry(&child_name) {
                                continue;
                            }

                            total += 1;
                            if names.len() < FILE_PICKER_PREVIEW_DIR_ITEMS {
                                let suffix = match child.file_type() {
                                    Ok(ft) if ft.is_dir() => "/",
                                    _ => "",
                                };
                                names.push(format!("{child_name}{suffix}"));
                            }
                        }

                        preview.push(format!("Items: {total}"));
                        if names.is_empty() {
                            preview.push("(empty directory)".to_string());
                        } else {
                            preview.push("".to_string());
                            preview.extend(names.into_iter().map(|name| format!("• {name}")));
                            let extra = total.saturating_sub(FILE_PICKER_PREVIEW_DIR_ITEMS);
                            if extra > 0 {
                                preview.push(format!("... and {extra} more"));
                            }
                        }
                        preview.push("".to_string());
                        preview.push("Press Enter to open this folder.".to_string());
                    }
                    Err(err) => {
                        preview.push(format!("Failed to read directory: {err}"));
                    }
                }

                self.file_picker_preview_lines = preview;
            }
            FilePickerEntryKind::File => {
                self.file_picker_preview_title = format!("File: {}", entry.name);
                let mut preview = vec![format!(
                    "Path: {}",
                    display_path_for_workspace(&entry.path, &self.workspace_dir)
                )];
                if let Some(size) = entry.size_bytes {
                    preview.push(format!("Size: {}", format_bytes(size)));
                }
                preview.push("".to_string());

                match read_file_preview_lines(
                    &entry.path,
                    FILE_PICKER_PREVIEW_MAX_BYTES,
                    FILE_PICKER_PREVIEW_MAX_LINES,
                ) {
                    Ok((lines, truncated, binary)) => {
                        if binary {
                            preview.push("Binary file detected.".to_string());
                            preview.push("Enter attaches metadata only.".to_string());
                        } else {
                            preview.push("Preview:".to_string());
                            preview.extend(lines.into_iter().map(|line| format!("  {line}")));
                            if truncated {
                                preview.push("".to_string());
                                preview.push(format!(
                                    "(preview truncated to {} bytes / {} lines)",
                                    FILE_PICKER_PREVIEW_MAX_BYTES, FILE_PICKER_PREVIEW_MAX_LINES
                                ));
                            }
                        }
                    }
                    Err(err) => preview.push(format!("Failed to read file: {err}")),
                }

                self.file_picker_preview_lines = preview;
            }
        }
    }

    fn select_file_picker_entry(&mut self) {
        let selected = self
            .filtered_file_picker_entries()
            .get(self.file_picker_selected)
            .map(|(_, entry)| (*entry).clone());

        let Some(entry) = selected else {
            return;
        };

        match entry.kind {
            FilePickerEntryKind::Parent | FilePickerEntryKind::Directory => {
                self.file_picker_dir = entry.path;
                self.file_picker_filter.clear();
                self.file_picker_selected = 0;
                if let Err(err) = self.reload_file_picker_entries() {
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!("Failed to open directory: {}", err),
                    ));
                    self.scroll = SCROLL_BOTTOM;
                }
            }
            FilePickerEntryKind::File => {
                self.attach_file_to_input(&entry.path);
            }
        }
    }

    fn attach_file_to_input(&mut self, path: &Path) {
        let file_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.workspace_dir.join(path)
        };

        if !file_path.exists() {
            self.messages.push(ChatMessage::new(
                "system",
                format!("File not found: {}", file_path.display()),
            ));
            self.scroll = SCROLL_BOTTOM;
            return;
        }

        if !file_path.is_file() {
            self.messages.push(ChatMessage::new(
                "system",
                format!("Not a file: {}", file_path.display()),
            ));
            self.scroll = SCROLL_BOTTOM;
            return;
        }

        let display_path = display_path_for_workspace(&file_path, &self.workspace_dir);
        match build_file_share_snippet(&file_path, &display_path, FILE_SHARE_MAX_BYTES) {
            Ok((snippet, truncated, binary)) => {
                if !self.input.trim().is_empty() {
                    self.input.push_str("\n\n");
                }
                self.input.push_str(&snippet);
                self.cursor_position = self.input.len();
                self.view_mode = ViewMode::Chat;

                let suffix = if binary {
                    " (binary file metadata only)".to_string()
                } else if truncated {
                    format!(" (truncated to {} bytes)", FILE_SHARE_MAX_BYTES)
                } else {
                    String::new()
                };

                self.messages.push(ChatMessage::new(
                    "system",
                    format!("Attached `{display_path}` to composer{suffix}. Press Enter to send.",),
                ));
                self.scroll = SCROLL_BOTTOM;
            }
            Err(err) => {
                self.messages.push(ChatMessage::new(
                    "system",
                    format!("Failed to attach file {}: {}", file_path.display(), err),
                ));
                self.scroll = SCROLL_BOTTOM;
            }
        }
    }

    fn navigate_history(&mut self, direction: isize) {
        if self.command_history.is_empty() {
            return;
        }

        let history_len = self.command_history.len();
        let new_index = match self.history_index {
            Some(current) => {
                let new = current as isize + direction;
                if new < 0 {
                    None
                } else if new >= history_len as isize {
                    Some(history_len - 1)
                } else {
                    Some(new as usize)
                }
            }
            None => {
                if direction > 0 {
                    Some(0)
                } else {
                    Some(history_len.saturating_sub(1))
                }
            }
        };

        self.history_index = new_index;
        if let Some(index) = new_index {
            self.input = self.command_history[index].clone();
            self.cursor_position = self.input.len();
        } else {
            self.input.clear();
            self.cursor_position = 0;
        }
    }

    fn search_history(&mut self) {
        // Enhanced search: find commands matching current input prefix
        if self.command_history.is_empty() {
            return;
        }

        let search_term = self.input.trim().to_lowercase();

        if search_term.is_empty() {
            // Empty search - show most recent
            if !self.command_history.is_empty() {
                self.input = self.command_history.last().unwrap().clone();
                self.cursor_position = self.input.len();
                self.history_index = Some(self.command_history.len() - 1);
            }
            return;
        }

        // Find the most recent command that starts with the search term
        for (index, cmd) in self.command_history.iter().enumerate().rev() {
            if cmd.to_lowercase().starts_with(&search_term) {
                self.input = cmd.clone();
                self.cursor_position = self.input.len();
                self.history_index = Some(index);
                return;
            }
        }

        // If no prefix match, search for contains
        for (index, cmd) in self.command_history.iter().enumerate().rev() {
            if cmd.to_lowercase().contains(&search_term) {
                self.input = cmd.clone();
                self.cursor_position = self.input.len();
                self.history_index = Some(index);
                return;
            }
        }
    }

    fn autochat_status_label(&self) -> Option<String> {
        if !self.autochat_running {
            return None;
        }

        let elapsed = self
            .autochat_started_at
            .map(|started| {
                let elapsed = started.elapsed();
                if elapsed.as_secs() >= 60 {
                    format!("{}m{:02}s", elapsed.as_secs() / 60, elapsed.as_secs() % 60)
                } else {
                    format!("{:.1}s", elapsed.as_secs_f64())
                }
            })
            .unwrap_or_else(|| "0.0s".to_string());

        let phase = self
            .autochat_status
            .as_deref()
            .unwrap_or("Relay is running…")
            .to_string();

        Some(format!(
            "{} Autochat {elapsed} • {phase}",
            current_spinner_frame()
        ))
    }

    fn chat_sync_summary(&self) -> String {
        if self.chat_sync_rx.is_none() && self.chat_sync_status.is_none() {
            if self.secure_environment {
                return "Remote sync: REQUIRED in secure environment (not running)".to_string();
            }
            return "Remote sync: disabled (set CODETETHER_CHAT_SYNC_ENABLED=true)".to_string();
        }

        let status = self
            .chat_sync_status
            .as_deref()
            .unwrap_or("Remote sync active")
            .to_string();
        let last_success = self
            .chat_sync_last_success
            .as_deref()
            .unwrap_or("never")
            .to_string();
        let last_error = self
            .chat_sync_last_error
            .as_deref()
            .unwrap_or("none")
            .to_string();

        format!(
            "Remote sync: {status}\nUploaded batches: {} ({})\nLast success: {last_success}\nLast error: {last_error}",
            self.chat_sync_uploaded_batches,
            format_bytes(self.chat_sync_uploaded_bytes)
        )
    }

    fn handle_chat_sync_event(&mut self, event: ChatSyncUiEvent) {
        match event {
            ChatSyncUiEvent::Status(status) => {
                self.chat_sync_status = Some(status);
            }
            ChatSyncUiEvent::BatchUploaded {
                bytes,
                records,
                object_key,
            } => {
                self.chat_sync_uploaded_bytes = self.chat_sync_uploaded_bytes.saturating_add(bytes);
                self.chat_sync_uploaded_batches = self.chat_sync_uploaded_batches.saturating_add(1);
                let when = chrono::Local::now().format("%H:%M:%S").to_string();
                self.chat_sync_last_success = Some(format!(
                    "{} • {} records • {} • {}",
                    when,
                    records,
                    format_bytes(bytes),
                    object_key
                ));
                self.chat_sync_last_error = None;
                self.chat_sync_status =
                    Some(format!("Synced {} ({})", records, format_bytes(bytes)));
            }
            ChatSyncUiEvent::Error(error) => {
                self.chat_sync_last_error = Some(error.clone());
                self.chat_sync_status = Some("Sync error (will retry)".to_string());
            }
        }
    }

    fn to_archive_record(
        message: &ChatMessage,
        workspace: &str,
        session_id: Option<String>,
    ) -> ChatArchiveRecord {
        let (message_type, tool_name, tool_success, tool_duration_ms) = match &message.message_type
        {
            MessageType::Text(_) => ("text".to_string(), None, None, None),
            MessageType::Image { .. } => ("image".to_string(), None, None, None),
            MessageType::ToolCall { name, .. } => {
                ("tool_call".to_string(), Some(name.clone()), None, None)
            }
            MessageType::ToolResult {
                name,
                success,
                duration_ms,
                ..
            } => (
                "tool_result".to_string(),
                Some(name.clone()),
                Some(*success),
                *duration_ms,
            ),
            MessageType::File { .. } => ("file".to_string(), None, None, None),
            MessageType::Thinking(_) => ("thinking".to_string(), None, None, None),
        };

        ChatArchiveRecord {
            recorded_at: chrono::Utc::now().to_rfc3339(),
            workspace: workspace.to_string(),
            session_id,
            role: message.role.clone(),
            agent_name: message.agent_name.clone(),
            message_type,
            content: message.content.clone(),
            tool_name,
            tool_success,
            tool_duration_ms,
        }
    }

    /// Return cached message lines, rebuilding only when content has changed.
    ///
    /// `build_message_lines` allocates heavily (hundreds of `Line<'static>` per call)
    /// and calls `syntect` for code blocks.  Rebuilding on every 50 ms tick was the
    /// primary cause of CPU churn / apparent "freeze" during long tool sessions.
    fn get_or_build_message_lines(
        &mut self,
        theme: &Theme,
        max_width: usize,
    ) -> &[ratatui::text::Line<'static>] {
        let streaming_snapshot = self.streaming_text.clone();
        // Also track whether any agent streaming texts are active (relay mode)
        let has_agent_streams = !self.streaming_agent_texts.is_empty();
        let needs_rebuild = self.cached_messages_len != self.messages.len()
            || self.cached_max_width != max_width
            || self.cached_streaming_snapshot != streaming_snapshot
            || self.cached_processing != self.is_processing
            || self.cached_autochat_running != self.autochat_running
            // Rebuild when agent streams start or stop (content changes every tick while streaming)
            || (has_agent_streams || self.is_processing);

        if needs_rebuild {
            self.cached_message_lines = build_message_lines(self, theme, max_width);
            self.cached_messages_len = self.messages.len();
            self.cached_max_width = max_width;
            self.cached_streaming_snapshot = streaming_snapshot;
            self.cached_processing = self.is_processing;
            self.cached_autochat_running = self.autochat_running;
        }

        &self.cached_message_lines
    }

    fn flush_chat_archive(&mut self) {
        let Some(path) = self.chat_archive_path.clone() else {
            self.archived_message_count = self.messages.len();
            return;
        };

        if self.archived_message_count >= self.messages.len() {
            return;
        }

        let workspace = self.workspace_dir.to_string_lossy().to_string();
        let session_id = self.session.as_ref().map(|session| session.id.clone());
        let records: Vec<ChatArchiveRecord> = self.messages[self.archived_message_count..]
            .iter()
            .map(|message| Self::to_archive_record(message, &workspace, session_id.clone()))
            .collect();

        if let Some(parent) = path.parent()
            && let Err(err) = std::fs::create_dir_all(parent)
        {
            tracing::warn!(error = %err, path = %parent.display(), "Failed to create chat archive directory");
            return;
        }

        let mut file = match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
        {
            Ok(file) => file,
            Err(err) => {
                tracing::warn!(error = %err, path = %path.display(), "Failed to open chat archive file");
                return;
            }
        };

        for record in records {
            if let Err(err) = serde_json::to_writer(&mut file, &record) {
                tracing::warn!(error = %err, path = %path.display(), "Failed to serialize chat archive record");
                return;
            }
            if let Err(err) = writeln!(&mut file) {
                tracing::warn!(error = %err, path = %path.display(), "Failed to write chat archive newline");
                return;
            }
        }

        self.archived_message_count = self.messages.len();
    }
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
