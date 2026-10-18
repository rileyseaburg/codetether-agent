//! Native relay, provider registry and verification bindings for story tasks.
//!
//! The shared task crate owns lifetime, retry and quality-repair sequencing.
use super::*;
use codetether_ralph_task::Context;
use std::marker::PhantomData;
mod relay;
/// Task service binding for one shared Ralph runtime.
pub struct TaskHost<H: Host>(PhantomData<H>);
#[async_trait::async_trait]
impl<H: Host> codetether_ralph_task::Host for TaskHost<H> {
    type Registry = H::Registry;
    type Agent = H::Agent;
    async fn relay(
        context: &Context<'_, Self>,
        registry: &Arc<Self::Registry>,
        prompt: &str,
    ) -> anyhow::Result<String> {
        relay::run::<H>(context, registry, prompt).await
    }
    async fn check(directory: &Path, checks: &QualityChecks) -> (bool, String) {
        codetether_ralph_core::quality::run_for_repair(directory, checks, &H::Quality::default())
            .await
    }
    async fn verify(directory: &Path, story: &UserStory) -> anyhow::Result<()> {
        H::verify(directory, story).await
    }
}
