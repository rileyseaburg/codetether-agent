use super::{Host, View, chars::paste_chars_no_newlines};
/// Normalize and route paste text in the established destination order.
/// # Arguments
/// `host` supplies view state and editing; `text` is the raw terminal paste.
/// # Returns
/// Completes after the first applicable paste destination handles the input.
/// # Panics
/// May panic on chat sidecar identifier overflow.
/// # Examples
/// ```no_run
/// # fn paste(host: &mut impl codetether_paste_buffer::routing::Host) {
/// codetether_paste_buffer::routing::handle(host, "text\r\n");
/// # }
/// ```
pub fn handle(host: &mut impl Host, text: &str) {
    let normalized = codetether_terminal_text::normalize_paste(text);

    if host.symbol_search_active() {
        paste_chars_no_newlines(&normalized, |ch| host.symbol_char(ch));
        host.refresh_symbols();
        return;
    }
    if host.view() == View::Bus && host.bus_filter_active() {
        paste_chars_no_newlines(&normalized, |ch| host.bus_char(ch));
        host.set_status(format!("Protocol filter: {}", host.bus_filter()));
        return;
    }
    if host.view() == View::Model {
        paste_chars_no_newlines(&normalized, |ch| host.model_char(ch));
        return;
    }
    if host.view() == View::Sessions {
        paste_chars_no_newlines(&normalized, |ch| host.session_char(ch));
        return;
    }
    if host.view() == View::Chat {
        crate::input::paste_into_chat(host, &normalized);
    }
}
