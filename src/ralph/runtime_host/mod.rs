//! Native providers, tools, sessions and command policy for shared Ralph state.
//!
//! The runtime crate owns state and service integration; these modules bind
//! concrete application types and preserve the existing execution methods.
use crate::provider::ProviderRegistry;
use codetether_ralph_core::UserStory;
use std::path::Path;
mod agent;
mod relay;
/// Native service binding exposed through the public RalphLoop type alias.
pub struct Native;
#[async_trait::async_trait]
impl codetether_ralph_runtime::Host for Native {
    type Registry = ProviderRegistry;
    type Agent = agent::Native;
    type Relay = relay::Native;
    type Git = crate::ralph::git_policy::Native;
    type Quality = codetether_ralph_agent::shell::Quality<crate::tool::registry_lifecycle::Native>;
    async fn verify(root: &Path, story: &UserStory) -> anyhow::Result<()> {
        crate::ralph::verification::run_story_verification(root, story).await
    }
}
