//! construct for image loading.
use super::*;
impl ImageTool {
    /// Construct the stateless image loader.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_image_tool::ImageTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}

impl Default for ImageTool {
    fn default() -> Self {
        Self::new()
    }
}
