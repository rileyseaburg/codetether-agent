//! Verify and back up a legacy snapshot before publishing indexed storage.
use super::{Legacy, digest::digest};
use anyhow::Result;
use std::{io::BufReader, path::Path};

/// Import a legacy JSON snapshot while preserving and verifying its original.
/// # Arguments
/// `path` is the legacy file or an already-published indexed locator.
/// # Returns
/// Unit after the indexed records, summaries and locator have been published;
/// an existing locator returns immediately.
/// # Errors
/// Propagates file/JSON/database errors, oversized input, invalid identities,
/// or a changed original/backup digest. A failed import does not delete evidence.
/// # Examples
/// ```rust,no_run
/// # fn example<S: codetether_session_store::legacy::Legacy>(path: &std::path::Path) -> anyhow::Result<()> {
/// codetether_session_store::legacy::ensure::<S>(path)?;
/// # Ok(()) }
/// ```
pub fn ensure<S: Legacy>(path: &Path) -> Result<()> {
    if crate::marker::is_marker(path)? {
        return Ok(());
    }
    let before = digest(path)?;
    let session: S = serde_json::from_reader(BufReader::new(std::fs::File::open(path)?))?;
    let backup = path.with_extension("legacy.json");
    if !backup.exists() {
        std::fs::copy(path, &backup)?;
        std::fs::File::open(&backup)?.sync_all()?;
    }
    anyhow::ensure!(
        digest(&backup)? == before && digest(path)? == before,
        "legacy session changed during import; stop its writer and retry"
    );
    let mut batch = crate::prepare(&session, &Default::default())?;
    crate::id::validate(session.id())?;
    batch.header = session.legacy_header()?;
    crate::transaction::commit(path, &batch)?;
    crate::summaries::import(path, session.id(), session.summary_index())?;
    crate::marker::create(path, session.id())?;
    Ok(())
}
