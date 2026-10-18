//! Native insertion of shared empty-search retry guidance.
use super::super::Runner;
pub(super) fn record(runner: &mut Runner<'_>, missed: bool) {
    runner
        .inner
        .progress
        .record_codesearch(&runner.inner.session.agent, missed);
}
pub(super) fn guard(runner: &mut Runner<'_>, step: usize) -> bool {
    let Some(text) = runner.inner.progress.codesearch_nudge(step) else {
        return false;
    };
    super::super::response::nudge::add(runner, text);
    true
}
