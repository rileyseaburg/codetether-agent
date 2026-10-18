//! Native notice application and ordered reconnect/retry actions.
use super::super::SelectArgs;
use crate::tui::app::session_runtime::SessionNotice;
pub(super) async fn apply(args: &mut SelectArgs<'_>, notice: SessionNotice) {
    crate::tui::app::background::apply_single_notice(
        args.app,
        args.cwd,
        args.slot,
        args.worker_bridge,
        args.io.event_rx,
        notice,
    )
    .await;
    crate::tui::app::background::stream_reconnect_execute::execute(
        args.app,
        args.slot,
        args.registry,
        args.runtime,
    )
    .await;
    super::super::smart_retry::execute_smart_switch_retry(
        args.app,
        args.slot,
        args.registry,
        args.runtime,
    )
    .await;
    super::super::watchdog_retry::execute(args.app, args.slot, args.registry, args.runtime).await;
}
