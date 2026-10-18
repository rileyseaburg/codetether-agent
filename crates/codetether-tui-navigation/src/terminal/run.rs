use super::Host;
use crossterm::event::Event;

/// Route a terminal stream item to the appropriate native input handler.
///
/// # Arguments
/// * `host` - Native input handlers and editor paste gate.
/// * `event` - Terminal event, stream error or end-of-stream marker.
/// # Returns
/// Whether the keyboard handler requested exit. Other events return `false`.
/// # Errors
/// Propagates the keyboard handler's error without changing it.
/// # Examples
/// ```text
/// let quit = terminal::dispatch(&mut native_host, reader.next().await).await?;
/// ```
pub async fn dispatch<H: Host>(
    host: &mut H,
    event: Option<Result<Event, std::io::Error>>,
) -> Result<bool, H::Error> {
    match event {
        Some(Ok(Event::Key(key))) => host.key(key).await,
        Some(Ok(Event::Paste(text))) => {
            if !host.editor_paste(&text) {
                host.paste(&text).await;
            }
            Ok(false)
        }
        Some(Ok(Event::Mouse(mouse))) => {
            host.mouse(mouse).await;
            Ok(false)
        }
        _ => Ok(false),
    }
}
