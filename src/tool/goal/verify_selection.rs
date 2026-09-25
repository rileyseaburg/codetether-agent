//! Process-wide verifier model chosen interactively, e.g. from the TUI.

use std::sync::RwLock;

static SELECTED: RwLock<Option<String>> = RwLock::new(None);

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
    let value = model
        .map(str::trim)
        .filter(|m| !m.is_empty())
        .map(str::to_string);
    if let Ok(mut slot) = SELECTED.write() {
        *slot = value;
    }
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
    SELECTED.read().ok().and_then(|slot| slot.clone())
}
