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
/// use codetether_agent::a2a::stream::event_id::EventId;
/// use codetether_agent::a2a::stream::resume_request::apply_resume;
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
mod tests {
    use super::EventId;
    use super::apply_resume;

    #[test]
    fn none_returns_buildable_request() {
        let client = reqwest::Client::new();
        let req = apply_resume(client.get("http://localhost/x"), None);
        assert!(req.build().is_ok());
    }

    #[test]
    fn some_sets_header() {
        let client = reqwest::Client::new();
        let id = EventId {
            epoch: "ep".into(),
            seq: 5,
        };
        let req = apply_resume(client.get("http://localhost/x"), Some(&id));
        let built = req.build().unwrap();
        assert_eq!(built.headers().get("Last-Event-ID").unwrap(), "ep.5");
    }
}
