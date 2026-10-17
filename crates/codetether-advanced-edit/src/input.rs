//! Parse required strings in their original validation order.
use super::ToolResult;
use serde_json::Value;
pub(super) struct Input {
    pub file_path: String,
    pub old_string: String,
    pub new_string: String,
    pub replace_all: bool,
}
impl Input {
    pub fn parse(params: &Value) -> Result<Self, ToolResult> {
        let file_path = super::file_path::parse(params)?;
        let old_string = super::old_string::parse(params, &file_path)?;
        let new_string = super::new_string::parse(params, &file_path, &old_string)?;
        let replace_all = params
            .get("replaceAll")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        Ok(Self {
            file_path,
            old_string,
            new_string,
            replace_all,
        })
    }
}
