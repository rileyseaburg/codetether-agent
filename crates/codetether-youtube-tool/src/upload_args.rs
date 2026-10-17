//! Build the validated upload action parameters.
use super::{
    YouTubeTool,
    params::{Params, UploadVideoParams},
};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl YouTubeTool {
    pub(super) async fn upload_args(&self, p: Params) -> Result<ToolResult> {
        let file_path = match super::upload_fields::file_path(p.file_path) {
            Ok(value) => value,
            Err(error) => return Ok(error),
        };
        let title = match super::upload_fields::title(p.title) {
            Ok(value) => value,
            Err(error) => return Ok(error),
        };
        self.upload_video(&UploadVideoParams {
            file_path,
            title,
            description: p.description,
            tags: p.tags,
            privacy_status: p.privacy_status,
            category_id: p.category_id,
        })
        .await
    }
}
