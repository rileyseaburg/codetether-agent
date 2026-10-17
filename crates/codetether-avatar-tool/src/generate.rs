//! Send the generate request.
use super::{AvatarTool, params::GenerateParams, voice_api_url};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl AvatarTool {
    pub(super) async fn generate(&self, params: &GenerateParams) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/avatar/generate");

        let form = match super::generate_form::build(params).await? {
            Ok(form) => form,
            Err(error) => return Ok(error),
        };
        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Avatar generate request failed: {e}"))?;

        super::generate_result::read(resp).await
    }
}
