//! Per-instance optimistic state; cloning does not share revisions.
use std::path::PathBuf;
use std::sync::Mutex;
/// Runtime-only checkpoint for a session loaded from SQLite.
///
/// # Examples
/// ```
/// let state = codetether_agent::session::store::State::default();
/// assert!(!state.is_persisted());
/// ```
#[derive(Debug, Default)]
pub struct State(pub(super) Mutex<Checkpoint>);
#[derive(Debug, Clone, Default)]
pub(super) struct Checkpoint {
    pub id: String,
    pub path: PathBuf,
    pub revision: i64,
    pub read_only: bool,
    pub message_start: usize,
    pub tool_start: usize,
    pub header: String,
    pub versions: [u64; 3],
    pub constraints: Vec<(usize, String)>,
    pub message_end: usize,
    pub tool_end: usize,
}
impl Clone for State {
    fn clone(&self) -> Self {
        Self(Mutex::new(self.0.lock().unwrap().clone()))
    }
}
impl State {
    pub(crate) fn path_for(&self, id: &str) -> Option<PathBuf> {
        let state = self.0.lock().ok()?;
        (state.revision > 0 && state.id == id).then(|| state.path.clone())
    }
    /// Whether this object has an acknowledged database revision.
    pub fn is_persisted(&self) -> bool {
        self.0.lock().is_ok_and(|state| state.revision > 0)
    }
}
