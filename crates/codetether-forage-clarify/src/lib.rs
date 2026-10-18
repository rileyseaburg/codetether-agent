//! Interactive goal clarification when forage finds no opportunities.
//!
//! Rather than dead-ending at "selected 0", forage infers *why* no work was
//! found and — when running interactively — asks the user to state their
//! goals, then persists them as an OKR so the next cycle has actionable work.

use anyhow::Result;
use std::io::{self, IsTerminal};

use codetether_okr::OkrRepository;

mod persist;

#[cfg(test)]
mod tests;

/// Why forage produced no opportunities this cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EmptyReason {
    /// No OKRs exist at all.
    NoOkrs,
    /// OKRs exist but every key result is complete or filtered out.
    NoActionableWork,
}

/// Decide whether interactive clarification should run.
///
/// Only prompt on a real TTY and when forage is not in quiet (TUI) mode.
pub(crate) fn can_prompt() -> bool {
    !codetether_forage_reporting::console::is_quiet()
        && io::stdin().is_terminal()
        && io::stdout().is_terminal()
}

/// Classify why the opportunity list came back empty.
pub(crate) fn classify_empty(okr_count: usize) -> EmptyReason {
    if okr_count == 0 {
        EmptyReason::NoOkrs
    } else {
        EmptyReason::NoActionableWork
    }
}

/// Run the full clarify flow: classify, prompt, and persist goals.
///
/// # Arguments
/// * `repo` — Repository receiving the clarified objective.
/// * `okr_count` — Existing objective count used to explain the empty queue.
/// # Returns
/// `Ok(true)` when new goals were saved and the caller should re-scan;
/// `Ok(false)` when prompting is disabled or no goals were captured.
/// # Errors
/// Forwards validation and persistence errors from saving the new objective.
/// # Examples
/// ```no_run
/// # async fn clarify(repo: &codetether_okr::OkrRepository) -> anyhow::Result<()> {
/// let rescan = codetether_forage_clarify::clarify_and_persist(repo, 0).await?;
/// # let _ = rescan;
/// # Ok(()) }
/// ```
pub async fn clarify_and_persist(repo: &OkrRepository, okr_count: usize) -> Result<bool> {
    if !can_prompt() {
        return Ok(false);
    }
    let reason = classify_empty(okr_count);
    let goals = persist::read_goals(reason);
    if goals.is_empty() {
        return Ok(false);
    }
    persist::persist_goals(repo, &goals).await?;
    Ok(true)
}
