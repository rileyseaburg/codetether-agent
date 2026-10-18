//! Signed request adapter for shared extended-heartbeat payloads.
//!
//! [`send_extended_task_heartbeat`] prepares a task's progress snapshot using
//! the shared HTTP helpers, sends it with the configured worker proof and
//! optional bearer authentication, and interprets the server response.
//! Callers supply progress containing a non-empty string `task_id` and the
//! requested lease extension in seconds.
#[path = "extended_heartbeat_request.rs"]
mod request;
use anyhow::Result;
use reqwest::Client;

/// Report task progress and request a lease extension through the worker API.
///
/// # Arguments
///
/// * `client` — HTTP client used to send the request.
/// * `server` — Server base URL, to which
///   `/v1/worker/tasks/heartbeat-extended` is appended directly.
/// * `token` — Optional bearer token attached to the request.
/// * `worker_id` — Worker identifier included in the payload and used when
///   applying the configured request proof.
/// * `progress` — Progress snapshot containing a non-empty string `task_id`.
///   The entire value becomes the payload's `checkpoint`. An unsigned integer
///   `checkpoints_reached` supplies the checkpoint sequence, defaulting to and
///   clamped to at least `1`; a string `current_checkpoint` supplies the optional
///   status message.
/// * `lease_seconds` — Requested lease extension in seconds, forwarded without
///   local range validation.
///
/// # Returns
///
/// Returns `Ok(())` for a successful HTTP status. This does not guarantee that
/// the server extended an active lease: a response with `"success": false` is
/// logged at debug level and still returns `Ok(())`. Missing or invalid JSON in
/// an otherwise successful response is also tolerated.
///
/// # Side Effects
///
/// Sends an HTTP POST with the progress snapshot and lease-extension request,
/// applying the configured worker proof and optional bearer token. The server
/// may update task progress and the task's lease.
///
/// # Errors
///
/// Returns an error if `progress` lacks a non-empty string `task_id`, request
/// proof preparation fails, the HTTP request cannot be sent, or the server
/// returns a non-success HTTP status. For a rejected response, the error
/// includes the status and response text when that text can be read.
pub(super) async fn send_extended_task_heartbeat(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    progress: serde_json::Value,
    lease_seconds: u64,
) -> Result<()> {
    let (task_id, payload) = codetether_a2a_http::worker::prepare_extended_heartbeat(
        worker_id,
        progress,
        lease_seconds,
    )?;
    let response = request::send(client, server, token, worker_id, &task_id, &payload).await?;
    codetether_a2a_http::worker::check_extended_heartbeat(response, &task_id).await
}