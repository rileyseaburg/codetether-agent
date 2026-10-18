//! Webview selection with classic-chat fallback.
use crate::Chat;
use ratatui::Frame;
pub(super) fn render(frame: &mut Frame, host: &mut impl Chat) {
    if !codetether_chat_layout::webview::is_webview(host.layout()) || !host.webview(frame) {
        host.classic(frame);
    }
}
