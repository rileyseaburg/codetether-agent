//! Live token-throughput sparkline for the status bar.
//!
//! Maintains a thread-local ring buffer of recent tokens/sec samples and
//! renders them as Unicode block glyphs. On truecolor terminals each block
//! is heat-tinted (cool cyan → hot magenta) by its normalized height.
//! Sampling and rendering happen only while `processing` is true; when idle
//! the buffer is cleared and `None` is returned (0% idle CPU).

use std::cell::RefCell;

use ratatui::{
    style::{Color, Style},
    text::Span,
};

use codetether_theme::gradient::rgb_supported;

const CAP: usize = 20;
const BLOCKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

thread_local! {
    static SAMPLES: RefCell<Vec<f64>> = const { RefCell::new(Vec::new()) };
}

/// Build throughput sparkline spans, or `None` when idle / no samples.
///
/// # Examples
///
/// ```
/// let spans = codetether_token_display::throughput_sparkline::sparkline_spans(false, || None);
/// assert!(spans.is_none());
/// ```
///
/// # Arguments
/// * `processing` - Whether the current turn is active.
/// * `throughput` - Lazily supplies the latest token rate while processing.
///
/// # Returns
/// Sparkline spans; idle calls clear the thread-local sample buffer.
pub fn sparkline_spans(
    processing: bool,
    throughput: impl FnOnce() -> Option<f64>,
) -> Option<Vec<Span<'static>>> {
    if !processing {
        SAMPLES.with(|s| s.borrow_mut().clear());
        return None;
    }
    let tps = throughput()?;
    SAMPLES.with(|s| {
        let mut buf = s.borrow_mut();
        buf.push(tps.max(0.0));
        if buf.len() > CAP {
            buf.remove(0);
        }
        let max = buf.iter().cloned().fold(1.0_f64, f64::max);
        let normalized: Vec<f64> = buf.iter().map(|&v| v / max).collect();
        if rgb_supported() {
            return Some(super::sparkline_paint::paint_samples(&normalized));
        }
        let spark: String = normalized
            .iter()
            .map(|&v| BLOCKS[(v * 7.0).round() as usize])
            .collect();
        Some(vec![Span::styled(spark, Style::default().fg(Color::Cyan))])
    })
}
