//! Application throughput adapter for the shared sparkline.
use crate::tui::app::state::App;
use ratatui::text::Span;
/// Sample the stream only when processing and render retained samples.
pub fn sparkline_spans(app: &App) -> Option<Vec<Span<'static>>> {
    codetether_token_display::throughput_sparkline::sparkline_spans(app.state.processing, || {
        app.state.streaming.streaming_tok_per_sec()
    })
}
