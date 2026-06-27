//! Durable client-side resume cursor: the last *fully processed* event id.
//!
//! The cursor advances only after a sequenced event's side effects commit, never
//! on mere receipt. See `docs/transport-phase1-wire-contract.md` section 6.

use std::fs;
use std::path::PathBuf;

use super::event_id::EventId;

/// A file-backed cursor recording the last successfully processed [`EventId`].
///
/// # Examples
///
/// ```rust
/// # let dir = tempfile::tempdir().unwrap();
/// # let path = dir.path().join("cursor");
/// use codetether_agent::a2a::stream::cursor::Cursor;
/// use codetether_agent::a2a::stream::event_id::EventId;
///
/// let mut cursor = Cursor::load(path.clone());
/// assert!(cursor.last().is_none());
///
/// cursor.commit(&EventId { epoch: "e".into(), seq: 7 }).unwrap();
/// let reloaded = Cursor::load(path);
/// assert_eq!(reloaded.last().unwrap().seq, 7);
/// ```
pub struct Cursor {
    path: PathBuf,
    last: Option<EventId>,
}

impl Cursor {
    /// Load a cursor from `path`, returning an empty cursor when absent/invalid.
    pub fn load(path: PathBuf) -> Self {
        let last = fs::read_to_string(&path)
            .ok()
            .and_then(|s| EventId::parse(s.trim()));
        Self { path, last }
    }

    /// The last processed event id, if any.
    pub fn last(&self) -> Option<&EventId> {
        self.last.as_ref()
    }

    /// Persist `id` as the new commit point (write-through to disk).
    ///
    /// # Errors
    ///
    /// Returns an error if the cursor file cannot be written.
    pub fn commit(&mut self, id: &EventId) -> std::io::Result<()> {
        fs::write(&self.path, id.format())?;
        self.last = Some(id.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Cursor;
    use super::EventId;

    #[test]
    fn commit_persists_and_reloads() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("c");
        let mut cursor = Cursor::load(path.clone());
        assert!(cursor.last().is_none());
        cursor
            .commit(&EventId { epoch: "ep".into(), seq: 3 })
            .unwrap();
        assert_eq!(Cursor::load(path).last().unwrap().seq, 3);
    }
}
