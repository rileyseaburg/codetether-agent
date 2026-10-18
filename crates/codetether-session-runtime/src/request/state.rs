use std::{path::PathBuf, sync::Arc};

/// Data needed to run one prompt without cloning the session.
///
/// `PromptRequest` owns the session and all prompt inputs required by the
/// runtime executor. Moving the session into the request avoids cloning
/// conversation history while a prompt is processed, then the executor returns
/// the updated session through a runtime notice.
///
/// The `prompt_for_pr` field intentionally preserves the submitted prompt text
/// for worktree pull-request handling, while `prompt` is consumed by the prompt
/// execution path.
/// # Examples
/// ```
/// let request = codetether_session_runtime::request::PromptRequest::new(
///     (), "hello".into(), Vec::<()>::new(), std::sync::Arc::new(()), None, None,
/// );
/// assert_eq!(request.prompt_for_pr, "hello");
/// ```
pub struct PromptRequest<S, R, I> {
    /// Session state that will be mutated by prompt execution and returned in
    /// the final runtime notice.
    pub session: S,
    /// User prompt text to send through the provider and tool execution flow.
    pub prompt: String,
    /// Image attachments that should be included with the prompt request.
    pub images: Vec<I>,
    /// Shared registry of configured model providers available to the prompt.
    pub registry: Arc<R>,
    /// Directory to restore on cancellation or after worktree-aware execution.
    pub original_dir: Option<PathBuf>,
    /// Repository root for runtime-created worktree isolation.
    pub worktree_root: Option<PathBuf>,
    /// Copy of the submitted prompt used when producing pull-request metadata
    /// for worktree results.
    pub prompt_for_pr: String,
}
