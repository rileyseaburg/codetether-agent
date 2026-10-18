//! Ordered event branches, short-circuit quit and final redraw tracking.
use super::Host;
use crate::Ready;
/// Handle one event and then drain work before updating redraw state.
/// # Arguments
/// * `host` - Existing application handlers and state.
/// * `event` - Event selected by the reactor.
/// * `before` - Visible state captured before waiting.
/// # Returns
/// True for shutdown or a terminal quit request; false after normal processing.
/// # Errors
/// Propagates terminal handling failures before post-event draining.
/// # Examples
/// ```no_run
/// async fn handle<H: codetether_tui_reactor::dispatch::Host>(host: &mut H, event: codetether_tui_reactor::Ready<H::Terminal, H::Session, H::Notice>, before: H::Snapshot) -> anyhow::Result<bool> {
///     codetether_tui_reactor::dispatch::dispatch(host, event, before).await
/// }
/// ```
pub async fn dispatch<H: Host>(
    host: &mut H,
    event: Ready<H::Terminal, H::Session, H::Notice>,
    before: H::Snapshot,
) -> anyhow::Result<bool> {
    match event {
        Ready::Terminal(event) => {
            let quit = host.terminal(event).await?;
            *host.needs_redraw() = true;
            if quit {
                return Ok(true);
            }
        }
        Ready::Shutdown => return Ok(true),
        Ready::Session(event) => {
            host.session(event).await;
            host.coalesce().await;
        }
        Ready::Notice(notice) => {
            host.notice(notice).await;
            host.reconnect().await;
            host.smart_retry().await;
            host.watchdog_retry().await;
        }
        Ready::Watchdog => host.watchdog().await,
        Ready::Audit => host.audit().await,
        Ready::Tick => {
            host.tick().await;
            host.tick_retry().await;
        }
    }
    host.drain().await;
    host.queued().await;
    host.inbox().await;
    let changed = before != host.snapshot();
    *host.needs_redraw() |= changed;
    Ok(false)
}
