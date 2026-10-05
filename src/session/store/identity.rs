//! Stable transaction identities make a retry after a lost acknowledgement safe.
use super::batch::Batch;
use sha2::{Digest, Sha256};
pub(super) fn nonce(batch: &Batch) -> String {
    let mut hash = Sha256::new();
    hash.update(batch.id.as_bytes());
    hash.update(batch.revision.to_le_bytes());
    hash.update(batch.header.as_bytes());
    for records in [&batch.messages, &batch.tools] {
        hash.update(records.start.to_le_bytes());
        hash.update(records.end.to_le_bytes());
        for body in &records.rows {
            hash.update(body.len().to_le_bytes());
            hash.update(body.as_bytes());
        }
    }
    hex::encode(hash.finalize())
}
