/// Loads local or remote images for vision-capable providers.
pub use codetether_image_tool as image;
/// Generates and edits images through the OpenAI Images API.
pub mod image_generation;
/// Typed image attachments extracted from tool-owned result metadata.
pub(crate) use codetether_tool_core::images as result_images;
