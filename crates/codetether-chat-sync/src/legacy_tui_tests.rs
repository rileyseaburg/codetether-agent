//! Existing TUI regression cases moved beside their implementation.
use crate::config_types::{minio_fallback_endpoint, normalize_minio_endpoint};

#[test]
fn normalize_minio_endpoint_strips_login_path() {
    assert_eq!(
        normalize_minio_endpoint("http://192.168.50.223:9001/login"),
        "http://192.168.50.223:9001"
    );
}

#[test]
fn normalize_minio_endpoint_adds_default_scheme() {
    assert_eq!(
        normalize_minio_endpoint("192.168.50.223:9000"),
        "http://192.168.50.223:9000"
    );
}

#[test]
fn fallback_endpoint_maps_console_port_to_s3_port() {
    assert_eq!(
        minio_fallback_endpoint("http://192.168.50.223:9001"),
        Some("http://192.168.50.223:9000".to_string())
    );
    assert_eq!(minio_fallback_endpoint("http://192.168.50.223:9000"), None);
}
