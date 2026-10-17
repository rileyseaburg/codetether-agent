//! S3 archival adapter for shared bus training records.
//!
//! [BusS3SinkConfig] resolves connection settings; [BusS3Sink] owns the client
//! and subscription. [spawn_bus_s3_sink] initializes the bounded S3 executor
//! and starts archival in a background task.
use anyhow::{Context, Result};
use chrono::Utc;
use codetether_bus_core::training::serialize_envelopes;
use codetether_bus_core::{AgentBus, BusEnvelope};
use codetether_secrets as secrets;
use minio::s3::builders::ObjectContent;
use minio::s3::client::{MinioClient, MinioClientBuilder};
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::types::S3Api;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::{sync::broadcast, task};
use tracing::{debug, error, info, warn};
mod config;
pub use config::BusS3SinkConfig;
mod sink;
pub use sink::BusS3Sink;
mod spawn;
pub use spawn::spawn_bus_s3_sink;
mod key;
use key::build_s3_key;
mod spool;
use spool as s3_spool;
/// Tool catalogs rendered as training system messages.
pub mod s3_tool_catalog_record {
    pub use codetether_bus_core::training::catalog::catalog_text;
}
