//! Native storage binding for the shared proactive worker loop.
mod host;
pub(super) fn spawn(session_id: String) {
    codetether_proactive_worker::spawn(session_id, std::sync::Arc::new(host::NativeHost));
}
