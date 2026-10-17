//! Behavior contracts shared with the original implementation.
use super::*;
mod broadcast;
mod encoding_constructor;
mod encoding_wire;
mod serialization;
mod server;
mod signing;
mod stub;
mod upload_failure;
mod upload_success;
fn config(endpoint: Option<String>, stub_mode: bool) -> S3SinkConfig {
    S3SinkConfig {
        bucket: "bucket".into(),
        prefix: "events/".into(),
        endpoint,
        access_key: Some("fixture-access".into()),
        secret_key: Some("fixture-secret".into()),
        region: "us-west-2".into(),
        stub_mode,
    }
}
