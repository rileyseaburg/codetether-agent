//! Closed, versioned editor/backend wire contracts.
use serde::{Deserialize, Serialize};

/// One compiler diagnostic, treated as data rather than instructions.
/// # Examples
/// ```
/// let d = codetether_rust_repair::Diagnostic { path: "src/lib.rs".into(),
/// code: "E0432".into(), message: "missing import".into(), range: serde_json::Value::Null };
/// assert_eq!(d.code, "E0432");
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Diagnostic {
    /// Relative POSIX source path.
    pub path: String,
    /// Compiler error identifier.
    pub code: String,
    /// Human-readable compiler diagnostic.
    pub message: String,
    /// Editor range, preserved opaquely.
    pub range: serde_json::Value,
}

/// A caller-bounded file snapshot; permission is independently validated.
/// # Examples
/// ```
/// let f = codetether_rust_repair::FileSnapshot { path: "Cargo.toml".into(), text: "".into(), editable: true };
/// assert!(f.editable);
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileSnapshot {
    /// Relative POSIX path, never read by the backend.
    pub path: String,
    /// Exact current contents, supplied by the editor.
    pub text: String,
    /// Whether the caller permits proposed edits to this file.
    pub editable: bool,
}

/// Complete input snapshot. Use [`crate::validate_request`] before completion.
/// # Examples
/// ```
/// let r = codetether_rust_repair::RepairRequest { schema_version: 1, diagnostics: vec![], files: vec![] };
/// assert!(codetether_rust_repair::validate_request(&r).is_ok());
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepairRequest {
    /// Wire schema, currently exactly 1.
    pub schema_version: u32,
    /// At most 50 diagnostics.
    pub diagnostics: Vec<Diagnostic>,
    /// At most 16 unique snapshots, including read-only instructions.
    pub files: Vec<FileSnapshot>,
}
