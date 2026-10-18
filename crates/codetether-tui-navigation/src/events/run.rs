use super::Host;
use crossterm::event::KeyEvent;
/// Dispatch one key through modal and shortcut priority layers.
/// # Arguments
/// * `host` - Native layer handlers and key-timing state.
/// * `key` - Terminal key event.
/// # Returns
/// Whether a handler requests quitting the UI.
/// # Errors
/// Propagates review or key-handler errors.
/// # Examples
/// ```text
/// events::dispatch(&mut native_host, key).await?;
/// ```
pub async fn dispatch<H: Host>(host: &mut H, key: KeyEvent) -> Result<bool, H::Error> {
    if !super::dispatchable(key) || host.question(key) {
        return Ok(false);
    }
    if host.editor(key).await || host.approval_scroll(key) {
        return Ok(false);
    }
    if host.okr(key).await {
        return Ok(false);
    }
    if let Some(resume) = host.review(key).await? {
        if resume {
            host.resume().await;
        }
        return Ok(false);
    }
    if let Some(quit) = host.priority(key).await {
        return Ok(quit);
    }
    if let Some(result) = host.modified(key) {
        return result;
    }
    let out = host.unmodified(key).await;
    *host.last_key() = Some(std::time::Instant::now());
    out
}
