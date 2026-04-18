//! Chat sync: MinIO/S3 archive worker.
//!
//! Periodically batches chat events and uploads to S3-compatible storage.

pub mod archive_reader;
pub mod batch_upload;
pub mod config;
pub mod config_types;
pub mod env_helpers;
pub mod minio_client;
pub mod s3_key;
pub mod types;
pub mod worker;

pub use config::parse_chat_sync_config;
pub use types::ChatSyncUiEvent;
pub use worker::run_chat_sync_worker;
