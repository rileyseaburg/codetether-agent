//! Executes prepared TetherScript plugin requests with timeout enforcement.
//!
//! This module forms the async boundary around the synchronous TetherScript
//! runner. It moves plugin evaluation onto Tokio's blocking thread pool so the
//! async runtime remains responsive while script parsing and hook execution run.
//! The module also normalizes runner completion, task join failures, and timeout
//! expiry into the task-layer result type used by the `tetherscript_plugin` tool.

use anyhow::Result;
use std::time::Duration;

use super::request::TetherScriptRun;
use super::run_result::TetherScriptRunResult;
use crate::tool::tetherscript::join::join_error;
use crate::tool::tetherscript::runner::{self, BrowserGrant, ComputerGrant};

/// Runs the already-loaded source, hook name, JSON arguments, and optional
/// browser capability described by `request`.
///
/// The synchronous interpreter or plugin host is invoked through
/// [`tokio::task::spawn_blocking`] because TetherScript execution can perform
/// CPU-bound work and must not occupy an async executor worker. The configured
/// timeout is enforced around the blocking task's join handle; when the timeout
/// expires, the caller receives [`TetherScriptRunResult::Timeout`] with the
/// number of seconds that was allowed.
///
/// # Arguments
///
/// * `request` - Prepared TetherScript execution parameters. The request is
///   expected to contain loaded source text, the hook to call, serialized
///   arguments, a timeout in seconds, and any browser grant restrictions.
///
/// # Returns
///
/// Returns [`TetherScriptRunResult::Finished`] when the blocking runner task
/// completes before the timeout, or [`TetherScriptRunResult::Timeout`] when the
/// timeout elapses first.
///
/// # Errors
///
/// Returns an [`anyhow::Error`] if the blocking task cannot be joined, for
/// example because the worker panicked. Join errors are converted with
/// [`join_error`] so callers receive the same error shape as other tool
/// execution failures.
///
/// # Side Effects
///
/// Executes caller-supplied TetherScript. When `request.grant_browser` is set,
/// the script may receive a browser capability constrained by the supplied
/// origins and scopes.
/// Execute a TetherScript run on a blocking thread with a timeout.
pub async fn run(request: TetherScriptRun) -> Result<TetherScriptRunResult> {
    let timeout = Duration::from_secs(request.timeout_secs);
    let task = tokio::task::spawn_blocking(move || {
        runner::run(
            request.source_name,
            request.source,
            request.hook,
            request.args,
            BrowserGrant {
                endpoint: request.grant_browser,
                origins: request.browser_origin,
                scopes: request.browser_scope,
            },
            ComputerGrant {
                enabled: request.grant_computer,
                origins: request.computer_origin,
                scopes: request.computer_scope,
            },
        )
    });
    match tokio::time::timeout(timeout, task).await {
        Ok(joined) => Ok(TetherScriptRunResult::Finished(joined.map_err(join_error)?)),
        Err(_) => Ok(TetherScriptRunResult::Timeout(timeout.as_secs())),
    }
}
