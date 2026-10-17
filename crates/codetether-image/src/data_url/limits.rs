//! Shared payload limits and accepted image MIME types.
pub(crate) const MAX_IMAGE_DECODED_BYTES: usize = 10 * 1024 * 1024;
pub(crate) const MAX_BASE64_PAYLOAD_CHARS: usize = (MAX_IMAGE_DECODED_BYTES + 2) / 3 * 4;
pub(super) const IMAGE_MIME_TYPES: &[&str] = &[
    "image/png",
    "image/jpeg",
    "image/gif",
    "image/webp",
    "image/bmp",
    "image/svg+xml",
];
