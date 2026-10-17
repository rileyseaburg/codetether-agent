//! Inject the `Last-Event-ID` header on stream reconnect.
//!
//! See `docs/transport-phase1-wire-contract.md` section 5. Applying the cursor
//! is purely additive: with no cursor the request is byte-identical to a cold
//! connect, preserving backward compatibility with servers that ignore it.

use reqwest::RequestBuilder;

use super::event_id::EventId;

/// Add `Last-Event-ID: <epoch>.<seq>` to `request` when a cursor exists.
///
/// Returns the builder unchanged when `last` is `None` (cold connect).
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_a2a_stream::event_id::EventId;
/// use codetether_a2a_stream::resume_request::apply_resume;
///
/// let client = reqwest::Client::new();
/// let req = client.get("http://localhost/v1/worker/tasks/stream");
/// let id = EventId { epoch: "ep".into(), seq: 12 };
/// let _req = apply_resume(req, Some(&id));
/// ```
pub fn apply_resume(request: RequestBuilder, last: Option<&EventId>) -> RequestBuilder {
    match last {
        Some(id) => request.header("Last-Event-ID", id.format()),
        None => request,
    }
}
#[cfg(test)]
#[path = "resume_request_tests.rs"]
mod tests;
