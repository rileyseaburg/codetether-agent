//! Utilities for cleaning path strings without accessing the filesystem.
//!
//! This module performs purely lexical path normalization: it rewrites path
//! components such as `.` and `..` based only on the text of the path. It does
//! not check whether the path exists, does not canonicalize through symlinks,
//! and does not require any filesystem permissions.

use std::path::{Component, Path, PathBuf};

/// Cleans a path by removing `.` components and applying `..` components
/// lexically.
///
/// The function iterates over [`Path::components`] and builds a new [`PathBuf`]:
///
/// - [`Component::CurDir`] components (`.`) are omitted.
/// - [`Component::ParentDir`] components (`..`) remove the previously emitted
///   component by calling [`PathBuf::pop`].
/// - All other components, including roots, prefixes, and normal path
///   segments, are preserved.
///
/// This is a string-level transformation only. It does not access the
/// filesystem, resolve symlinks, verify that the path exists, or report errors.
/// Because leading `..` components are handled with `PathBuf::pop`, they are
/// discarded when there is no previous component to remove.
///
/// # Parameters
///
/// * `path` - The path to clean.
///
/// # Returns
///
/// A new [`PathBuf`] containing the lexically cleaned path.
///
/// # Examples
///
/// /// use std::path::{Path, PathBuf};
///
/// let cleaned = clean::lexical(Path::new("a/./b/../c"));
/// assert_eq!(cleaned, PathBuf::from("a/c"));
/// ///
/// Leading parent-directory components are not preserved:
///
/// /// use std::path::{Path, PathBuf};
///
/// let cleaned = clean::lexical(Path::new("../a"));
/// assert_eq!(cleaned, PathBuf::from("a"));
pub fn lexical(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            _ => out.push(component.as_os_str()),
        }
    }
    out
}