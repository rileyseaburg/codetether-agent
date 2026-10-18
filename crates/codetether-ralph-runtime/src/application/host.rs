//! Shared runtime choices and policy-gated story verification.
use super::ApplicationHost;
use crate::Host;
use codetether_application_registry::{Application as ToolApplication, ToolRegistry};
use codetether_ralph_agent::shell::{GitPolicy, Quality, Verification};
use codetether_ralph_core::UserStory;
use std::path::Path;
#[async_trait::async_trait]
impl<P, T, A, R> Host for ApplicationHost<P, T, A, R>
where
    P: Send + Sync + 'static,
    T: ToolApplication,
    A: codetether_ralph_agent::Host<Registry = ToolRegistry<T>>,
    R: codetether_ralph_relay::Host<Registry = P>,
{
    type Registry = P;
    type Agent = A;
    type Relay = R;
    type Git = GitPolicy<T>;
    type Quality = Quality<T>;
    async fn verify(root: &Path, story: &UserStory) -> anyhow::Result<()> {
        codetether_ralph_core::verification::run_story_verification(
            root,
            story,
            &Verification::<T>::new(),
        )
        .await
    }
}
