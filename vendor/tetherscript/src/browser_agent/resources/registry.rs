//! Storage for deterministic external browser resources.

use super::{images, BrowserResource, ImageResourceMetadata, ResourceKind, ResourcePayload};

/// Page-local registry of host-provided external resources.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResourceRegistry {
    entries: Vec<BrowserResource>,
}

impl ResourceRegistry {
    /// Register or replace a text resource.
    pub fn register_text(
        &mut self,
        url: impl Into<String>,
        kind: ResourceKind,
        text: impl Into<String>,
    ) {
        self.upsert(BrowserResource::text(url, kind, text));
    }

    /// Register or replace image bytes.
    pub fn register_image(&mut self, url: impl Into<String>, bytes: Vec<u8>) {
        self.upsert(BrowserResource::bytes(url, bytes));
    }

    /// Return resources in deterministic insertion order.
    pub fn entries(&self) -> &[BrowserResource] {
        &self.entries
    }

    /// Return metadata for all registered images.
    pub fn image_metadata(&self) -> Vec<ImageResourceMetadata> {
        self.entries.iter().filter_map(images::metadata).collect()
    }

    pub(crate) fn text(&self, kind: ResourceKind, url: &str) -> Option<&str> {
        self.entries.iter().find_map(|entry| match &entry.payload {
            ResourcePayload::Text(text) if entry.kind == kind && entry.url == url => {
                Some(text.as_str())
            }
            _ => None,
        })
    }

    pub(crate) fn has(&self, kind: ResourceKind, url: &str) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.kind == kind && entry.url == url)
    }

    fn upsert(&mut self, resource: BrowserResource) {
        if let Some(existing) = self
            .entries
            .iter_mut()
            .find(|entry| entry.kind == resource.kind && entry.url == resource.url)
        {
            *existing = resource;
        } else {
            self.entries.push(resource);
        }
    }
}
