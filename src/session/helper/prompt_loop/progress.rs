//! Loop counters and workspace validation state.

use std::collections::HashSet;
use std::path::PathBuf;
use uuid::Uuid;

/// Workspace state used to scope post-edit validation.
pub(crate) struct WorkspaceState {
    /// Effective working directory for tools and validation.
    pub cwd: PathBuf,
    /// Dirty files that predated the prompt turn.
    pub baseline_dirty: HashSet<PathBuf>,
    /// Files touched by successful tools during the turn.
    pub touched: HashSet<PathBuf>,
}

/// Progress counters retained across prompt-loop steps.
pub(crate) struct LoopState {
    /// Assistant text accumulated for the final result.
    pub output: String,
    /// Maximum provider/tool iterations allowed for the turn.
    pub max_steps: usize,
    /// Number of failed post-edit validation attempts.
    pub validation_retries: u8,
    /// Canonical signature of the previous tool-call batch.
    pub last_tool_signature: Option<String>,
    /// Number of consecutive batches matching the prior signature.
    pub repeated_tools: u32,
    /// Guard against repeated edit-family invocations.
    pub repeat_guard: super::super::repeat_guard::RepeatGuard,
    /// Consecutive code-search calls returning no matches.
    pub codesearch_misses: u32,
    /// Build-mode retries waiting for an execution tool call.
    pub build_retries: u8,
    /// Retries after prose promised but omitted a native tool call.
    pub native_retries: u8,
    /// Steps elapsed since the last file-writing tool call.
    pub steps_since_write: u32,
    /// Correlation identifier used for agent-bus messages.
    pub turn_id: String,
}

impl LoopState {
    /// Creates zeroed progress state for the requested step budget.
    pub fn new(max_steps: usize) -> Self {
        Self {
            output: String::new(),
            max_steps,
            validation_retries: 0,
            last_tool_signature: None,
            repeated_tools: 0,
            repeat_guard: Default::default(),
            codesearch_misses: 0,
            build_retries: 0,
            native_retries: 0,
            steps_since_write: 0,
            turn_id: Uuid::new_v4().to_string(),
        }
    }
}
