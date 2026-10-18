//! Observed verifier execution with application-provided runtime operations.
//!
//! [review] owns the observation lifecycle. [Host] supplies provider/tool/loop
//! operations and [Run] carries the exact prompts and execution limits.
mod execute;
mod host;
mod request;
use crate::{ReviewExecution, VerificationRequest};
pub use host::Host;
pub use request::Run;
use std::path::Path;
/// Execute a verifier and retain the identity captured for that run.
/// # Arguments
/// * `worker_model` - Worker's model used when no verifier model is selected.
/// * `workspace` - Repository inspected by the verifier.
/// * `request` - Goal scope and claimed transition.
/// # Returns
/// Report or execution error paired with the observation's actual identity.
/// # Examples
/// ```no_run
/// async fn review<H: codetether_goal_verification::runtime::Host>(request: &codetether_goal_verification::VerificationRequest) {
///     let _result = codetether_goal_verification::runtime::review::<H>(None, std::path::Path::new("."), request).await;
/// }
/// ```
pub async fn review<H: Host>(
    worker_model: Option<&str>,
    workspace: &Path,
    request: &VerificationRequest,
) -> ReviewExecution {
    let mut ticket = codetether_verifier_state::observation::shared_observations().start();
    let report = execute::run::<H>(worker_model, workspace, request, &mut ticket).await;
    ticket.finish(&report);
    ReviewExecution {
        report,
        identity: ticket.identity(),
    }
}
