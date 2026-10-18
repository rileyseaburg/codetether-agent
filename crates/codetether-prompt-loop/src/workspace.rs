//! Mutable workspace-validation state for one prompt turn.
use std::{collections::HashSet, path::PathBuf};

/// Workspace state used to scope post-edit validation.
/// # Examples
/// ```
/// let state = codetether_prompt_loop::WorkspaceState { cwd: ".".into(), baseline_dirty: Default::default(), touched: Default::default() };
/// assert!(state.touched.is_empty());
/// ```
pub struct WorkspaceState {
    /// Effective working directory for tools and validation.
    pub cwd: PathBuf,
    /// Dirty files that predated the prompt turn.
    pub baseline_dirty: HashSet<PathBuf>,
    /// Files touched by successful tools during the turn.
    pub touched: HashSet<PathBuf>,
}
