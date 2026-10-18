//! Native clipboard adapter for shared mux session attachment.

mod clipboard;
#[cfg(test)]
pub(super) use codetether_mux_client::MuxConnection;

pub(in crate::mux) fn clipboard_content() -> Option<String> {
    clipboard::local_content()
}
