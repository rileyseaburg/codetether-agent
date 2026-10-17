//! Failures reported by archival configuration and delivery.

use thiserror::Error;

/// HTTP, filesystem, configuration, local upload, or remote service failures.
///
/// # Examples
/// ```
/// use codetether_event_stream::s3_sink::S3SinkError;
/// let error = S3SinkError::MissingConfig("bucket".into());
/// assert!(matches!(error, S3SinkError::MissingConfig(_)));
/// ```
#[derive(Error, Debug)]
pub enum S3SinkError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Missing S3 configuration: {0}")]
    MissingConfig(String),

    #[error("Upload failed: {0}")]
    UploadFailed(String),

    #[error("S3 returned error: {0}")]
    S3Error(String),
}
