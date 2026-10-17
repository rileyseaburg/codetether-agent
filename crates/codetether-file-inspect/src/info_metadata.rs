//! info metadata for filesystem inspection.
use super::*;
pub(super) fn describe(path: &str, metadata: &std::fs::Metadata) -> Vec<String> {
    let mut info = Vec::new();

    // Basic info
    info.push(format!("Path: {}", path));
    info.push(format!(
        "Size: {} ({} bytes)",
        format_size(metadata.len()),
        metadata.len()
    ));

    // File type
    let file_type = if metadata.is_dir() {
        "directory"
    } else if metadata.is_symlink() {
        "symlink"
    } else {
        "file"
    };
    info.push(format!("Type: {}", file_type));

    // Permissions (Unix)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode();
        info.push(format!("Permissions: {:o}", mode & 0o777));
    }

    // Modified time
    if let Ok(modified) = metadata.modified()
        && let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH)
    {
        let secs = duration.as_secs();
        info.push(format!("Modified: {} seconds since epoch", secs));
    }

    info
}
