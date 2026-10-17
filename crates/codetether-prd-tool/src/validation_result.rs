//! validation result for PRD construction and validation.
use super::*;

pub(super) struct ValidationResult {
    pub(super) is_valid: bool,
    pub(super) errors: Vec<String>,
    pub(super) warnings: Vec<String>,
}

impl ValidationResult {
    pub(super) fn summary(&self) -> String {
        let mut lines = Vec::new();

        if !self.errors.is_empty() {
            lines.push("## Errors".to_string());
            for err in &self.errors {
                lines.push(format!("- ❌ {err}"));
            }
        }

        if !self.warnings.is_empty() {
            if !lines.is_empty() {
                lines.push(String::new());
            }
            lines.push("## Warnings".to_string());
            for warn in &self.warnings {
                lines.push(format!("- ⚠️ {warn}"));
            }
        }

        if self.is_valid && self.warnings.is_empty() {
            lines.push("✅ All checks passed".to_string());
        }

        lines.join("\n")
    }
}
