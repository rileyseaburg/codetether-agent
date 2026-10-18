//! Per-instance optimistic state; cloning does not share revisions.
use std::path::PathBuf;
use std::sync::Mutex;
/// Runtime-only checkpoint for a session loaded from SQLite.
///
/// # Examples
/// ```
/// let state = codetether_session_store::State::default();
/// assert!(!state.is_persisted());
/// ```
#[derive(Debug, Default)]
pub struct State(
    /// Instance-local checkpoint lock shared only by borrows of this state.
    pub Mutex<Checkpoint>,
);
/// Persisted revision and loaded-window metadata for one native session instance.
/// # Examples
/// ```rust
/// let checkpoint = codetether_session_store::Checkpoint::default();
/// assert_eq!(checkpoint.revision, 0);
/// ```
#[derive(Debug, Clone, Default)]
pub struct Checkpoint {
    /// Durable session identity.
    pub id: String,
    /// Locator identifying the containing database.
    pub path: PathBuf,
    /// Last acknowledged persisted revision.
    pub revision: i64,
    /// Whether this loaded window permits writes.
    pub read_only: bool,
    /// Absolute offset of the loaded message window.
    pub message_start: usize,
    /// Absolute offset of the loaded tool-use window.
    pub tool_start: usize,
    /// Last acknowledged serialized session header.
    pub header: String,
    /// Acknowledged message, page and tool vector mutation identities.
    pub versions: [u64; 3],
    /// Protected sequence/excerpt pairs retained beyond the loaded window.
    pub constraints: Vec<(usize, String)>,
    /// Absolute persisted end of message history.
    pub message_end: usize,
    /// Absolute persisted end of tool history.
    pub tool_end: usize,
}
impl Clone for State {
    fn clone(&self) -> Self {
        Self(Mutex::new(self.0.lock().unwrap().clone()))
    }
}
impl State {
    /// Return the locator only for a matching, persisted identity.
    /// # Examples
    /// ```rust
    /// assert!(codetether_session_store::State::default().path_for("new").is_none());
    /// ```
    pub fn path_for(&self, id: &str) -> Option<PathBuf> {
        let state = self.0.lock().ok()?;
        (state.revision > 0 && state.id == id).then(|| state.path.clone())
    }
    /// Whether this object has an acknowledged database revision.
    pub fn is_persisted(&self) -> bool {
        self.0.lock().is_ok_and(|state| state.revision > 0)
    }
}
