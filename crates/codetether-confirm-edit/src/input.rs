//! Parsed single-file replacement and optional confirmation decision.
use super::ToolResult;
use serde_json::Value;
pub(super) struct Input {
    pub path: String,
    pub old_string: String,
    pub new_string: String,
    pub confirm: Option<bool>,
}
impl Input {
    pub fn parse(input: &Value) -> Result<Self, ToolResult> {
        let path = super::path::parse(input)?;
        let old_string = super::old_string::parse(input, &path)?;
        let new_string = super::new_string::parse(input, &path, &old_string)?;
        let confirm = input.get("confirm").and_then(|v| v.as_bool());

        Ok(Self {
            path,
            old_string,
            new_string,
            confirm,
        })
    }
}
