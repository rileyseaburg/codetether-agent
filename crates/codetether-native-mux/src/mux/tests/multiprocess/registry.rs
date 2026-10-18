//! Registry scanning rooted at the proof's private data directory.

use std::path::Path;

use crate::mux::registry::{MuxRecord, SessionTarget};

/// Find the server record under `root/mux` that hosts `name`.
pub(super) async fn find_session(root: &Path, name: &str) -> Option<SessionTarget> {
    let mut entries = tokio::fs::read_dir(root.join("mux")).await.ok()?;
    while let Ok(Some(entry)) = entries.next_entry().await {
        if entry.path().extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let Ok(bytes) = tokio::fs::read(entry.path()).await else {
            continue;
        };
        let Ok(record) = serde_json::from_slice::<MuxRecord>(&bytes) else {
            continue;
        };
        if record.hosts(name) {
            return Some(SessionTarget {
                record,
                session: name.to_string(),
            });
        }
    }
    None
}
