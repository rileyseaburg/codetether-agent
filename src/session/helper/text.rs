pub use codetether_context_policy::transcript::{
    extract_text_content, latest_user_text, role_label,
};
use regex::Regex;
use std::path::Path;
use std::sync::OnceLock;

/// Extracts candidate file paths from text that exist in the workspace.
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

/// Truncates a string to max_chars and adds an ellipsis if needed.
pub fn truncate_with_ellipsis(value: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    let mut chars = value.chars();
    let mut output = String::new();
    for _ in 0..max_chars {
        if let Some(c) = chars.next() {
            output.push(c);
        } else {
            break;
        }
    }

    if chars.next().is_some() {
        format!("{output}...")
    } else {
        output
    }
}
