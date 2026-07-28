//! Download record data types.

const COMPACT_BODY_LIMIT: usize = 4096;

/// Stable status for a deterministic download.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DownloadStatus {
    /// The host recorded all currently known download metadata.
    Completed,
    /// The download was canceled before completion.
    Canceled,
    /// The download failed with a deterministic reason.
    Failed(String),
}

/// In-memory metadata for one page download.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DownloadRecord {
    /// Requested download URL.
    pub url: String,
    /// Browser-suggested filename.
    pub suggested_filename: String,
    /// Best-known MIME type.
    pub mime: String,
    /// Total byte length known to the host.
    pub byte_len: usize,
    /// Compact body copy when the payload is small enough.
    pub body: Option<Vec<u8>>,
    /// Download completion state.
    pub status: DownloadStatus,
}

impl DownloadRecord {
    /// Build a completed download record.
    pub fn completed(
        url: impl Into<String>,
        filename: impl Into<String>,
        mime: impl Into<String>,
        body: impl Into<Vec<u8>>,
    ) -> Self {
        let body = body.into();
        let byte_len = body.len();
        let body = (byte_len <= COMPACT_BODY_LIMIT).then_some(body);
        Self {
            url: url.into(),
            suggested_filename: filename.into(),
            mime: mime.into(),
            byte_len,
            body,
            status: DownloadStatus::Completed,
        }
    }
}
