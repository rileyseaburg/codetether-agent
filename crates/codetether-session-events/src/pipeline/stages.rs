//! Ordered transformations that may consume an event.
/// Frontend event stages; returning None stops subsequent stages.
/// # Implementors
/// Frontends adapt existing lifecycle, tool, text and telemetry handlers.
#[allow(async_fn_in_trait)]
pub trait Stages {
    /// Owned event passed between stages.
    type Event;
    /// Apply session lifecycle effects.
    async fn lifecycle(&mut self, event: Self::Event) -> Option<Self::Event>;
    /// Apply play-break effects.
    fn interlude(&mut self, event: Self::Event) -> Option<Self::Event>;
    /// Apply retry effects.
    fn retry(&mut self, event: Self::Event) -> Option<Self::Event>;
    /// Apply tool-call effects.
    async fn tools(&mut self, event: Self::Event) -> Option<Self::Event>;
    /// Apply streamed text effects.
    fn text(&mut self, event: Self::Event) -> Option<Self::Event>;
    /// Apply usage effects.
    fn usage(&mut self, event: Self::Event) -> Option<Self::Event>;
    /// Apply errors after the other event categories.
    async fn errors(&mut self, event: Self::Event) -> Option<Self::Event>;
}
