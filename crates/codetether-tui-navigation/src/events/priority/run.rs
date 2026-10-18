use super::Host;
use crossterm::event::KeyEvent;
/// Offer a key to interrupt and modal handlers in priority order.
/// # Arguments
/// * `host` - Native modal state and handlers.
/// * `key` - Key event to offer.
/// # Returns
/// A consumed/quit decision, or None to continue ordinary key dispatch.
/// # Examples
/// ```text
/// priority::handle(&mut native_host, key).await;
/// ```
pub async fn handle(host: &mut impl Host, key: KeyEvent) -> Option<bool> {
    if let Some(quit) = host.interrupt(key) {
        return Some(quit);
    }
    if host.interlude(key) || host.goal(key) {
        return Some(false);
    }
    if host.spawn_active() && host.spawn(key).await {
        return Some(false);
    }
    if host.fuzzy(key) {
        return Some(false);
    }
    if host.editor_lsp(key).await {
        return Some(false);
    }
    if host.editor(key) {
        return Some(false);
    }
    None
}
