//! Constructors for report and violation data.
use crate::{GuardFile, GuardReport, GuardViolation};

impl GuardReport {
    /// Create an empty-violation report for the supplied file metrics.
    pub fn new(files: Vec<GuardFile>) -> Self {
        Self {
            files,
            violations: Vec::new(),
        }
    }
}

impl GuardViolation {
    /// Create a path-specific violation from its rendered explanation.
    pub fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }
}
