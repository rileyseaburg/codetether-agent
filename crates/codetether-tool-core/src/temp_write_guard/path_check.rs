//! Decide whether a filesystem path targets a hard-denied temp directory.

use super::roots::{DENIED_WINDOWS_SEGMENTS, all};
use std::path::{Component, Path, PathBuf};

/// Returns the denied temp root when `path` writes into one.
///
/// The path is lexically normalized first so `/tmp/../tmp/x` and
/// `./tmp/../../tmp/x` cannot slip past a prefix comparison. Relative paths
/// are resolved against the current directory, which keeps a workspace file
/// named `tmp/` from being confused with the system temp root.
pub(super) fn denied_root(path: &Path) -> Option<PathBuf> {
    let absolute = absolutize(path);
    if windows_temp(&absolute) {
        return Some(absolute);
    }
    all().into_iter().find(|root| absolute.starts_with(root))
}

fn absolutize(path: &Path) -> PathBuf {
    let joined = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("/"))
            .join(path)
    };
    normalize(&joined)
}

fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

fn windows_temp(path: &Path) -> bool {
    let text = path
        .to_string_lossy()
        .to_ascii_lowercase()
        .replace('/', "\\");
    DENIED_WINDOWS_SEGMENTS
        .iter()
        .any(|segment| text.contains(segment))
}
