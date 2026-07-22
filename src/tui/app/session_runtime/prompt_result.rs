//! Convert prompt task results into runtime notices.
//!
//! Prompt execution is usually run as an asynchronous task, so there are two
//! layers of outcome to interpret: whether the task itself panicked, and whether
//! the prompt run returned an application-level error. This module normalizes
//! both outcomes into [`SessionNotice`] values while preserving the associated
//! [`Session`] for the runtime or UI code that receives the notice.

use crate::session::Session;

use super::SessionNotice;

/// Result returned by a prompt execution task.
///
/// The outer [`Result`] reports whether the task completed normally or panicked.
/// The inner [`anyhow::Result`] reports whether the prompt run itself succeeded
/// or returned an error after the task completed normally.
///
/// The three meaningful states are:
///
/// - `Ok(Ok(()))`: the prompt run finished successfully.
/// - `Ok(Err(_))`: the prompt run completed but returned an error.
/// - `Err(_)`: the task panicked and yielded a panic payload.
pub(super) type PromptRunResult = Result<anyhow::Result<()>, Box<dyn std::any::Any + Send>>;

/// Convert a completed prompt task result into a session notice.
///
/// Successful prompt runs become [`SessionNotice::Finished`]. Returned
/// application errors and task panics become [`SessionNotice::Failed`], with the
/// error message converted into displayable text.
///
/// # Arguments
///
/// * `result` - The prompt task outcome to classify.
/// * `session` - The session that was being processed by the prompt task. It is
///   moved into the returned notice.
///
/// # Returns
///
/// A [`SessionNotice`] describing whether the session finished successfully or
/// failed.
///
/// # Side Effects
///
/// This function has no side effects. It does not modify external state, write
/// logs, or persist the session.
pub(super) fn notice(result: PromptRunResult, session: Session) -> SessionNotice {
    match result {
        Ok(Ok(())) => SessionNotice::Finished(session),
        Ok(Err(error)) => SessionNotice::Failed {
            session,
            error: format!("{error:#}"),
        },
        Err(error) => SessionNotice::Failed {
            session,
            error: format!("Provider task panicked: {}", panic_message(&error)),
        },
    }
}

/// Return a human-readable message from a panic payload.
///
/// Tokio and other task runtimes expose panic payloads as boxed [`std::any::Any`]
/// values. Most Rust panics carry either a [`String`] or a string slice, and
/// this helper extracts those common forms without taking ownership of the
/// payload.
///
/// # Arguments
///
/// * `error` - Panic payload captured from a failed prompt task.
///
/// # Returns
///
/// The panic message when the payload is a `String` or `&str`; otherwise,
/// `"unknown panic"`.
///
/// # Preconditions
///
/// Returned messages are borrowed from `error`, so the payload must outlive the
/// returned string slice.
fn panic_message(error: &Box<dyn std::any::Any + Send>) -> &str {
    error
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| error.downcast_ref::<&str>().copied())
        .unwrap_or("unknown panic")
}

#[cfg(test)]
#[path = "prompt_result_tests.rs"]
mod tests;
