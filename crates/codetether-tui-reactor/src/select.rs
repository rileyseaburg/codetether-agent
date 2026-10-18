//! Biased event selection preserving terminal responsiveness.
use crate::{Ready, Sources};
use futures::{Stream, StreamExt};
/// Select one ready event, polling terminal input first.
/// # Arguments
/// * `sources` - Borrowed stream, channels and timer intervals.
/// # Returns
/// One owned event. Closed channels are disabled for this selection; terminal
/// end-of-stream remains an event for the application to handle.
/// # Examples
/// ```no_run
/// async fn wait<S: futures::Stream + Unpin>(sources: &mut codetether_tui_reactor::Sources<'_, S, (), ()>) {
///     let _event = codetether_tui_reactor::select(sources).await;
/// }
/// ```
pub async fn select<S: Stream + Unpin, E, N>(
    sources: &mut Sources<'_, S, E, N>,
) -> Ready<S::Item, E, N> {
    tokio::select! {
        // Poll terminal input first so ready session events cannot starve it.
        biased;
        maybe = sources.reader.next() => Ready::Terminal(maybe),
        Some(()) = sources.shutdown.recv() => Ready::Shutdown,
        Some(event) = sources.events.recv() => Ready::Session(event),
        Some(notice) = sources.notices.recv() => Ready::Notice(notice),
        _ = sources.timers.watchdog.tick() => Ready::Watchdog,
        _ = sources.timers.audit.tick() => Ready::Audit,
        _ = sources.timers.tick.tick() => Ready::Tick,
    }
}
