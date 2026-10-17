//! tool for image loading.
use super::*;
#[async_trait]
impl Tool for ImageTool {
    fn id(&self) -> &str {
        "image"
    }

    fn name(&self) -> &str {
        "Image Loader"
    }

    fn description(&self) -> &str {
        "Load an image from a file path or URL and encode it for use with vision-capable models. \
         Supports PNG, JPEG, GIF, WebP, SVG, BMP, and TIFF formats. \
         Returns a base64-encoded data URL that can be used in image content parts."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        self.load(args).await
    }
}
