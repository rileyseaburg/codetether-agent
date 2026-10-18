use super::PromptRequest;
use std::{path::PathBuf, sync::Arc};

impl<S, R, I> PromptRequest<S, R, I> {
    /// Build a prompt request from moved session state.
    ///
    /// # Arguments
    ///
    /// * `session` - Session to move into the runtime for prompt execution.
    /// * `prompt` - User prompt text to execute.
    /// * `images` - Image attachments associated with the prompt.
    /// * `registry` - Shared provider registry used by the executor.
    /// * `original_dir` - Directory to restore when cancellation or worktree
    ///   cleanup needs to return to the caller's original location.
    /// * `worktree_root` - Optional worktree root for isolated prompt execution.
    ///
    /// # Returns
    ///
    /// A request containing all data needed to run the prompt. The prompt text
    /// is also copied into `prompt_for_pr` so worktree result handling can refer
    /// to the original user request after execution.
    /// # Examples
    /// ```
    /// let request = codetether_session_runtime::request::PromptRequest::new(
    ///     (), "hello".into(), Vec::<()>::new(), std::sync::Arc::new(()), None, None,
    /// );
    /// assert_eq!(request.prompt, request.prompt_for_pr);
    /// ```
    pub fn new(
        session: S,
        prompt: String,
        images: Vec<I>,
        registry: Arc<R>,
        original_dir: Option<PathBuf>,
        worktree_root: Option<PathBuf>,
    ) -> Self {
        let prompt_for_pr = prompt.clone();
        Self {
            session,
            prompt,
            images,
            registry,
            original_dir,
            worktree_root,
            prompt_for_pr,
        }
    }
}
