//! Heat-colored paint for throughput sparkline blocks.
//!
//! Each sample block is tinted by its normalized height: cool cyan for
//! low throughput up to hot magenta for peaks, so bursts pop visually.

use ratatui::{style::Style, text::Span};

use crate::tui::ui::gradient::lerp_rgb;

const COOL: (u8, u8, u8) = (0, 150, 200);
const HOT: (u8, u8, u8) = (255, 60, 180);
const BLOCKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

/// Paint normalized samples (`0.0..=1.0`) as heat-tinted block spans.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::sparkline_paint::paint_samples;
/// let spans = paint_samples(&[0.0, 0.5, 1.0]);
/// assert_eq!(spans.len(), 3);
/// ```
pub fn paint_samples(normalized: &[f64]) -> Vec<Span<'static>> {
    normalized
        .iter()
        .map(|&v| {
            let v = v.clamp(0.0, 1.0);
            let block = BLOCKS[(v * 7.0).round() as usize];
            let color = lerp_rgb(COOL, HOT, v as f32);
            Span::styled(block.to_string(), Style::default().fg(color))
        })
        .collect()
}
