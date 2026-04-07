//! Main processing watchdog timer, protocol view, and agent picker helpers
//!
//! These are `impl App` methods extracted from the branch monolith.
//! Integration target: src/tui/app/impl_app.rs or new dedicated modules.


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
}
}
}
}
