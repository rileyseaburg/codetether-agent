//! Verification entrypoint selecting the native tool application.
use codetether_ralph_core::UserStory;
pub async fn run_story_verification(
    root: &std::path::Path,
    story: &UserStory,
) -> anyhow::Result<()> {
    codetether_native_ralph::verification::run_story_verification::<
        crate::tool::registry_lifecycle::Native,
    >(root, story)
    .await
}
