//! Extract unique file references that exist under a workspace root.
use regex::Regex;
use std::path::Path;
use std::sync::OnceLock;

/// Extracts candidate file paths from text that exist in the workspace.
/// Apply the established extract candidate file paths rule.
/// # Arguments
/// Receives the message, tool or workspace inputs named in the signature.
/// # Returns
/// The decision or matching paths without executing tools.
/// # Examples
/// ```
/// assert!(codetether_build_policy::extract_candidate_file_paths("no paths", std::path::Path::new("."), 1).is_empty());
/// ```
pub fn extract_candidate_file_paths(text: &str, cwd: &Path, max_files: usize) -> Vec<String> {
    static FILE_PATH_RE: OnceLock<Regex> = OnceLock::new();
    let re = FILE_PATH_RE
        .get_or_init(|| Regex::new(r"(?P<path>[a-zA-Z0-9_\-\./]+\.[a-zA-Z0-9]+)").unwrap());

    let mut out = Vec::new();
    for cap in re.captures_iter(text) {
        let Some(path) = cap.name("path").map(|m| m.as_str()) else {
            continue;
        };
        let path_str = path.to_string();
        if path_str.is_empty() || out.iter().any(|p: &String| p == &path_str) {
            continue;
        }
        if cwd.join(&path_str).exists() {
            out.push(path_str);
        }
        if out.len() >= max_files {
            break;
        }
    }
    out
}
