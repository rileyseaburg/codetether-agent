//! Merge global and project instruction files under a cumulative byte cap.

use std::path::{Path, PathBuf};

pub(super) fn load_all(
    start_dir: &Path,
    max_bytes: usize,
    global_file: Option<(String, PathBuf)>,
) -> Vec<(String, PathBuf)> {
    let mut results = Vec::new();
    let mut remaining = max_bytes;
    add_global(&mut results, &mut remaining, global_file);
    add_project(start_dir, &mut results, &mut remaining);
    results
}

fn add_global(
    results: &mut Vec<(String, PathBuf)>,
    remaining: &mut usize,
    global_file: Option<(String, PathBuf)>,
) {
    if let Some((content, path)) = global_file {
        *remaining = remaining.saturating_sub(content.len());
        results.push((content, path));
    }
}

fn add_project(start_dir: &Path, results: &mut Vec<(String, PathBuf)>, remaining: &mut usize) {
    for dir in super::dirs::root_to_leaf(start_dir) {
        if *remaining == 0 {
            break;
        }
        if let Some((content, path)) = super::read::preferred_instruction_file(&dir, *remaining) {
            *remaining = remaining.saturating_sub(content.len());
            results.push((content, path));
        }
    }
}
