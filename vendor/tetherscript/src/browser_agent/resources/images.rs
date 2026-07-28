//! Image resource metadata extraction.

use super::{BrowserResource, ImageResourceMetadata, ResourceKind, ResourcePayload};

pub(crate) fn metadata(resource: &BrowserResource) -> Option<ImageResourceMetadata> {
    match &resource.payload {
        ResourcePayload::Bytes(bytes) if resource.kind == ResourceKind::Image => {
            Some(ImageResourceMetadata {
                url: resource.url.clone(),
                byte_len: bytes.len(),
            })
        }
        _ => None,
    }
}
