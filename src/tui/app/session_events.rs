use crate::session::{Session, SessionEvent};
use crate::tui::app::smart_switch::maybe_schedule_smart_switch_retry;
use crate::tui::app::state::App;
use crate::tui::app::smart_switch::smart_switch_model_key;
use crate::tui::app::text::truncate_preview;
use crate::tui::app::worker_bridge::{handle_processing_started, handle_processing_stopped};
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::worker_bridge::TuiWorkerBridge;

pub async fn handle_session_event(
    app: &mut App,
    session: &mut Session,
    worker_bridge: &Option<TuiWorkerBridge>,
    evt: SessionEvent,
) {
    // Update watchdog timestamp on every session event
    app.state.main_last_event_at = Some(std::time::Instant::now());

    match evt {
        SessionEvent::Thinking => {
            handle_processing_started(app, worker_bridge).await;
            if app.state.processing_started_at.is_none() {
                app.state.begin_request_timing();
            }
            app.state.status = "Thinking…".to_string();
        }
        SessionEvent::ToolCallStart { name, arguments } => {
            handle_processing_started(app, worker_bridge).await;
            if app.state.processing_started_at.is_none() {
                app.state.begin_request_timing();
            }
            app.state.reset_tool_preview_scroll();
            app.state.status = format!("Running tool: {name}");
            app.state.messages.push(ChatMessage::new(
                MessageType::ToolCall {
                    name: name.clone(),
                    arguments: arguments.clone(),
                },
                format!("{name}: {}", truncate_preview(&arguments, 240)),
            ));
            app.state.scroll_to_bottom();
        }
        SessionEvent::ToolCallComplete {
            name,
            output,
            success,
            duration_ms,
        } => {
            app.state.reset_tool_preview_scroll();
            app.state.messages.push(ChatMessage::new(
                MessageType::ToolResult {
                    name: name.clone(),
                    output: output.clone(),
                    success,
                    duration_ms: Some(duration_ms),
                },
                format!("{name}: {}", truncate_preview(&output, 600)),
            ));
            app.state.last_tool_name = Some(name.clone());
            app.state.last_tool_latency_ms = Some(duration_ms);
            app.state.last_tool_success = Some(success);
            app.state.status = format!("Tool finished: {name}");
            app.state.scroll_to_bottom();
        }
        SessionEvent::TextChunk(chunk) => {
            app.state.note_text_token();
            app.state.streaming_text = chunk;
        }
        SessionEvent::TextComplete(text) => {
            app.state.note_text_token();
            app.state.streaming_text.clear();
            app.state
                .messages
                .push(ChatMessage::new(MessageType::Assistant, text));
            app.state.status = "Assistant replied".to_string();
            app.state.scroll_to_bottom();
        }
        SessionEvent::ThinkingComplete(text) => {
            if !text.is_empty() {
                app.state.reset_tool_preview_scroll();
                app.state.messages.push(ChatMessage::new(
                    MessageType::Thinking(text.clone()),
                    truncate_preview(&text, 600),
                ));
                app.state.scroll_to_bottom();
            }
        }
        SessionEvent::UsageReport {
            model,
            prompt_tokens,
            completion_tokens,
            duration_ms,
        } => {
            app.state.last_completion_model = Some(model.clone());
            app.state.last_completion_latency_ms = Some(duration_ms);
            app.state.last_completion_prompt_tokens = Some(prompt_tokens);
            app.state.last_completion_output_tokens = Some(completion_tokens);
            app.state.status = format!(
                "Completed with model {model} • {} in / {} out • {} ms",
                prompt_tokens, completion_tokens, duration_ms
            );
        }
        SessionEvent::SessionSync(updated) => {
            *session = *updated;
            app.state.session_id = Some(session.id.clone());
        }
        SessionEvent::Done => {
            handle_processing_stopped(app, worker_bridge).await;
            app.state.streaming_text.clear();
            app.state.complete_request_timing();
            app.state.status = "Ready".to_string();
        }
        SessionEvent::Error(err) => {
            handle_processing_stopped(app, worker_bridge).await;
            app.state.streaming_text.clear();
            app.state.complete_request_timing();

            // Attempt smart switch retry on retryable provider errors
            let current_model = session.metadata.model.as_deref();
            let current_provider = current_model.and_then(|m| m.split('/').next());
            let prompt = app.state.main_inflight_prompt.clone().unwrap_or_default();

            if let Some(pending) = maybe_schedule_smart_switch_retry(
                &err,
                current_model,
                current_provider,
                &app.state.available_models,
                &prompt,
                app.state.smart_switch_retry_count,
                &app.state.smart_switch_attempted_models,
            ) {
                app.state.smart_switch_retry_count += 1;
                app
                    .state
                    .smart_switch_attempted_models
                    .push(current_model.unwrap_or("unknown").to_string());
                app.state.smart_switch_attempted_models.push(pending.target_model.clone());
                app.state.status = format!(
                    "Smart switch retry {}/{} → {}",
                    app.state.smart_switch_retry_count,
                    smart_switch_max_retries(),
                    pending.target_model,
                );
                app.state.pending_smart_switch_retry = Some(pending);
            } else {
                // No retry possible — reset smart switch state
                app.state.smart_switch_retry_count = 0;
                app.state.smart_switch_attempted_models.clear();
                app.state.pending_smart_switch_retry = None;
            }

            app.state
                .messages
                .push(ChatMessage::new(MessageType::Error, err.clone()));
            app.state.status = "Error".to_string();
            app.state.scroll_to_bottom();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::Session;
    use crate::tui::chat::message::MessageType;

    #[tokio::test]
    async fn text_chunk_replaces_streaming_preview_with_latest_cumulative_text() {
        let mut app = App::default();
        let mut session = Session::new().await.expect("session should create");

        handle_session_event(
            &mut app,
            &mut session,
            &None,
            SessionEvent::TextChunk("hel".to_string()),
        )
        .await;
        assert_eq!(app.state.streaming_text, "hel");

        handle_session_event(
            &mut app,
            &mut session,
            &None,
            SessionEvent::TextChunk("hello".to_string()),
        )
        .await;
        assert_eq!(app.state.streaming_text, "hello");
    }

    #[tokio::test]
    async fn tool_completion_records_duration_for_chat_and_latency_view() {
        let mut app = App::default();
        let mut session = Session::new().await.expect("session should create");

        handle_session_event(
            &mut app,
            &mut session,
            &None,
            SessionEvent::ToolCallComplete {
                name: "read".to_string(),
                output: "src/main.rs".to_string(),
                success: true,
                duration_ms: 42,
            },
        )
        .await;

        let Some(message) = app.state.messages.last() else {
            panic!("expected a tool result message");
        };
        match &message.message_type {
            MessageType::ToolResult {
                name,
                success,
                duration_ms,
                ..
            } => {
                assert_eq!(name, "read");
                assert!(*success);
                assert_eq!(*duration_ms, Some(42));
            }
            other => panic!("expected tool result message, got {other:?}"),
        }
        assert_eq!(app.state.last_tool_name.as_deref(), Some("read"));
        assert_eq!(app.state.last_tool_latency_ms, Some(42));
        assert_eq!(app.state.last_tool_success, Some(true));
    }

    #[tokio::test]
    async fn usage_report_updates_latency_snapshot() {
        let mut app = App::default();
        let mut session = Session::new().await.expect("session should create");

        handle_session_event(
            &mut app,
            &mut session,
            &None,
            SessionEvent::UsageReport {
                model: "openai/gpt-5.4".to_string(),
                prompt_tokens: 120,
                completion_tokens: 64,
                duration_ms: 1_250,
            },
        )
        .await;

        assert_eq!(
            app.state.last_completion_model.as_deref(),
            Some("openai/gpt-5.4")
        );
        assert_eq!(app.state.last_completion_latency_ms, Some(1_250));
        assert_eq!(app.state.last_completion_prompt_tokens, Some(120));
        assert_eq!(app.state.last_completion_output_tokens, Some(64));
    }

    #[tokio::test]
    async fn text_events_record_request_ttft_and_last_token() {
        let mut app = App::default();
        let mut session = Session::new().await.expect("session should create");
        app.state.processing_started_at =
            Some(std::time::Instant::now() - std::time::Duration::from_millis(15));

        handle_session_event(
            &mut app,
            &mut session,
            &None,
            SessionEvent::TextChunk("hello".to_string()),
        )
        .await;

        let first = app
            .state
            .current_request_first_token_ms
            .expect("expected ttft after first chunk");
        assert_eq!(app.state.current_request_last_token_ms, Some(first));

        app.state.processing_started_at =
            Some(std::time::Instant::now() - std::time::Duration::from_millis(30));
        handle_session_event(
            &mut app,
            &mut session,
            &None,
            SessionEvent::TextComplete("hello".to_string()),
        )
        .await;

        assert_eq!(app.state.current_request_first_token_ms, Some(first));
        assert!(
            app.state
                .current_request_last_token_ms
                .expect("expected last token timing")
                >= first
        );
    }

    #[tokio::test]
    async fn done_promotes_request_timing_snapshot() {
        let mut app = App::default();
        let mut session = Session::new().await.expect("session should create");
        app.state.processing_started_at = Some(std::time::Instant::now());
        app.state.current_request_first_token_ms = Some(120);
        app.state.current_request_last_token_ms = Some(980);

        handle_session_event(&mut app, &mut session, &None, SessionEvent::Done).await;

        assert_eq!(app.state.last_request_first_token_ms, Some(120));
        assert_eq!(app.state.last_request_last_token_ms, Some(980));
        assert!(app.state.processing_started_at.is_none());
        assert!(app.state.current_request_first_token_ms.is_none());
        assert!(app.state.current_request_last_token_ms.is_none());
    }
}