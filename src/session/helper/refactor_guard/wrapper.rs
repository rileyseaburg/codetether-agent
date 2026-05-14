use std::path::Path;

pub fn target(path: &str, text: &str) -> Option<String> {
    let stem = Path::new(path).file_stem()?.to_string_lossy();
    let expected = format!("{stem}Root");
    if !text.contains(&expected) {
        return None;
    }
    if text.contains(&format!("<{expected}")) || text.contains(&format!("{expected}(")) {
        Some(expected)
    } else {
        None
    }
}

pub fn stem(path: &str) -> Option<String> {
    Path::new(path)
        .file_stem()
        .map(|stem| stem.to_string_lossy().to_string())
}
