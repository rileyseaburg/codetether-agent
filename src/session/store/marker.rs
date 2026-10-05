//! Small locators fence legacy readers; old binaries cannot deserialize a Session.
use anyhow::Result;
use std::io::{Read, Write};
use std::path::Path;
const PREFIX: &[u8] = b"{\"codetether_store\":1,";
pub(super) fn is_marker(path: &Path) -> Result<bool> {
    let mut file = std::fs::File::open(path)?;
    let mut prefix = vec![0; PREFIX.len()];
    Ok(file.read_exact(&mut prefix).is_ok() && prefix == PREFIX)
}
pub(super) fn check(path: &Path, expected: &str) -> Result<()> {
    anyhow::ensure!(
        !path.exists() || (is_marker(path)? && id(path)? == expected),
        "LEGACY_SESSION_WRITER: import or reload the session before writing; stop older runtimes"
    );
    Ok(())
}
pub(super) fn create(path: &Path, id: &str) -> Result<()> {
    if path.exists() && is_marker(path)? {
        return Ok(());
    }
    let parent = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("missing parent"))?;
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    write!(
        temp,
        "{{\"codetether_store\":1,\"id\":{}}}",
        serde_json::to_string(id)?
    )?;
    temp.as_file().sync_all()?;
    temp.persist(path)?;
    #[cfg(unix)]
    std::fs::File::open(parent)?.sync_all()?;
    Ok(())
}
pub(super) fn id(path: &Path) -> Result<String> {
    let file = std::fs::File::open(path)?;
    let value: serde_json::Value = serde_json::from_reader(file.take(4096))?;
    value["id"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| anyhow::anyhow!("invalid session locator"))
}
