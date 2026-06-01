//! Inner select!-based loop body.

use futures::StreamExt;

#[path = "dirty.rs"]
mod dirty;

pub(super) async fn select_once(args: &mut super::SelectArgs<'_>) -> anyhow::Result<bool> {
    let before = dirty::Snapshot::capture(args.app);
    let app = &mut *args.app;
    let session = &mut *args.session;
    let bridge = &mut *args.worker_bridge;
    tokio::select! {
        maybe = args.reader.next() => {
            let quit = super::terminal::handle_terminal_event(app, args.cwd, session, args.registry, bridge, args.event_tx, args.result_tx, maybe).await?;
            app.state.needs_redraw = true;
            if quit { return Ok(true); }
        }
        Some(evt) = args.io.event_rx.recv() => {
            crate::tui::app::session_events::handle_session_event(app, session, bridge, evt).await;
            super::coalesce::drain(app, session, bridge, args.io.event_rx).await;
        }
        Some(result) = args.io.result_rx.recv() => {
            crate::tui::app::background::apply_single_result(app, args.cwd, session, bridge, result).await;
            super::smart_retry::execute_smart_switch_retry(app, session, args.registry, args.event_tx, args.result_tx).await;
        }
        Some(()) = args.io.shutdown_rx.recv() => return Ok(true),
        _ = args.timers.watchdog.tick() => super::watchdog::maybe_watchdog_restart(app, session, args.registry, args.event_tx, args.result_tx, args.timers.watchdog_interval).await,
        _ = args.timers.tick.tick() => super::tick::run(app).await,
    }
    // Drain background updates before marking dirty (they may produce new state).
    crate::tui::app::background::drain_background_updates(
        app,
        args.cwd,
        session,
        args.bus_handle,
        bridge,
        args.io.event_rx,
        args.io.result_rx,
    )
    .await;
    super::bus_inbox::trigger_next(
        app,
        args.cwd,
        session,
        args.registry,
        bridge,
        args.event_tx,
        args.result_tx,
    )
    .await;
    app.state.needs_redraw |= before.changed_since(app);
    Ok(false)
}
