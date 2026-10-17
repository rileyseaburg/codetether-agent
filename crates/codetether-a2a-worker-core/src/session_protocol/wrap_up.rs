//! Worker step-budget wrap-up message construction.
use super::wrap_up_step;
use codetether_provider_core::{ContentPart, Message, Role};
/// Return a user-role wrap-up message exactly at the configured boundary.
///
/// # Arguments
/// * `step` - Current model step.
/// * `max_steps` - Configured step budget.
/// # Returns
/// The existing wrap-up instruction at its boundary, otherwise `None`.
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::session_protocol::wrap_up_message;
/// assert!(wrap_up_message(170, 200).is_some());
/// assert!(wrap_up_message(169, 200).is_none());
/// ```

pub fn wrap_up_message(step: usize, max_steps: usize) -> Option<Message> {
    if max_steps < 2 || step != wrap_up_step(max_steps) {
        return None;
    }
    let remaining = max_steps.saturating_sub(step);
    tracing::warn!(target: "codetether_agent::a2a::worker::session_steps::session_wrap_up",
        step,
        max_steps,
        remaining,
        "Injecting step-budget wrap-up nudge"
    );
    Some(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: format!(
                "SYSTEM NOTICE: only {remaining} model steps remain in this task's budget \
                 of {max_steps}. Stop expanding scope now. In the remaining steps: \
                 (1) make sure the code compiles/passes the narrowest check you already ran, \
                 (2) `git add` and commit everything you changed with a descriptive message, \
                 (3) push the branch and open or update the pull request if the task asked for one, \
                 (4) post the commit/PR link and a short status back to the issue or PR. \
                 Unfinished items must be listed in the PR description as follow-ups, not attempted."
            ),
        }],
    })
}
