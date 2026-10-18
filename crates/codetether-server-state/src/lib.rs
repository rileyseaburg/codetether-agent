//! HTTP request state sharing the services initialized by server startup.
//!
//! [AppState] retains the startup [Parts] without reconstructing services. Its
//! focused modules implement service-access contracts used by API routes.
//! The application supplies its concrete cognition runtime as `R`.
use codetether_server_runtime::Parts;
mod audit;
mod bus;
mod cognition;
mod construction;
mod discovery;
mod k8s;
mod plugin;
mod recovery;
mod router;
mod task;
mod task_ingest;
mod voice;
/// Services shared by HTTP handlers, with an application-defined runtime.
/// # Examples
/// ```no_run
/// # fn example<R>(parts: codetether_server_runtime::Parts<R>) {
/// let state = codetether_server_state::AppState { inner: parts };
/// let copy = state.clone();
/// assert!(std::sync::Arc::ptr_eq(&state.inner.bus, &copy.inner.bus));
/// # }
/// ```
pub struct AppState<R> {
    /// Initialized services retained with their existing ownership.
    pub inner: Parts<R>,
}
impl<R> Clone for AppState<R> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
