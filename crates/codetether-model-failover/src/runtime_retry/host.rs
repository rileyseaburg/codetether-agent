use super::State;
use std::future::Future;
/// Native session access and runtime submission for scheduled retries.
/// # Implementors
/// UI adapters retain session storage, views, provider registries and runtime handles.
pub trait Host {
    /// Borrowed provider registry handle.
    type Provider;
    /// Borrow retry and UI state.
    fn state(&mut self) -> State<'_>;
    /// Outer None means unavailable session; inner None means no selected model.
    fn current_model(&self) -> Option<Option<&str>>;
    /// Replace and save the model on an available session, ignoring save failure.
    fn set_model(&mut self, model: String) -> impl Future<Output = ()>;
    /// Refresh the retained session view after restoration.
    fn refresh_view(&mut self);
    /// Start timing a retry request.
    fn begin_timing(&mut self);
    /// Submit the retry prompt using the borrowed configured provider.
    fn submit(&mut self, prompt: &str, provider: &Self::Provider) -> impl Future<Output = ()>;
}
