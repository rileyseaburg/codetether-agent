//! Runtime capability mode that contributes a sourced ledger constraint.
//!
//! A read-only inspection agent may not run commands at all, while a
//! verification agent (for example the goal verifier) must run read-only
//! checks such as tests and `git status` but may not modify files.

use super::constraint_entry::ConstraintEntry;

/// Capability of the delegated agent, as seen by the constraint ledger.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(Mode::from_flags(false, false), Mode::Verification);
/// assert_eq!(Mode::from_flags(true, false), Mode::ReadOnly);
/// assert_eq!(Mode::from_flags(false, true), Mode::Mutating);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Mode {
    /// Inspect only: no commands, no edits.
    ReadOnly,
    /// Run read-only verification commands; no edits.
    Verification,
    /// Full implementation access.
    Mutating,
}

impl Mode {
    /// Map the prompt's `read_only` / `expects_changes` flags to a mode.
    pub(crate) fn from_flags(read_only: bool, expects_changes: bool) -> Self {
        match (read_only, expects_changes) {
            (true, _) => Self::ReadOnly,
            (false, false) => Self::Verification,
            (false, true) => Self::Mutating,
        }
    }

    /// The runtime constraint this mode adds to the ledger, if any.
    pub(super) fn entry(self) -> Option<ConstraintEntry> {
        let (source, text) = match self {
            Self::ReadOnly => (
                "runtime read-only mode",
                "Do not run shell commands or mutate files.",
            ),
            Self::Verification => (
                "runtime verification mode",
                "Run read-only verification commands (tests, builds, git status/diff) as \
                 needed, but do not create, edit, delete, commit, or push anything.",
            ),
            Self::Mutating => return None,
        };
        Some(ConstraintEntry::new(source, text))
    }
}
