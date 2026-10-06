//! Process-wide verifier model chosen interactively, e.g. from the TUI.

use parking_lot::RwLock;
use std::sync::{Arc, LazyLock};

static SELECTED: LazyLock<Arc<VerifierSelection>> = LazyLock::new(Arc::default);

/// Shared harness-owned selection; HTTP tests can use an isolated instance.
#[derive(Debug, Default)]
pub(crate) struct VerifierSelection(RwLock<Option<String>>);

impl VerifierSelection {
    /// Read the selected runtime override, without resolving a provider.
    pub(crate) fn get(&self) -> Option<String> {
        self.0.read().clone()
    }
    /// Replace the override; clearing it restores environment/config fallback.
    pub(crate) fn set(&self, model: Option<&str>) {
        *self.0.write() = model
            .map(str::trim)
            .filter(|m| !m.is_empty())
            .map(str::to_string);
    }
}

/// Return the same process-wide store used by the TUI and verifier harness.
pub(crate) fn shared_selection() -> Arc<VerifierSelection> {
    Arc::clone(&SELECTED)
}

/// Set the verifier model for every later goal verification in this process.
///
/// Passing `None` or a blank value clears the selection so the environment
/// override, worker model, and configured default apply again.
///
/// # Arguments
///
/// * `model` — Model identifier such as `openai/gpt-5`, or `None` to clear.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tool::goal::verify::{selected_verifier_model, set_verifier_model};
///
/// set_verifier_model(Some("anthropic/claude-opus-4"));
/// assert_eq!(selected_verifier_model().as_deref(), Some("anthropic/claude-opus-4"));
/// set_verifier_model(None);
/// assert_eq!(selected_verifier_model(), None);
/// ```
pub fn set_verifier_model(model: Option<&str>) {
    SELECTED.set(model);
}

/// Return the interactively selected verifier model, if any.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tool::goal::verify::{selected_verifier_model, set_verifier_model};
///
/// set_verifier_model(Some("openai/gpt-5"));
/// assert_eq!(selected_verifier_model().as_deref(), Some("openai/gpt-5"));
/// # set_verifier_model(None);
/// ```
pub fn selected_verifier_model() -> Option<String> {
    SELECTED.get()
}
