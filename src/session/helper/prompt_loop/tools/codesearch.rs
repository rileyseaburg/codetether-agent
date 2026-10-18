//! Native insertion of shared empty-search retry guidance.
use super::super::Runner;
pub(super) fn record(runner: &mut Runner<'_>, missed: bool) {
    runner
        .progress
        .record_codesearch(&runner.session.agent, missed);
}
pub(super) fn guard(runner: &mut Runner<'_>, step: usize) -> bool {
    let Some(text) = runner.progress.codesearch_nudge(step) else {
        return false;
    };
    super::super::response::nudge::add(runner, text);
    true
}
