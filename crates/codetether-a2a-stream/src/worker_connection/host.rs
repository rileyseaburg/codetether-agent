use crate::event_id::EventId;
use std::future::Future;
/// Native services used while consuming the worker's stream.
/// # Implementors
/// Workers supply task dispatch/reconciliation, request metadata, HTTP error
/// formatting and server-state updates.
pub trait Host: crate::dispatch::Host {
    /// Build the stream request with the last committed event ID.
    fn request(&self, last: Option<&EventId>) -> reqwest::RequestBuilder;
    /// Publish that an accepted stream response is connected.
    fn connected(&self) -> impl Future<Output = ()> + Send;
    /// Decode a rejected HTTP response into the existing application error.
    fn connect_error(
        &self,
        response: reqwest::Response,
    ) -> impl Future<Output = anyhow::Error> + Send;
}
