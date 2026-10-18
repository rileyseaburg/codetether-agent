//! Native relay, provider registry and verification bindings for story tasks.
//!
//! The shared task crate owns lifetime, retry and quality-repair sequencing.
use super::*;
use codetether_ralph_task::{Context, Host};
mod relay;
/// Native execution binding exposed through the stage host associated type.
/// # Examples
/// ```rust,no_run
/// type Binding = <codetether_agent::ralph::RalphLoop as codetether_ralph_stage::Host>::Task;
/// fn assert_host<H: codetether_ralph_task::Host>() {}
/// assert_host::<Binding>();
/// ```
pub struct Native;
#[async_trait::async_trait]
impl Host for Native {
    type Registry = ProviderRegistry;
    type Agent = super::super::model::host::Native;
    async fn relay(
        context: &Context<'_, Self>,
        registry: &Arc<Self::Registry>,
        prompt: &str,
    ) -> anyhow::Result<String> {
        relay::run(context, registry, prompt).await
    }
    async fn check(directory: &Path, checks: &QualityChecks) -> (bool, String) {
        codetether_ralph_core::quality::run_for_repair(directory, checks, &quality::QualityHost)
            .await
    }
    async fn verify(directory: &Path, story: &UserStory) -> anyhow::Result<()> {
        run_story_verification(directory, story).await
    }
}
