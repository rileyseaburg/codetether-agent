//! A sourced execution constraint for a delegated task.

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ConstraintEntry {
    pub(super) source: &'static str,
    pub(super) text: String,
}

impl ConstraintEntry {
    pub(super) fn new(source: &'static str, text: &str) -> Self {
        Self {
            source,
            text: text.trim().to_string(),
        }
    }
}
