use super::{args::ImagegenArgs, client::ImagesClient};
use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::{collections::VecDeque, sync::Mutex};

/// Generates or edits an image with OpenAI's Images API.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tool::image_generation::ImageGenerationTool;
/// let tool = ImageGenerationTool::new();
/// ```
pub struct ImageGenerationTool {
    pub(super) client: ImagesClient,
    pub(super) recent: Mutex<VecDeque<String>>,
}

impl ImageGenerationTool {
    /// Creates an image tool using the configured OpenAI endpoint and credentials.
    pub fn new() -> Self {
        Self {
            client: ImagesClient::new(),
            recent: Mutex::new(VecDeque::new()),
        }
    }
}

impl Default for ImageGenerationTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for ImageGenerationTool {
    fn id(&self) -> &str {
        "image_gen"
    }
    fn name(&self) -> &str {
        "Image Generation"
    }
    fn description(&self) -> &str {
        super::DESCRIPTION
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let args: ImagegenArgs = serde_json::from_value(args)?;
        match self.generate(args).await {
            Ok(result) => Ok(result),
            Err(error) => Ok(ToolResult::error(error.to_string())),
        }
    }
}
