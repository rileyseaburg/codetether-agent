//! Native clipboard callback for shared mux input filtering and transport.
#[path = "input/clipboard.rs"]
mod clipboard;
pub(super) use codetether_mux_input::send;
pub(super) struct Pipeline(codetether_mux_input::Pipeline);
impl Pipeline {
    pub(super) fn new() -> Self {
        Self(codetether_mux_input::Pipeline::new())
    }
    pub(super) fn filter(&mut self, raw: &[u8]) -> codetether_mux_input::Filtered {
        self.0.filter(raw, clipboard::local_content)
    }
}
