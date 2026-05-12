//! Path resolution and spool-directory helpers.
//!
//! Provides the default spool path and endpoint normalization
//! used when constructing the storage layer.
//!
//! # Examples
//!
//! ```ignore
//! let dir = default_spool_dir();
//! assert!(dir.ends_with("traces/pending"));
//! ```

use std::path::PathBuf;

/// Resolve the directory where oracle traces are buffered
/// before upload to remote storage.
///
/// Reads `CODETETHER_ORACLE_SPOOL_DIR` if set; otherwise
/// falls back to `$HOME/.codetether/traces/pending`.
///
/// # Examples
///
/// ```ignore
/// std::env::set_var("CODETETHER_ORACLE_SPOOL_DIR", "/tmp/spool");
/// assert_eq!(default_spool_dir(), PathBuf::from("/tmp/spool"));
/// ```
pub fn default_spool_dir() -> PathBuf {
    if let Ok(path) = std::env::var("CODETETHER_ORACLE_SPOOL_DIR") {
        return PathBuf::from(path);
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".codetether")
        .join("traces")
        .join("pending")
}

/// Ensure an endpoint string starts with a scheme.
///
/// Returns the input unchanged when it already begins with
/// `http://` or `https://`. Otherwise prepends `https://` when
/// `secure` is true, or `http://` when false.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(normalize_endpoint("minio:9000", false), "http://minio:9000");
/// assert_eq!(normalize_endpoint("https://s3.aws", true), "https://s3.aws");
/// ```
pub fn normalize_endpoint(endpoint: &str, secure: bool) -> String {
    let endpoint = endpoint.trim_end_matches('/');
    if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
        endpoint.to_string()
    } else if secure {
        format!("https://{endpoint}")
    } else {
        format!("http://{endpoint}")
    }
}
