//! Preserve the application's public verifier type while owning its implementation.
/// Declare the existing two-field verifier with native goal-review execution.
/// # Usage
/// The tool application parameter selects the normal native swarm and workspace services.
/// ```text
/// codetether_native_goal_tools::declare_verifier!(pub struct LlmVerifier; ToolApplication);
/// ```
#[macro_export]
macro_rules! declare_verifier {
    ($visibility:vis struct $name:ident; $(,)? $tools:ty) => {
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
$visibility struct $name {
    /// Model the worker ran on; used when no verifier model is selected.
    pub worker_model: Option<String>,
    /// Directory the verifier inspects.
    pub workspace: ::std::path::PathBuf,
}

#[$crate::__private::async_trait]
impl $crate::__private::VerifierAgent for $name {
    async fn review(&self, request: &$crate::__private::VerificationRequest) -> $crate::__private::Result<String> {
        <Self as $crate::__private::VerifierAgent>::review_with_identity(self, request).await.report
    }

    async fn review_with_identity(&self, request: &$crate::__private::VerificationRequest) -> $crate::__private::ReviewExecution {
        $crate::verification::review::<$tools>(
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

impl $crate::VerifierFactory for $name {
    fn create(worker_model: Option<String>, workspace: ::std::path::PathBuf) -> Self {
        Self {
            worker_model,
            workspace,
        }
    }
}

    };
}
