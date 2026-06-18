//! Live token-throughput sparkline for the status bar.
//!
//! Maintains a thread-local ring buffer of recent tokens/sec samples and
//! renders them as Unicode block glyphs. Sampling and rendering happen only
//! while `processing` is true; when idle the buffer is cleared and `None` is
//! returned, so an idle TUI performs no work here (0% idle CPU).

use std::cell::RefCell;

use ratatui::{
    style::{Color, Style},
    text::Span,
};

use crate::tui::app::state::App;

const CAP: usize = 20;
const BLOCKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

thread_local! {
    static SAMPLES: RefCell<Vec<f64>> = const { RefCell::new(Vec::new()) };
}

/// Build a throughput sparkline span, or `None` when idle / no samples.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::throughput_sparkline::sparkline_span;
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// let _maybe = sparkline_span(app);
/// # }
/// ```
pub fn sparkline_span(app: &App) -> Option<Span<'static>> {
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
        let spark: String = buf
            .iter()
            .map(|&v| BLOCKS[((v / max) * 7.0).round() as usize])
            .collect();
        Some(Span::styled(spark, Style::default().fg(Color::Cyan)))
    })
}
