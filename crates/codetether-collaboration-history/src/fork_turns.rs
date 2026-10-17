//! Validated transcript inheritance selector for spawned agents.

use anyhow::{Result, bail};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Validated parent transcript inheritance scope.
///
/// # Examples
/// ```
/// use codetether_collaboration_history::ForkTurns;
/// assert_eq!(ForkTurns::parse(Some("2")).unwrap(),ForkTurns::Count(2));
/// ```
pub enum ForkTurns {
    /// Inherit no transcript.
    None,
    /// Inherit all selected parent user turns.
    All,
    /// Inherit the requested number of recent parent user turns.
    Count(usize),
}

impl ForkTurns {
    /// Parse a scope, defaulting to all when omitted.
    /// # Arguments
    /// * `value` — None, all, or a positive integer after whitespace trimming.
    /// # Returns
    /// The requested transcript scope.
    /// # Errors
    /// Rejects zero, non-integer counts and unknown names.
    /// # Examples
    /// ```
    /// assert!(codetether_collaboration_history::ForkTurns::parse(Some("0")).is_err());
    /// ```
    pub fn parse(value: Option<&str>) -> Result<Self> {
        match value.unwrap_or("all").trim() {
            "none" => Ok(Self::None),
            "all" => Ok(Self::All),
            value => match value.parse::<usize>() {
                Ok(count) if count > 0 => Ok(Self::Count(count)),
                _ => bail!("fork_turns must be none, all, or a positive integer"),
            },
        }
    }
}

#[cfg(test)]
#[path = "fork_turns_tests.rs"]
mod tests;
