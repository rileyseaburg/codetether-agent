//! Filesystem capabilities assigned to a swarm participant.

/// Filesystem permissions assigned to one swarm participant.
/// # Examples
/// ```
/// use codetether_workspace_tools::Capability;
/// assert_eq!(Capability::from_task_flags(true, true, true), Capability::Mutating);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Capability {
    /// Inspection tools only.
    ReadOnly,
    /// Inspection plus confined shell and non-committing Git operations.
    Verification,
    /// Rooted mutation tools with commits enabled.
    Mutating,
}

impl Capability {
    /// Map read-only and verification flags to one filesystem capability.
    pub fn from_flags(read_only: bool, verification: bool) -> Self {
        match (read_only, verification) {
            (true, _) => Self::ReadOnly,
            (false, true) => Self::Verification,
            (false, false) => Self::Mutating,
        }
    }

    /// Resolves a worker capability, giving an explicit source-change contract
    /// precedence over stale read-only or verification classification flags.
    pub fn from_task_flags(read_only: bool, verification: bool, expects_changes: bool) -> Self {
        if expects_changes {
            Self::Mutating
        } else {
            Self::from_flags(read_only, verification)
        }
    }

    pub(crate) fn allows_commit(self) -> bool {
        self == Self::Mutating
    }

    /// Whether the participant may produce source changes.
    pub fn is_mutating(self) -> bool {
        self == Self::Mutating
    }
}
