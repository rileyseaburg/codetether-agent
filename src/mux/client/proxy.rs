//! Native clipboard adapter for shared terminal attachment.
#[path = "proxy/clipboard.rs"]
mod clipboard;
use super::connection::MuxConnection;
use anyhow::Result;
pub(super) use codetether_mux_attachment::Outcome;
pub(super) async fn run(
    connection: &mut MuxConnection,
    id: u64,
    offset: u64,
    replay_until: u64,
    alternate_screen: bool,
) -> Result<Outcome> {
    codetether_mux_attachment::run(
        connection,
        id,
        offset,
        replay_until,
        alternate_screen,
        clipboard::local_content,
    )
    .await
}
