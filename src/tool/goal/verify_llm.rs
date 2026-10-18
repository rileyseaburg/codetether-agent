//! Production verifier: a separate LLM agent loop over the workspace.

use super::{ReviewExecution, VerificationRequest, VerifierAgent};
use async_trait::async_trait;
use std::path::PathBuf;

/// Verifier that runs a second LLM in its own agent thread.
///
/// It gets read and verification tools (it can read files and run tests and
/// git) but cannot edit or commit, so it can only judge the work, not do it.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tool::goal::verify::LlmVerifier;
/// use std::path::PathBuf;
///
/// let verifier = LlmVerifier {
///     worker_model: Some("openai/gpt-5".into()),
///     workspace: PathBuf::from("/repo"),
/// };
/// assert_eq!(verifier.workspace, PathBuf::from("/repo"));
/// ```
#[derive(Clone, Debug)]
pub struct LlmVerifier {
    /// Model the worker ran on; used when no verifier model is selected.
    pub worker_model: Option<String>,
    /// Directory the verifier inspects.
    pub workspace: PathBuf,
}

#[async_trait]
impl VerifierAgent for LlmVerifier {
    async fn review(&self, request: &VerificationRequest) -> anyhow::Result<String> {
        self.review_with_identity(request).await.report
    }

    async fn review_with_identity(&self, request: &VerificationRequest) -> ReviewExecution {
        codetether_goal_verification::runtime::review::<super::runtime_host::Native>(
            self.worker_model.as_deref(),
            &self.workspace,
            request,
        )
        .await
    }

    async fn identity(&self) -> String {
        "llm-verifier".into()
    }
}

impl codetether_native_goal_tools::VerifierFactory for LlmVerifier {
    fn create(worker_model: Option<String>, workspace: PathBuf) -> Self {
        Self {
            worker_model,
            workspace,
        }
    }
}
