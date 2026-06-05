//! Commands accepted by the TUI session runtime.
//!
//! This module defines the message types used to communicate with the
//! background TUI session runtime. A command either transfers ownership of a
//! prompt request into the runtime, asks the active prompt to stop, or requests
//! runtime shutdown.

use std::{path::PathBuf, sync::Arc};

use crate::provider::ProviderRegistry;
use crate::session::{ImageAttachment, Session};

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
pub(crate) struct PromptRequest {
    /// Session state that will be mutated by prompt execution and returned in
    /// the final runtime notice.
    pub session: Session,
    /// User prompt text to send through the provider and tool execution flow.
    pub prompt: String,
    /// Image attachments that should be included with the prompt request.
    pub images: Vec<ImageAttachment>,
    /// Shared registry of configured model providers available to the prompt.
    pub registry: Arc<ProviderRegistry>,
    /// Directory to restore on cancellation or after worktree-aware execution.
    pub original_dir: Option<PathBuf>,
    /// Repository root for runtime-created worktree isolation.
    pub worktree_root: Option<PathBuf>,
    /// Copy of the submitted prompt used when producing pull-request metadata
    /// for worktree results.
    pub prompt_for_pr: String,
}

impl std::fmt::Debug for PromptRequest {
    /// Formats a prompt request without dumping heavyweight runtime state.
    ///
    /// The debug representation includes the session identifier, prompt text,
    /// and attachment count, but omits the provider registry, directory, and
    /// worktree details to keep logs readable and avoid exposing unnecessary
    /// execution state.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PromptRequest")
            .field("session_id", &self.session.id)
            .field("prompt", &self.prompt)
            .field("image_count", &self.images.len())
            .finish_non_exhaustive()
    }
}

impl PromptRequest {
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
    /// * `worktree` - Optional worktree state for isolated prompt execution.
    ///
    /// # Returns
    ///
    /// A request containing all data needed to run the prompt. The prompt text
    /// is also copied into `prompt_for_pr` so worktree result handling can refer
    /// to the original user request after execution.
    pub(crate) fn new(
        session: Session,
        prompt: String,
        images: Vec<ImageAttachment>,
        registry: Arc<ProviderRegistry>,
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

/// Runtime control messages.
///
/// `SessionCommand` is sent over the runtime command channel by
/// [`super::handle::TuiSessionHandle`]. Prompt submission transfers ownership of
/// a boxed [`PromptRequest`] to keep channel messages reasonably sized, while
/// cancellation and shutdown commands are lightweight control signals.
pub(crate) enum SessionCommand {
    /// Submit a prompt request for execution by the runtime.
    SubmitPrompt(Box<PromptRequest>),
    /// Request cancellation of the currently running prompt, if any.
    CancelCurrent,
    /// Request runtime shutdown after notifying any active prompt.
    Shutdown,
}
