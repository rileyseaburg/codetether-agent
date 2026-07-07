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

use crate::tui::app::state::App;
use crate::tui::ui::gradient::rgb_supported;

const CAP: usize = 20;
const BLOCKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

thread_local! {
    static SAMPLES: RefCell<Vec<f64>> = const { RefCell::new(Vec::new()) };
}

/// Build throughput sparkline spans, or `None` when idle / no samples.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::throughput_sparkline::sparkline_spans;
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// let _maybe = sparkline_spans(app);
/// # }
/// ```
pub fn sparkline_spans(app: &App) -> Option<Vec<Span<'static>>> {
    if !app.state.processing {
        SAMPLES.with(|s| s.borrow_mut().clear());
        return None;
    }
    let tps = app.state.streaming_tok_per_sec()?;
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
