//! Chat layout render bindings.
use codetether_chat_layout::webview::ChatLayoutMode;
use ratatui::Frame;
/// Select and render a chat layout.
/// # Implementors
/// Native adapters expose the selected mode and bind both chat renderers.
pub trait Chat {
    /// Current chat layout choice.
    fn layout(&self) -> ChatLayoutMode;
    /// Render Webview, returning false if it cannot fit.
    fn webview(&mut self, frame: &mut Frame) -> bool;
    /// Render classic chat.
    fn classic(&mut self, frame: &mut Frame);
}
