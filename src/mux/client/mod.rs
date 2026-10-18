//! Native clipboard adapter for shared mux session attachment.

mod clipboard;
pub(in crate::mux) mod handshake;

pub(super) use codetether_mux_client::MuxConnection;
pub(super) use handshake::probe;

/// Attach using clipboard access supplied by the native application.
pub(super) async fn attach(target: &crate::mux::registry::SessionTarget) -> anyhow::Result<()> {
    codetether_mux_attachment::attach_session(target, clipboard::local_content).await
}
