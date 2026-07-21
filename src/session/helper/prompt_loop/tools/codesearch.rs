//! Build-agent protection against repeated empty code searches.

use super::super::Runner;

/// Updates the consecutive empty-search counter for build agents.
pub(super) fn record(runner: &mut Runner<'_>, missed: bool) {
    if !super::super::super::build::is_build_agent(&runner.session.agent) {
        return;
    }
    runner.progress.codesearch_misses = if missed {
        runner.progress.codesearch_misses + 1
    } else {
        0
    };
}

/// Inserts a corrective nudge when empty-search retries reach the limit.
pub(super) fn guard(runner: &mut Runner<'_>, step: usize) -> bool {
    if runner.progress.codesearch_misses
        < super::super::super::loop_constants::MAX_CONSECUTIVE_CODESEARCH_NO_MATCHES
    {
        return false;
    }
    tracing::warn!(
        step,
        consecutive_codesearch_no_matches = runner.progress.codesearch_misses,
        "Detected codesearch no-match thrash"
    );
    super::super::response::nudge::add(
        runner,
        super::super::super::loop_constants::CODESEARCH_THRASH_NUDGE,
    );
    true
}
