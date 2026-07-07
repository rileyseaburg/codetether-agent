//! Time-animated sweeping gradient text.
//!
//! Like [`super::gradient::gradient_spans`] but the gradient phase shifts
//! with wall-clock time, so the color band visibly travels through the
//! text. Redraws are driven by the existing processing/animation ticks.

use ratatui::style::{Modifier, Style};
use ratatui::text::Span;

use super::gradient::{Rgb, lerp_rgb};

/// Build per-character spans whose gradient sweeps over `period_ms`.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::gradient_sweep::sweep_spans;
/// let spans = sweep_spans("Hey", (0, 229, 255), (255, 64, 216), true, 3000);
/// assert_eq!(spans.len(), 3);
/// ```
pub fn sweep_spans(
    text: &str,
    from: Rgb,
    to: Rgb,
    bold: bool,
    period_ms: u128,
) -> Vec<Span<'static>> {
    let phase = phase_now(period_ms);
    let chars: Vec<char> = text.chars().collect();
    let last = chars.len().saturating_sub(1).max(1) as f32;
    chars
        .iter()
        .enumerate()
        .map(|(i, c)| {
            // Triangle-wave so the sweep ping-pongs instead of snapping.
            let t = ((i as f32 / last) + phase) % 1.0;
            let tri = if t < 0.5 { t * 2.0 } else { 2.0 - t * 2.0 };
            let mut style = Style::default().fg(lerp_rgb(from, to, tri));
            if bold {
                style = style.add_modifier(Modifier::BOLD);
            }
            Span::styled(c.to_string(), style)
        })
        .collect()
}

fn phase_now(period_ms: u128) -> f32 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    (now % period_ms.max(1)) as f32 / period_ms.max(1) as f32
}
