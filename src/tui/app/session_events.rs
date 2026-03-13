use crate::session::{Session, SessionEvent};
use crate::tui::app::state::App;
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
    match evt {
        SessionEvent::Thinking => {
            handle_processing_started(app, worker_bridge).await;
            app.state.processing_started_at = Some(std::time::Instant::now());
            app.state.status = "Thinking…".to_string();
        }
        SessionEvent::ToolCallStart { name, arguments } => {
            handle_processing_started(app, worker_bridge).await;
            if app.state.processing_started_at.is_none() {
                app.state.processing_started_at = Some(std::time::Instant::now());
            }
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
        } => {
            app.state.messages.push(ChatMessage::new(
                MessageType::ToolResult {
                    name: name.clone(),
                    output: output.clone(),
                    success,
                    duration_ms: None,
                },
                format!("{name}: {}", truncate_preview(&output, 600)),
            ));
            app.state.status = format!("Tool finished: {name}");
            app.state.scroll_to_bottom();
        }
        SessionEvent::TextChunk(chunk) => {
            app.state.streaming_text.push_str(&chunk);
        }
        SessionEvent::TextComplete(text) => {
            app.state.streaming_text.clear();
            app.state
                .messages
                .push(ChatMessage::new(MessageType::Assistant, text));
            app.state.status = "Assistant replied".to_string();
            app.state.scroll_to_bottom();
        }
        SessionEvent::ThinkingComplete(text) => {
            if !text.is_empty() {
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
            app.state.processing_started_at = None;
            app.state.status = "Ready".to_string();
        }
        SessionEvent::Error(err) => {
            handle_processing_stopped(app, worker_bridge).await;
            app.state.streaming_text.clear();
            app.state.processing_started_at = None;
            app.state
                .messages
                .push(ChatMessage::new(MessageType::Error, err.clone()));
            app.state.status = "Error".to_string();
            app.state.scroll_to_bottom();
        }
    }
}
