use super::State;
use std::future::Future;
/// Native UI effects and provider task handoff.
/// # Implementors
/// UI adapters retain timing, worker bridge, session slot and provider services.
pub trait Host {
    /// Provider handle borrowed from the caller's configured registry.
    type Provider;
    /// Image attachment passed through to the provider task.
    type Image;
    /// Whether answer review currently holds the session goal.
    fn held(&self) -> bool;
    /// Clear captured input after the hold gate accepts submission.
    fn clear_input(&mut self);
    /// Announce processing to the native worker bridge.
    fn processing_started(&mut self) -> impl Future<Output = ()>;
    /// Start native request timing.
    fn begin_timing(&mut self);
    /// Borrow fields updated when a new prompt begins dispatch.
    fn state(&mut self) -> State<'_>;
    /// Scroll the chat view to the newest content.
    fn scroll(&mut self);
    /// Hand the prompt and images to the configured provider runtime.
    fn spawn(
        &mut self,
        provider: &Self::Provider,
        prompt: &str,
        images: Vec<Self::Image>,
    ) -> impl Future<Output = ()>;
    /// Report missing provider configuration through the native UI.
    fn no_provider(&mut self) -> impl Future<Output = ()>;
}
