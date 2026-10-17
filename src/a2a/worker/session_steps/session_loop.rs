//! Worker loop consumes shared steering, hold, cancellation, and continuation policy.

use super::{SessionStepsOutcome, session_context::Context, session_failure, session_response};
use crate::session::{
    Session,
    runtime::{boundary, continuation, steering},
};
use anyhow::Result;

pub(super) async fn run(
    context: &Context<'_>,
    session: &mut Session,
    max_steps: usize,
) -> Result<SessionStepsOutcome> {
    let mut output = String::new();
    let mut completed = false;
    for step in 1..=max_steps {
        boundary::ensure_running(&session.id).await?;
        steering::drain_into(session);
        super::session_wrap_up::maybe_inject(session, step, max_steps);
        let response = super::session_complete::complete(context, session).await?;
        if session_response::process_response(&context.response, session, &mut output, response)
            .await?
        {
            match continuation::after_message(session).await? {
                continuation::Continuation::Finish => {
                    completed = true;
                    break;
                }
                continuation::Continuation::Goal => output.clear(),
                continuation::Continuation::Steering => {}
            }
        }
    }
    if !completed {
        session_failure::record_loop_halt(
            session,
            &format!("step budget ({max_steps}) exhausted before completion"),
        )
        .await;
    }
    session.save().await?;
    Ok(SessionStepsOutcome {
        text: output.trim().into(),
        budget_exhausted: !completed,
        max_steps,
    })
}
