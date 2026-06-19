//! Inner select!-based loop body.

use futures::StreamExt;

#[path = "dirty.rs"]
mod dirty;
#[path = "queued_prompt.rs"]
mod queued_prompt;

pub(super) async fn select_once(args: &mut super::SelectArgs<'_>) -> anyhow::Result<bool> {
    let before = dirty::Snapshot::capture(args.app);
    let app = &mut *args.app;
    let slot = &mut *args.slot;
    let bridge = &mut *args.worker_bridge;
    tokio::select! {
        // Biased: always poll terminal input FIRST so navigation/scroll keys
        // stay responsive while a session is actively streaming. Without this,
        // tokio picks a ready branch at random and the high-volume session
        // event stream starves keypresses, making the TUI feel frozen.
        biased;
        maybe = args.reader.next() => {
            let quit = super::terminal::handle_terminal_event(app, args.cwd, slot, args.registry, bridge, args.runtime, maybe).await?;
            app.state.needs_redraw = true;
            if quit { return Ok(true); }
        }
        Some(()) = args.io.shutdown_rx.recv() => return Ok(true),
        Some(evt) = args.io.event_rx.recv() => {
            crate::tui::app::session_events::handle_session_event(app, slot, bridge, evt).await;
            super::coalesce::drain(app, slot, bridge, args.io.event_rx).await;
        }
        Some(notice) = args.io.notice_rx.recv() => {
            crate::tui::app::background::apply_single_notice(app, args.cwd, slot, bridge, args.io.event_rx, notice).await;
            super::smart_retry::execute_smart_switch_retry(app, slot, args.registry, args.runtime).await;
            super::watchdog_retry::execute(app, slot, args.registry, args.runtime).await;
        }
        _ = args.timers.watchdog.tick() => super::watchdog::maybe_watchdog_restart(app, args.runtime, args.timers.watchdog_interval).await,
        _ = args.timers.tick.tick() => {
            super::tick::run(app).await;
            super::tick::check(app, args.runtime, args.timers.watchdog_interval).await;
        }
    }
    // Drain background updates before marking dirty (they may produce new state).
    crate::tui::app::background::drain_background_updates(
        app,
        args.cwd,
        slot,
        args.bus_handle,
        bridge,
        args.io.event_rx,
        args.io.notice_rx,
    )
    .await;
    queued_prompt::drain(app, args.cwd, slot, args.registry, bridge, args.runtime).await;
    super::bus_inbox::trigger_next(app, args.cwd, slot, args.registry, bridge, args.runtime).await;
    app.state.needs_redraw |= before.changed_since(app);
    Ok(false)
}
