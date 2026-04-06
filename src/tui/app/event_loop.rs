use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use crossterm::event::{Event, EventStream};
use futures::StreamExt;
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;

use crate::bus::BusHandle;
use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::background::drain_background_updates;
use crate::tui::app::event_handlers::{handle_event, handle_mouse_event, handle_paste_event};
use crate::tui::app::smart_switch::should_execute_smart_switch;
use crate::tui::app::state::App;
use crate::tui::app::worker_bridge::sync_worker_bridge_agents;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::chat::sync;
use crate::tui::constants::MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS;
use crate::tui::ui::main::ui;
use crate::tui::worker_bridge::TuiWorkerBridge;

pub async fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    cwd: &Path,
    registry: Option<Arc<ProviderRegistry>>,
    session: &mut Session,
    bus_handle: &mut BusHandle,
    mut worker_bridge: Option<TuiWorkerBridge>,
    event_tx: mpsc::Sender<SessionEvent>,
    mut event_rx: mpsc::Receiver<SessionEvent>,
    result_tx: mpsc::Sender<anyhow::Result<Session>>,
    mut result_rx: mpsc::Receiver<anyhow::Result<Session>>,
) -> anyhow::Result<()> {
    let mut reader = EventStream::new();
    let tick = Duration::from_millis(50);
    let watchdog_interval = Duration::from_secs(MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS);
    let mut watchdog_timer = tokio::time::interval(watchdog_interval);

    loop {
        // Draw first so the latest state is always visible.
        sync_worker_bridge_agents(app, &worker_bridge);
        terminal.draw(|f| ui(f, app, session))?;

        // Wait for whichever fires first: terminal event, session event,
        // completed result, watchdog timeout, or a 50 ms tick.
        tokio::select! {
            // Terminal key / resize events (async, no thread-blocking).
            maybe_event = reader.next() => {
                match maybe_event {
                    Some(Ok(Event::Key(key))) => {
                        if handle_event(
                            app, cwd, session, &registry, &worker_bridge,
                            &event_tx, &result_tx, key,
                        ).await? {
                            break;
                        }
                    }
                    Some(Ok(Event::Paste(text))) => {
                        handle_paste_event(app, &text).await;
                    }
                    Some(Ok(Event::Mouse(mouse))) => {
                        handle_mouse_event(app, mouse);
                    }
                    Some(Ok(Event::Resize(_, _))) => {}
                    Some(Err(_)) => {}
                    _ => {}
                }
            }

            // Session streaming events (text chunks, tool calls, Done, …).
            Some(evt) = event_rx.recv() => {
                crate::tui::app::session_events::handle_session_event(
                    app, session, &worker_bridge, evt,
                ).await;
            }

            // Completed session results from spawned prompt tasks.
            Some(result) = result_rx.recv() => {
                crate::tui::app::background::apply_single_result(
                    app, cwd, session, &mut worker_bridge, result,
                ).await;

                // Execute pending smart switch retry if one was scheduled
                if let Some(pending) = app.state.pending_smart_switch_retry.take() {
                    if should_execute_smart_switch(session.metadata.model.as_deref(), Some(&pending)) {
                        session.metadata.model = Some(pending.target_model.clone());
                        let _ = session.save().await;

                        app.state.processing = true;
                        app.state.begin_request_timing();
                        app.state.main_inflight_prompt = Some(pending.prompt.clone());
                        app.state.main_last_event_at = Some(Instant::now());
                        app.state.status = format!("Retrying with {}…", pending.target_model);

                        if let Some(registry) = registry.as_ref() {
                            let mut session_for_task = session.clone();
                            let event_tx = event_tx.clone();
                            let result_tx = result_tx.clone();
                            let reg = Arc::clone(registry);
                            let prompt = pending.prompt;
                            tokio::spawn(async move {
                                let result = session_for_task
                                    .prompt_with_events(&prompt, event_tx, reg)
                                    .await
                                    .map(|_| session_for_task);
                                let _ = result_tx.send(result).await;
                            });
                        }
                    }
                }
            }

            // Watchdog timer: detect stuck requests and auto-restart.
            _ = watchdog_timer.tick() => {
                if app.state.processing {
                    let timed_out = app.state.main_last_event_at
                        .map(|t| t.elapsed() >= watchdog_interval)
                        .unwrap_or(true);

                    if timed_out {
                        let prompt = app.state.main_watchdog_root_prompt.clone()
                            .or_else(|| app.state.main_inflight_prompt.clone());

                        if let Some(prompt) = prompt {
                            app.state.main_watchdog_restart_count += 1;
                            let count = app.state.main_watchdog_restart_count;

                            // Cancel the stuck request: mark processing as stopped,
                            // clear streaming state, and record the error.
                            app.state.processing = false;
                            app.state.streaming_text.clear();
                            app.state.clear_request_timing();
                            app.state.status = format!(
                                "Watchdog timeout — restarting request (attempt {count})"
                            );

                            app.state.messages.push(ChatMessage::new(
                                MessageType::Error,
                                format!(
                                    "⚠ Watchdog: no events for {MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS}s. \
                                     Auto-restarting (attempt {count})."
                                ),
                            ));
                            app.state.scroll_to_bottom();

                            // Re-send the original prompt.
                            if let Some(registry) = registry.as_ref() {
                                app.state.main_inflight_prompt = Some(prompt.clone());
                                app.state.processing = true;
                                app.state.begin_request_timing();
                                app.state.main_last_event_at = Some(Instant::now());

                                let mut session_for_task = session.clone();
                                let event_tx = event_tx.clone();
                                let result_tx = result_tx.clone();
                                let reg = Arc::clone(registry);
                                tokio::spawn(async move {
                                    let result = session_for_task
                                        .prompt_with_events(&prompt, event_tx, reg)
                                        .await
                                        .map(|_| session_for_task);
                                    let _ = result_tx.send(result).await;
                                });
                            }
                        }
                    }
                }
            }

            // Chat sync UI events from background worker.
            // TODO: implement chat_sync_rx handling once ChatSyncUiEvent is defined

            // Tick: drain bus messages & worker bridge state.
            _ = tokio::time::sleep(tick) => {}
        }

        // Non-blocking drain of any remaining bus / worker bridge messages
        // that arrived while we were handling the selected branch above.
        drain_background_updates(
            app,
            cwd,
            session,
            bus_handle,
            &mut worker_bridge,
            &mut event_rx,
            &mut result_rx,
        )
        .await;
    }

    if let Some(bridge) = worker_bridge.as_ref() {
        let _ = bridge.cmd_tx.try_send(
            crate::tui::worker_bridge::WorkerBridgeCmd::DeregisterAgent {
                name: "tui".to_string(),
            },
        );
    }

    Ok(())
}
