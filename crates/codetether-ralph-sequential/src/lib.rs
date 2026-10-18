//! Sequential Ralph story lifecycle and ordered completion effects.
//!
//! Implement [Host] for native resources, then call [run] on the live host.
//! The shared scheduler owns iteration limits; this crate owns story selection,
//! prompt enrichment, response handling, verification and completion ordering.
mod host;
mod runtime;
mod scheduling;
mod seq_error;
mod seq_failure;
mod seq_iteration;
mod seq_prompt;
mod seq_response;
mod seq_success;
mod seq_unchecked;
use codetether_ralph_core::{RalphState, RalphStatus, UserStory};
pub use host::{Execution, Host, Prompts, Reporting, State};
use runtime::Runtime;
use tracing::{info, warn};
/// Run sequential stories against the host's live state and services.
///
/// # Arguments
/// * `host` - Native state, prompt, reporting and execution services.
/// # Returns
/// Returns after completion, exhaustion of the iteration budget or no ready story.
/// # Errors
/// Propagates progress, quality, commit and save errors. Model errors are recorded.
/// # Examples
/// ```rust,no_run
/// # async fn example(host: &mut impl codetether_ralph_sequential::Host) -> anyhow::Result<()> {
/// codetether_ralph_sequential::run(host).await
/// # }
/// ```
pub async fn run(host: &mut impl Host) -> anyhow::Result<()> {
    codetether_ralph_core::scheduling::run_sequential(&mut Runtime { host }).await
}

pub use seq_error::record_sequential_error;

use seq_failure::record_quality_failure;

pub use seq_iteration::begin_sequential_iteration;

pub use seq_prompt::sequential_prompt;

pub use seq_response::handle_sequential_response;

use seq_success::finish_verified_story;

use seq_unchecked::finish_unchecked_story;
