//! Stable ordered suggestions based on names and edit distance.
use super::InvalidTool;
impl InvalidTool {
    pub(super) fn suggestions(&self, requested: &str) -> String {
        if !self.available_tools.is_empty() {
            // Find similar tool names
            let similar: Vec<&String> = self
                .available_tools
                .iter()
                .filter(|t| {
                    t.contains(requested)
                        || requested.contains(t.as_str())
                        || Self::levenshtein(t, requested) <= 3
                })
                .take(3)
                .collect();

            if similar.is_empty() {
                format!("Available tools: {}", self.available_tools.join(", "))
            } else {
                format!(
                    "Did you mean: {}?\n\nAll available tools: {}",
                    similar
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(", "),
                    self.available_tools.join(", ")
                )
            }
        } else {
            "No tool list provided. Check available tools.".to_string()
        }
    }
}
