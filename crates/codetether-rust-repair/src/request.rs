//! Bounded request decoding and editable-snapshot validation.
use crate::{RepairRequest, paths};
use anyhow::{Result, ensure};
use std::collections::HashSet;

/// Maximum UTF-8 request size, including the JSON envelope.
pub const MAX_REQUEST_BYTES: usize = 512 * 1024;

/// Decode exactly one bounded JSON request and validate its edit scope.
/// # Arguments
/// * `bytes` - Complete stdin payload.
/// # Returns
/// A validated snapshot without reading any referenced files.
/// # Errors
/// Rejects oversized, malformed, unknown-field or unsafe requests.
/// # Examples
/// ```
/// assert!(codetether_rust_repair::parse_request(br#"{"schema_version":1,"diagnostics":[],"files":[]}"#).is_ok());
/// ```
pub fn parse_request(bytes: &[u8]) -> Result<RepairRequest> {
    ensure!(bytes.len() <= MAX_REQUEST_BYTES, "request exceeds 512 KiB");
    let request = serde_json::from_slice(bytes)?;
    validate_request(&request)?;
    Ok(request)
}

/// Validate schema, sizes, paths and caller-requested edit scope.
/// # Arguments
/// * `request` - In-memory snapshot to inspect.
/// # Returns
/// Unit for a valid bounded request.
/// # Errors
/// Rejects duplicate files and edits outside diagnostic files or Cargo.toml.
/// # Examples
/// ```
/// let r = codetether_rust_repair::RepairRequest { schema_version:1, diagnostics:vec![], files:vec![] };
/// assert!(codetether_rust_repair::validate_request(&r).is_ok());
/// ```
pub fn validate_request(request: &RepairRequest) -> Result<()> {
    ensure!(
        request.schema_version == 1,
        "unsupported request schema_version"
    );
    ensure!(
        request.files.len() <= 16 && request.diagnostics.len() <= 50,
        "snapshot count limit exceeded"
    );
    ensure!(
        serde_json::to_vec(request)?.len() <= MAX_REQUEST_BYTES,
        "request exceeds 512 KiB"
    );
    for diagnostic in &request.diagnostics {
        paths::validate_path(&diagnostic.path)?;
    }
    let mut seen = HashSet::new();
    for file in &request.files {
        paths::validate_path(&file.path)?;
        ensure!(
            seen.insert(&file.path),
            "duplicate snapshot path: {}",
            file.path
        );
        if file.editable {
            ensure!(
                !paths::forbidden(&file.path),
                "protected editable path: {}",
                file.path
            );
            ensure!(
                file.path.rsplit('/').next() == Some("Cargo.toml")
                    || request.diagnostics.iter().any(|d| d.path == file.path),
                "editable file is outside diagnostic/manifest scope: {}",
                file.path
            );
        }
    }
    Ok(())
}
