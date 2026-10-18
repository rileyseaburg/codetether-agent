//! Borrow native channels and terminal input for shared event selection.
use super::super::SelectArgs;
type Ready = codetether_tui_reactor::Ready<
    std::io::Result<crossterm::event::Event>,
    crate::session::SessionEvent,
    crate::tui::app::session_runtime::SessionNotice,
>;
pub(super) async fn next(args: &mut SelectArgs<'_>) -> Ready {
    codetether_tui_reactor::select(&mut codetether_tui_reactor::Sources {
        reader: &mut *args.reader,
        shutdown: &mut *args.io.shutdown_rx,
        events: &mut *args.io.event_rx,
        notices: &mut *args.io.notice_rx,
        timers: &mut *args.timers,
    })
    .await
}
