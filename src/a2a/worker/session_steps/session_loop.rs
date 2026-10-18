//! Adapt application session state to the shared worker loop driver.
use super::{SessionStepsOutcome, session_context::Context};
use crate::session::Session;
use anyhow::Result;
#[path = "session_loop_host.rs"]
mod host;
struct ApplicationHost<'a, 'context> {
    context: &'a Context<'context>,
    session: &'a mut Session,
}
pub(super) async fn run(
    context: &Context<'_>,
    session: &mut Session,
    max_steps: usize,
) -> Result<SessionStepsOutcome> {
    codetether_a2a_worker_core::session_protocol::driver::run_scoped(
        &mut ApplicationHost { context, session },
        max_steps,
    )
    .await
}
