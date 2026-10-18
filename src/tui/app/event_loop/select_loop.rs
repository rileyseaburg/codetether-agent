//! Native dispatch and redraw tracking around shared event selection.
use codetether_tui_reactor::Ready;
#[path = "dirty.rs"]
mod dirty;
#[path = "select_loop/events.rs"]
mod events;
#[path = "select_loop/notice.rs"]
mod notice;
#[path = "queued_prompt.rs"]
mod queued_prompt;
#[path = "select_loop/ticks.rs"]
mod ticks;
#[path = "select_loop/wait.rs"]
mod wait;
pub(super) async fn select_once(args: &mut super::SelectArgs<'_>) -> anyhow::Result<bool> {
    let before = dirty::capture(args.app);
    match wait::next(args).await {
        Ready::Terminal(event) => {
            if events::terminal(args, event).await? {
                return Ok(true);
            }
        }
        Ready::Shutdown => return Ok(true),
        Ready::Session(event) => events::session(args, event).await,
        Ready::Notice(event) => notice::apply(args, event).await,
        Ready::Watchdog => ticks::watchdog(args).await,
        Ready::Audit => super::tick::refresh_audit(args.app).await,
        Ready::Tick => ticks::apply(args).await,
    }
    let app = &mut *args.app;
    let slot = &mut *args.slot;
    let bridge = &mut *args.worker_bridge;
    // Drain background updates before marking dirty (they may produce new state).
    crate::tui::app::background::drain_background_updates(
        app,
        args.cwd,
        slot,
        args.bus_recorder,
        bridge,
        args.io.event_rx,
        args.io.notice_rx,
    )
    .await;
    queued_prompt::drain(app, args.cwd, slot, args.registry, bridge, args.runtime).await;
    super::bus_inbox::trigger_next(app, args.cwd, slot, args.registry, bridge, args.runtime).await;
    app.state.needs_redraw |= before != dirty::capture(app);
    Ok(false)
}
