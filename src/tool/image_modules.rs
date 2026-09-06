/// Loads local or remote images for vision-capable providers.
pub mod image;
/// Generates and edits images through the OpenAI Images API.
pub mod image_generation;
/// Typed image attachments extracted from tool-owned result metadata.
pub(crate) mod result_images;