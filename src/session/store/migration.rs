//! One-time legacy import. Preserve the original before publishing a locator.
use crate::session::Session;
use anyhow::Result;
use sha2::{Digest, Sha256};
use std::io::{BufReader, Read};
use std::path::Path;
pub(super) fn ensure(path: &Path) -> Result<()> {
    if super::marker::is_marker(path)? {
        return Ok(());
    }
    let before = digest(path)?;
    let session: Session = serde_json::from_reader(BufReader::new(std::fs::File::open(path)?))?;
    let backup = path.with_extension("legacy.json");
    if !backup.exists() {
        std::fs::copy(path, &backup)?;
        std::fs::File::open(&backup)?.sync_all()?;
    }
    anyhow::ensure!(
        digest(&backup)? == before && digest(path)? == before,
        "legacy session changed during import; stop its writer and retry"
    );
    let mut batch = super::batch::prepare(&session, &Default::default())?;
    super::id::validate(&session.id)?;
    batch.header = super::header::legacy(&session)?;
    super::transaction::commit(path, &batch)?;
    super::summary_import::run(path, &session)?;
    super::marker::create(path, &session.id)?;
    Ok(())
}
fn digest(path: &Path) -> Result<Vec<u8>> {
    let mut file = std::fs::File::open(path)?;
    anyhow::ensure!(
        file.metadata()?.len() <= 512 * 1024 * 1024,
        "legacy session exceeds 512 MiB import cap"
    );
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    Ok(hash.finalize().to_vec())
}
