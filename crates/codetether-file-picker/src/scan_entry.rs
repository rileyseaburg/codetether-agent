//! Helpers for converting filesystem paths into file-picker entries.
//!
//! This module classifies a discovered path as either a directory or a file,
//! hides dot-prefixed entries, and appends display metadata used by the file
//! picker UI. Directories are collected separately from files so callers can
//! sort, group, or render them independently.

use std::path::PathBuf;

use super::image::is_image_file;
use super::types::FilePickerEntry;

/// Adds a visible filesystem entry to the appropriate file-picker collection.
///
/// The entry name is derived from the final path component. Dot-prefixed names
/// are ignored so hidden files and directories are not shown in the picker.
/// Directory display names receive a trailing slash, while image files receive
/// an `" [image]"` suffix for quick visual identification.
///
/// # Arguments
///
/// * `path` - Filesystem path to inspect and convert into a [`FilePickerEntry`].
/// * `dirs` - Destination collection for directory entries.
/// * `files` - Destination collection for non-directory file entries.
///
/// # Side Effects
///
/// Appends one entry to either `dirs` or `files` when `path` is visible. No entry
/// is appended when the file name cannot be represented except as `"?"` and that
/// name is not dot-prefixed, or when the derived name starts with `.`.
///
/// # Preconditions
///
/// The path may point to an existing file or directory. If filesystem metadata
/// cannot be read, [`PathBuf::is_dir`] treats the path as non-directory and the
/// entry is added to `files`.
pub fn push_entry(
    path: PathBuf,
    is_dir: bool,
    dirs: &mut Vec<FilePickerEntry>,
    files: &mut Vec<FilePickerEntry>,
) {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("?")
        .to_string();
    if name.starts_with('.') {
        return;
    }
    if is_dir {
        dirs.push(FilePickerEntry {
            path,
            is_dir: true,
            name: format!("{name}/"),
        });
    } else {
        let suffix = if is_image_file(&path) { " [image]" } else { "" };
        files.push(FilePickerEntry {
            path,
            is_dir: false,
            name: format!("{name}{suffix}"),
        });
    }
}
