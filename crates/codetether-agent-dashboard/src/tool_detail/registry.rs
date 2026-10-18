//! Tool-detail reads through the shared parent-scoped agent view.
use super::Host;
use codetether_agent_trace::LiveTraceSnapshot;
use codetether_agent_view::{AgentSnapshot, Host as ViewHost};
use codetether_provider_core::Message;
use std::marker::PhantomData;

/// Bind a shared agent-view host to dashboard tool-detail reads.
/// # Examples
/// ```
/// # fn example<H: codetether_agent_view::Host>() {
/// let registry = codetether_agent_dashboard::tool_detail::Registry::<H>::new();
/// # let _ = registry;
/// # }
/// ```
pub struct Registry<H>(PhantomData<fn() -> H>);

impl<H> Registry<H> {
    /// Construct a stateless binding to the existing agent-view host.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<H> Default for Registry<H> {
    fn default() -> Self {
        Self::new()
    }
}

impl<H: ViewHost> Host for Registry<H> {
    fn find(&self, name: &str, parent: &str) -> Option<AgentSnapshot> {
        codetether_agent_view::find::<H>(name, parent)
    }
    fn transcript(&self, name: &str, parent: &str) -> Option<Vec<Message>> {
        codetether_agent_view::transcript::<H>(name, parent)
    }
    fn trace(&self, name: &str, parent: &str) -> Option<LiveTraceSnapshot> {
        codetether_agent_view::live_trace::<H>(name, parent)
    }
}
