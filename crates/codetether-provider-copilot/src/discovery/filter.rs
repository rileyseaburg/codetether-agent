//! Picker and policy admission for models.
use crate::*;
pub(super) fn enabled(model: &CopilotModelInfo) -> bool {
    // Skip models explicitly disabled in the picker
    if model.model_picker_enabled == Some(false) {
        return false;
    }
    // Skip models with a disabled policy state
    if let Some(ref policy) = model.policy
        && policy.state.as_deref() == Some("disabled")
    {
        return false;
    }
    true
}
