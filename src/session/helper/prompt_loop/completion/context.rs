//! State retained across provider attempts within one prompt step.

use super::super::Runner;
use crate::provider::Message;
use crate::session::{Bucket, DerivePolicy, DerivedContext};
use anyhow::Result;

/// Context and counters retained while retrying one provider step.
pub(super) struct Attempt {
    /// Derived message history sent to the provider.
    pub derived: DerivedContext,
    /// Optional proactive language-server context message.
    pub proactive: Option<Message>,
    /// Delegation bucket used for routing feedback.
    pub bucket: Bucket,
    /// Baseline context derivation policy for the step.
    pub policy: DerivePolicy,
    /// Provider attempts made with the current target.
    pub count: usize,
    /// Transient upstream failures observed during the step.
    pub upstream_retries: u8,
}

impl Attempt {
    /// Derives the initial request context for `step`.
    ///
    /// # Errors
    ///
    /// Returns an error when context derivation fails.
    pub async fn new(runner: &Runner<'_>, step: usize) -> Result<Self> {
        let policy = crate::session::effective_policy(runner.session, &runner.model.model_id);
        let derived = super::derived::derive(runner, policy, None).await?;
        let proactive = super::derived::lsp(runner, step).await;
        let bucket = crate::session::bucket_for_messages(runner.session.history());
        Ok(Self {
            derived,
            proactive,
            bucket,
            policy,
            count: 0,
            upstream_retries: 0,
        })
    }
}
