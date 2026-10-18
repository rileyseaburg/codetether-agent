//! Validate stored pointers before using them for a session.
use std::path::Path;

pub(super) fn valid_target(id: &str, path: &Path) -> bool {
    path.is_absolute()
        && path.is_file()
        && path.file_name().and_then(|name| name.to_str()) == Some(&format!("{id}.json"))
}
