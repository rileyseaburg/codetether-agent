//! S3-compatible configuration, request signing, and archival uploads.
//!
//! [`S3SinkConfig`] accepts explicit settings or environment configuration.
//! [`S3Sink`] uploads bytes or local files; [`ArchivedEventFile`] records results.

mod archive;
mod config;
mod constructors;
mod digest;
mod endpoint;
mod environment;
mod error;
mod signing;
mod sink;
mod upload;
mod upload_file;
mod upload_headers;
mod upload_response;
pub use archive::ArchivedEventFile;
pub use config::S3SinkConfig;
pub use error::S3SinkError;
pub use sink::S3Sink;
#[cfg(test)]
mod contract;
#[cfg(test)]
mod tests;
