//! Local clipboard access supplied to shared mux attachment flows.
#[path = "proxy/clipboard.rs"]
mod clipboard;
pub(super) use codetether_mux_attachment::Outcome;
pub(super) fn clipboard_content() -> Option<String> {
    clipboard::local_content()
}
