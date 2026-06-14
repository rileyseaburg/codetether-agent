//! Partition-name → filesystem-path mapping for [`FileDurableLog`].

use std::path::{Path, PathBuf};

/// Map a partition name to a sanitized `<dir>/<partition>.jsonl` path.
///
/// Partition names come from task/PRD ids; sanitize to keep them as single
/// path components and avoid traversal.
pub(super) fn partition_path(dir: &Path, partition: &str) -> PathBuf {
    dir.join(format!("{}.jsonl", sanitize(partition)))
}

fn sanitize(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect();
    if cleaned.is_empty() {
        "_".to_string()
    } else {
        cleaned
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitizes_traversal_and_separators() {
        let p = partition_path(Path::new("/tmp/log"), "../task/1");
        assert_eq!(p, Path::new("/tmp/log/___task_1.jsonl"));
    }

    #[test]
    fn empty_partition_falls_back() {
        let p = partition_path(Path::new("/tmp/log"), "");
        assert_eq!(p, Path::new("/tmp/log/_.jsonl"));
    }
}
