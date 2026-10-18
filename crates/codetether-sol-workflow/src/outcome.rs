//! Private workflow result consumed by output rendering.

/// Final state needed by the CLI renderer.
pub(super) struct Outcome {
    pub(super) plan: String,
    pub(super) final_text: String,
    pub(super) session_id: String,
    pub(super) review_rounds: usize,
    pub(super) unresolved_diagnostics: usize,
    pub(super) worker_model: String,
}
