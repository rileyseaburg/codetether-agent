//! Native clipboard adapter for shared program reconnect.
use super::connection::MuxConnection;
use anyhow::Result;
pub(super) async fn attach(
    connection: &mut MuxConnection,
    id: u64,
) -> Result<Option<super::proxy::Outcome>> {
    codetether_mux_attachment::program::attach(connection, id, super::proxy::clipboard_content)
        .await
}
