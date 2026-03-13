use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use crossterm::event::{Event, EventStream};
use futures::StreamExt;
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;

use crate::bus::BusHandle;
use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::background::drain_background_updates;
use crate::tui::app::event_handlers::handle_event;
use crate::tui::app::state::App;
use crate::tui::app::worker_bridge::sync_worker_bridge_agents;
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

    loop {
        // Draw first so the latest state is always visible.
        sync_worker_bridge_agents(app, &worker_bridge);
        terminal.draw(|f| ui(f, app, session))?;

        // Wait for whichever fires first: terminal event, session event,
        // completed result, or a 50 ms tick (for bus / worker polling).
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
            }

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
