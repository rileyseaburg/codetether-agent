//! Native periodic work after a shared timer selection.
use super::super::SelectArgs;
pub(super) async fn watchdog(args: &mut SelectArgs<'_>) {
    super::super::watchdog::maybe_watchdog_restart(
        args.app,
        args.runtime,
        args.timers.watchdog_interval,
    )
    .await;
}
pub(super) async fn apply(args: &mut SelectArgs<'_>) {
    super::super::tick::run(args.app).await;
    super::super::tick::check_and_retry(
        args.app,
        args.slot,
        args.registry,
        args.runtime,
        args.timers.watchdog_interval,
    )
    .await;
}
