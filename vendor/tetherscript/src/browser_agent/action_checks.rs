//! Actionability report values.

/// Boolean actionability checks used before an agent action is dispatched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionabilityReport {
    pub visible: bool,
    pub enabled: bool,
    pub editable: bool,
    pub receives_pointer: bool,
}

impl ActionabilityReport {
    pub(crate) fn new(editable: bool) -> Self {
        Self {
            visible: true,
            enabled: true,
            editable,
            receives_pointer: true,
        }
    }
}
