//! Format unknown tool arguments and corrective suggestions.
use super::InvalidTool;
use serde_json::Value;
impl InvalidTool {
    pub(super) fn message(&self, args: &Value) -> String {
        let requested = args["requested_tool"]
            .as_str()
            .unwrap_or(&self.requested_tool);

        let passed_args = if let Some(obj) = args.get("args") {
            serde_json::to_string_pretty(obj).unwrap_or_else(|_| "{}".to_string())
        } else {
            "{}".to_string()
        };

        let suggestions = self.suggestions(requested);
        let message = format!(
            "Tool '{}' not found.\n\n\
            Arguments received: {}\n\n\
            {}",
            requested, passed_args, suggestions
        );

        message
    }
}
