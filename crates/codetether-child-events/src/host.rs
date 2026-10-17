//! Native observation, event projection and approval effects.
use crate::Event;
/// Map native events after observing them and handle their approval payloads.
/// # Implementors
/// Applications retain native trace storage and approval handling here.
pub trait Host {
    /// Native event carried by the channel.
    type SourceEvent: Send;
    /// Native approval request payload.
    type Approval;
    /// Observe each received event before projecting or accumulating it.
    fn observe(agent: &str, event: &Self::SourceEvent);
    /// Consume the native event and return its accumulator effect.
    fn project(event: Self::SourceEvent) -> Event<Self::Approval>;
    /// Handle one approval request using the application's existing decision policy.
    fn approve(request: &Self::Approval);
}
