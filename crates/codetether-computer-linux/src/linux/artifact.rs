//! Create private screenshot directories before launching capture utilities.

use std::os::unix::fs::PermissionsExt;

pub(super) fn directory() -> std::io::Result<tempfile::TempDir> {
    tempfile::Builder::new()
        .prefix("codetether-snapshot-")
        .permissions(std::fs::Permissions::from_mode(0o700))
        .tempdir()
}
