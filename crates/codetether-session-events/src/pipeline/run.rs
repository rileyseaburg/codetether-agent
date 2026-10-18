//! Activity, ordered stage execution and final retention/dispatch.
use super::{Stages, View};
/// Deliver one event through the ordered frontend pipeline.
/// # Arguments
/// * `host` — Event stages and frontend presentation state.
/// * `event` — Owned event received from the runtime.
/// # Returns
/// Trims retained state after processing; dispatches any remaining event to context.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_session_events::pipeline::View>(host: &mut H, event: H::Event) {
/// codetether_session_events::pipeline::handle(host, event).await;
/// # }
/// ```
pub async fn handle<H: View>(host: &mut H, event: H::Event) {
    note_event(host);
    let Some(event) = run(host, event).await else {
        return host.trim();
    };
    host.trim();
    host.context(event);
}
async fn run<H: Stages>(host: &mut H, mut event: H::Event) -> Option<H::Event> {
    event = host.lifecycle(event).await?;
    event = host.interlude(event)?;
    event = host.retry(event)?;
    event = host.tools(event).await?;
    event = host.text(event)?;
    event = host.usage(event)?;
    host.errors(event).await
}
fn note_event(host: &mut impl View) {
    *host.last_event_at() = Some(std::time::Instant::now());
    if host.auto_follow() {
        host.scroll_to_bottom();
    }
}
