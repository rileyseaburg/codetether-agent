//! Types for host-provided external browser resources.

/// Kind of deterministic external resource stored for a page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceKind {
    /// JavaScript source used by `<script src>`.
    Script,
    /// CSS source used by stylesheet links.
    Stylesheet,
    /// Image bytes referenced by image elements.
    Image,
    /// Source-map text referenced by bundled JavaScript.
    SourceMap,
}

/// Payload held for a deterministic external resource.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourcePayload {
    /// Text payload for JavaScript or CSS.
    Text(String),
    /// Raw byte payload for images.
    Bytes(Vec<u8>),
}

/// One registered deterministic resource.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserResource {
    /// URL used to match document references.
    pub url: String,
    /// Resource category.
    pub kind: ResourceKind,
    /// Stored resource payload.
    pub payload: ResourcePayload,
}

/// Lightweight image resource metadata for agent inspection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageResourceMetadata {
    /// URL used to register the image bytes.
    pub url: String,
    /// Number of bytes stored for this image.
    pub byte_len: usize,
}

impl BrowserResource {
    pub(crate) fn text(
        url: impl Into<String>,
        kind: ResourceKind,
        text: impl Into<String>,
    ) -> Self {
        Self {
            url: url.into(),
            kind,
            payload: ResourcePayload::Text(text.into()),
        }
    }

    pub(crate) fn bytes(url: impl Into<String>, bytes: Vec<u8>) -> Self {
        Self {
            url: url.into(),
            kind: ResourceKind::Image,
            payload: ResourcePayload::Bytes(bytes),
        }
    }
}
