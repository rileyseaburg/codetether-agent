//! Validate upload_model arguments and dispatch its request.
use super::{AvatarTool, params::*};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl AvatarTool {
    pub(super) async fn upload_model_args(&self, p: Params) -> Result<ToolResult> {
        let name = match super::required::upload_model_name::parse(p.name) {
            Ok(value) => value,
            Err(error) => return Ok(error),
        };
        let file_path = match super::required::upload_model_file_path::parse(p.file_path) {
            Ok(value) => value,
            Err(error) => return Ok(error),
        };
        self.upload_model(&UploadModelParams { name, file_path })
            .await
    }
}
