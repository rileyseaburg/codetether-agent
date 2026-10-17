//! execute for PRD construction and validation.
use super::*;
impl PrdTool {
    pub(super) async fn dispatch(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;
        match p.action.as_str() {
            "analyze" => self.analyze(p).await,
            "generate" => self.generate(p).await,
            "validate" => self.validate(p).await,
            "save" => self.save(p).await,
            _ => Ok(ToolResult::error(format!(
                "Unknown action: {}. Use analyze/generate/validate/save",
                p.action
            ))),
        }
    }
}
