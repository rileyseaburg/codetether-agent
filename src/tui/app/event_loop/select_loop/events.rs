//! Native terminal and session-event dispatch after shared selection.
use super::super::SelectArgs;
pub(super) async fn terminal(
    args: &mut SelectArgs<'_>,
    event: Option<std::io::Result<crossterm::event::Event>>,
) -> anyhow::Result<bool> {
    let quit = super::super::terminal::handle_terminal_event(
        args.app,
        args.cwd,
        args.slot,
        args.registry,
        args.worker_bridge,
        args.runtime,
        event,
    )
    .await?;
    args.app.state.needs_redraw = true;
    Ok(quit)
}
pub(super) async fn session(args: &mut SelectArgs<'_>, event: crate::session::SessionEvent) {
    crate::tui::app::session_events::handle_session_event(
        args.app,
        args.slot,
        args.worker_bridge,
        event,
    )
    .await;
    super::super::coalesce::drain(args.app, args.slot, args.worker_bridge, args.io.event_rx).await;
}
