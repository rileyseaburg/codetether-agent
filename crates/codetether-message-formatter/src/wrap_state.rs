//! Output rows and current row accumulated during wrapping.
use ratatui::text::{Line, Span};
#[derive(Default)]
pub(crate) struct WrapState {
    pub(crate) out: Vec<Line<'static>>,
    pub(crate) cur: Vec<Span<'static>>,
    pub(crate) cur_w: usize,
}
