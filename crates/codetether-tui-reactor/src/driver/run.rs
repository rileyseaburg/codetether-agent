//! Draw, process one event and repeat until quit.
use super::Host;
/// Drive the interactive loop and finish resources after a normal quit.
/// # Arguments
/// * `host` - Owned native loop state and callbacks.
/// # Returns
/// Success after the quit signal and shutdown callback.
/// # Errors
/// Returns draw failures immediately. Event-processing failures are logged and
/// followed by another iteration, matching the existing recovery policy.
/// # Examples
/// ```no_run
/// async fn drive<H: codetether_tui_reactor::driver::Host>(host: H) -> anyhow::Result<()> {
///     codetether_tui_reactor::driver::run(host).await
/// }
/// ```
pub async fn run<H: Host>(mut host: H) -> anyhow::Result<()> {
    loop {
        host.before_draw();
        host.update_mux().await;
        host.draw()?;
        if super::resilient::run_resilient(&mut host).await {
            break;
        }
    }
    host.finish().await;
    Ok(())
}
