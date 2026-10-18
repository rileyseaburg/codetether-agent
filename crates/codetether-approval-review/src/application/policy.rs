//! Native runtime and session-goal lookup required by reviewer dispatch.
use crate::runtime::ReviewRuntime;
use codetether_application_providers::ProviderRegistry;
use codetether_application_registry::ToolRegistry;
use std::future::Future;
/// Application identities and remaining native reviewer services.
/// # Implementors
/// Select existing provider/tool registries and agent policy, then supply the
/// installed runtime and session journal lookup used by background reviews.
pub trait Application: Send + Sync + 'static {
    /// Tool catalog and runtime authorization used by the reviewer.
    type Tools: codetether_application_registry::Application;
    /// Provider factory and cache identity used for model resolution.
    type Providers: codetether_application_providers::Application;
    /// Agent loop using this application's tool registry and runtime policy.
    type Agent: codetether_swarm_agent::Host<Registry = ToolRegistry<Self::Tools>>;
    /// Read the installed reviewer runtime only when enabled.
    fn enabled() -> Option<ReviewRuntime<ProviderRegistry<Self::Providers>>>;
    /// Read the current goal block for an optional session identifier.
    fn goal(session_id: Option<&str>) -> impl Future<Output = Option<String>> + Send;
}
