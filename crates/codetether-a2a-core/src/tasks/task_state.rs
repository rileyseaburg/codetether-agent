use super::*;

/// A2A Task States
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// TaskState data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::TaskState::Submitted;
/// match value {
///     codetether_a2a_core::TaskState::Submitted => {},
///     codetether_a2a_core::TaskState::Working => {},
///     codetether_a2a_core::TaskState::Completed => {},
///     codetether_a2a_core::TaskState::Failed => {},
///     codetether_a2a_core::TaskState::Cancelled => {},
///     codetether_a2a_core::TaskState::InputRequired => {},
///     codetether_a2a_core::TaskState::Rejected => {},
///     codetether_a2a_core::TaskState::AuthRequired => {},
/// }
/// ```
pub enum TaskState {
    Submitted,
    Working,
    Completed,
    Failed,
    Cancelled,
    InputRequired,
    Rejected,
    AuthRequired,
}

impl TaskState {
    /// Report whether a task has reached a terminal state.
    ///
    /// # Arguments
    /// None.
    ///
    /// # Returns
    /// True for completed, failed, cancelled or rejected tasks.
    ///
    /// # Examples
    /// ```
    /// assert!(codetether_a2a_core::TaskState::Completed.is_terminal());
    /// ```
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            TaskState::Completed | TaskState::Failed | TaskState::Cancelled | TaskState::Rejected
        )
    }

    /// Report whether a task remains in a nonterminal state.
    ///
    /// # Arguments
    /// None.
    ///
    /// # Returns
    /// The inverse of is_terminal.
    ///
    /// # Examples
    /// ```
    /// assert!(codetether_a2a_core::TaskState::Working.is_active());
    /// ```
    pub fn is_active(&self) -> bool {
        !self.is_terminal()
    }
}
