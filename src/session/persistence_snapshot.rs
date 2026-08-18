//! Borrowed session serialization without cloning the transcript.

use anyhow::Result;

use super::Session;

#[path = "persistence_snapshot_data.rs"]
mod data;

#[cfg(test)]
#[path = "persistence_snapshot_tests.rs"]
mod tests;

pub(super) fn serialize(session: &Session) -> Result<Vec<u8>> {
    let snapshot = data::Snapshot::from_session(session);
    let encode = || serde_json::to_vec(&snapshot).map_err(Into::into);
    match tokio::runtime::Handle::try_current().map(|handle| handle.runtime_flavor()) {
        Ok(tokio::runtime::RuntimeFlavor::MultiThread) => tokio::task::block_in_place(encode),
        Ok(tokio::runtime::RuntimeFlavor::CurrentThread) | Err(_) => encode(),
        _ => encode(),
    }
}
