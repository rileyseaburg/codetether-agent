//! Binary-preview placeholder content.
use std::path::Path;

pub(crate) fn binary_lines(path: &Path, lines: Vec<String>, binary: bool) -> Vec<String> {
    if binary {
        vec![format!("Binary file: {}", path.display())]
    } else {
        lines
    }
}
