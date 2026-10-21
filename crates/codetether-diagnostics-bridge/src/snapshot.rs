//! Versioned editor-facing wire format, with no conversation fields.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Language-server diagnostics indexed by canonical local file URI.
pub type Diagnostics = HashMap<String, Vec<lsp_types::Diagnostic>>;

/// Complete producer state, replaced atomically rather than appended.
/// # Examples
/// ```
/// use codetether_diagnostics_bridge::{Snapshot, Diagnostics};
/// let snapshot = Snapshot { schema_version: 1, pid: std::process::id(),
///     workspace_uri: "file:///workspace/".into(), server: "rust-analyzer".into(),
///     updated_at_ms: 1, diagnostics: Diagnostics::new() };
/// assert_eq!(snapshot.schema_version, 1);
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    /// Wire-format version, currently 1.
    pub schema_version: u8,
    /// Producer PID; consumers must also verify heartbeat freshness.
    pub pid: u32,
    /// Canonical percent-encoded directory URI with trailing slash.
    pub workspace_uri: String,
    /// Server command identity, without arguments.
    pub server: String,
    /// Wall-clock heartbeat in Unix epoch milliseconds.
    pub updated_at_ms: u64,
    /// Complete filtered diagnostic cache, including empty publications.
    pub diagnostics: Diagnostics,
}
