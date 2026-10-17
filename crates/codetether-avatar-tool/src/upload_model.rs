//! Send the upload_model request.
use super::{AvatarTool, params::UploadModelParams, voice_api_url};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl AvatarTool {
    pub(super) async fn upload_model(&self, params: &UploadModelParams) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/avatar/upload-model");

        let form = match super::upload_model_form::build(params).await? {
            Ok(form) => form,
            Err(error) => return Ok(error),
        };
        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Upload model failed: {e}"))?;

        super::upload_model_result::read(resp, params).await
    }
}
