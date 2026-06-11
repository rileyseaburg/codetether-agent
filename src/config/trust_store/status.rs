use std::path::PathBuf;

/// Current trust metadata for one canonical workspace.
#[derive(Debug, Clone)]
pub struct ProjectTrustStatus {
    /// Whether a trust record currently exists.
    pub trusted: bool,
    /// Canonical workspace path used for the trust decision.
    pub workspace: PathBuf,
    /// SHA-256 key derived from the canonical workspace path.
    pub key: String,
    /// Filesystem path for the trust record.
    pub record_path: PathBuf,
}
