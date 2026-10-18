//! Small locators fence legacy readers; old binaries cannot deserialize a Session.
use anyhow::Result;
use std::io::{Read, Write};
use std::path::Path;
const PREFIX: &[u8] = b"{\"codetether_store\":1,";
/// Check whether an existing file starts with the locator prefix.
/// # Arguments
/// `path` selects the file to inspect.
/// # Returns
/// Whether the file has the expected prefix.
/// # Errors
/// Opening the file fails.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::marker::is_marker;
/// # fn example(path: &std::path::Path) -> anyhow::Result<()> {
/// let result = is_marker(path)?;
/// # Ok(()) }
/// ```
pub fn is_marker(path: &Path) -> Result<bool> {
    let mut file = std::fs::File::open(path)?;
    let mut prefix = vec![0; PREFIX.len()];
    Ok(file.read_exact(&mut prefix).is_ok() && prefix == PREFIX)
}
/// Verify that an existing locator belongs to the expected session.
/// # Arguments
/// `path` is the locator; `expected` is the allowed session identity.
/// # Returns
/// Unit when no file exists or the existing locator matches.
/// # Errors
/// Reading fails or the file is a legacy snapshot or another session locator.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::marker::check;
/// # fn example(path: &std::path::Path) -> anyhow::Result<()> {
/// let result = check(path, "session-123")?;
/// # Ok(()) }
/// ```
pub fn check(path: &Path, expected: &str) -> Result<()> {
    anyhow::ensure!(
        !path.exists() || (is_marker(path)? && id(path)? == expected),
        "LEGACY_SESSION_WRITER: import or reload the session before writing; stop older runtimes"
    );
    Ok(())
}
/// Persist a small session locator with file and directory synchronization.
/// # Arguments
/// `path` is the locator destination; `id` is its session identity.
/// # Returns
/// Unit after persistence, or when a locator already exists.
/// # Errors
/// Path resolution, reading, serialization or file persistence fails.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::marker::create;
/// # fn example(path: &std::path::Path) -> anyhow::Result<()> {
/// let result = create(path, "session-123")?;
/// # Ok(()) }
/// ```
pub fn create(path: &Path, id: &str) -> Result<()> {
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
/// Read the session identity from a bounded locator document.
/// # Arguments
/// `path` selects the locator to read.
/// # Returns
/// The locator session identity.
/// # Errors
/// Opening, parsing or finding a string identity fails.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::marker::id;
/// # fn example(path: &std::path::Path) -> anyhow::Result<()> {
/// let result = id(path)?;
/// # Ok(()) }
/// ```
pub fn id(path: &Path) -> Result<String> {
    let file = std::fs::File::open(path)?;
    let value: serde_json::Value = serde_json::from_reader(file.take(4096))?;
    value["id"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| anyhow::anyhow!("invalid session locator"))
}
