//! Bind approval reviewer execution to the native application runtime.
use super::{prompt::ReviewSubject, verdict::ReviewVerdict};
use crate::{config::ReviewConfig, provider::ProviderRegistry};
use std::path::PathBuf;
/// Review a proposed change with the configured read-only reviewer.
pub async fn review(
    config: &ReviewConfig,
    registry: &ProviderRegistry,
    session_model: Option<&str>,
    workspace: PathBuf,
    subject: ReviewSubject,
) -> ReviewVerdict {
    codetether_approval_review::review::<super::host::Native>(
        config,
        registry,
        session_model,
        workspace,
        subject,
    )
    .await
}
