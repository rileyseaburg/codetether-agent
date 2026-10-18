use super::Host;
use std::time::Instant;
/// Render a dirty frame when keyboard or streaming cadence permits.
/// # Arguments
/// Host supplies terminal and UI state; last_draw tracks the previous frame.
/// # Returns
/// Success after skipping or drawing a frame.
/// # Errors
/// Propagates native draw errors before clearing dirty state.
/// # Examples
/// ```ignore
/// draw_if_ready(&mut terminal_host, &mut last_draw)?;
/// ```
pub fn draw_if_ready<H: Host>(
    host: &mut H,
    last_draw: &mut Option<Instant>,
) -> Result<(), H::Error> {
    if !*host.needs_redraw() || !super::ready(host.processing(), host.last_input(), *last_draw) {
        return Ok(());
    }
    let rendered = host.draw()?;
    super::finish(host.needs_redraw(), last_draw, rendered);
    Ok(())
}
