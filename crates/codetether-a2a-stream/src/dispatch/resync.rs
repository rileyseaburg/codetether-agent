use super::Host;
use crate::{cursor::Cursor, frame::ParsedFrame, resync::parse_resync};
/// Drop the resume cursor and cold-reconcile pending tasks on `resync-required`.
pub(super) async fn handle_resync<H: Host>(frame: &ParsedFrame, host: &H, cursor: &mut Cursor) {
    match parse_resync(&frame.data) {
        Ok(resync) => {
            tracing::warn!(reason = ?resync.reason, epoch = %resync.epoch, "Stream resync required")
        }
        Err(error) => tracing::warn!(error = %error, "Malformed resync-required payload"),
    }
    if let Err(error) = cursor.reset() {
        tracing::warn!(error = %error, "Failed to reset stream cursor on resync");
    }
    if let Err(error) = host.reconcile().await {
        tracing::warn!(error = %error, "Cold reconcile after resync failed");
    }
}
