//! Native field projection for shared tool-event publication.
use super::{super::Runner, call::Call, outcome::Outcome};
use codetether_bus_core::tool_events::{self, Context};
fn context<'a>(runner: &'a Runner<'_>, step: usize, call: &'a Call) -> Context<'a> {
    Context {
        agent: &runner.inner.session.agent,
        turn_id: &runner.inner.progress.turn_id,
        step,
        request_id: &call.id,
        tool_name: &call.name,
    }
}
pub(super) fn request(runner: &Runner<'_>, step: usize, call: &Call) {
    tool_events::request(
        runner.inner.session.bus.as_ref(),
        context(runner, step, call),
        &call.input,
    );
}
pub(super) fn response(runner: &Runner<'_>, step: usize, call: &Call, outcome: &Outcome) {
    tool_events::response(
        runner.inner.session.bus.as_ref(),
        context(runner, step, call),
        &outcome.rendered,
        outcome.success,
    );
}
