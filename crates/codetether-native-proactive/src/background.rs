//! Native provider policy and typed events for background RLM work.
/// Background worker binding retaining the application's event session type.
/// # Examples
/// ```
/// let _: Option<codetether_native_proactive::Background<()>> = None;
/// ```
pub type Background<S> = codetether_application_providers::background::Background<
    codetether_native_providers::registry::application::Native,
    S,
>;
/// Progress callback receiving events for the application session type.
/// # Examples
/// ```
/// let _: Option<codetether_native_proactive::Notify<()>> = None;
/// ```
pub type Notify<S> = codetether_application_providers::background::Notify<S>;
