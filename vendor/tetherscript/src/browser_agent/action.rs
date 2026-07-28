//! Action result types for agent browser interactions.

use crate::browser_agent::action_checks::ActionabilityReport;

/// Integer viewport rectangle for a resolved element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundingBox {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
}

impl BoundingBox {
    pub(crate) fn visible(self) -> bool {
        self.width > 0 && self.height > 0
    }
}

/// Result metadata for a completed browser action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionReport {
    pub action: String,
    pub locator: String,
    pub bounds: BoundingBox,
    pub actionability: ActionabilityReport,
}

impl ActionReport {
    pub(crate) fn new(
        action: &str,
        locator: String,
        bounds: BoundingBox,
        actionability: ActionabilityReport,
    ) -> Self {
        Self {
            action: action.into(),
            locator,
            bounds,
            actionability,
        }
    }
}
