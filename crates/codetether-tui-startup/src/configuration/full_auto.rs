//! Full-auto access resolution and final session flag.
use super::Host;
use codetether_config::AccessMode;
/// Resolve the effective access mode, letting `--yolo` force
/// [`AccessMode::Full`]. Pure function — no session mutation.
pub fn effective_access_mode(access_mode: Option<AccessMode>, yolo: bool) -> Option<AccessMode> {
    if yolo {
        Some(AccessMode::Full)
    } else {
        access_mode
    }
}

/// Apply the `--yolo` session policy (auto-apply edits). Call this **after**
/// config so the flag is authoritative.
pub fn apply_session_policy(host: &mut impl Host, yolo: bool) {
    if yolo {
        host.enable_auto_apply();
    }
}
