//! Decode the target path from a unified-diff file header.

pub(super) fn file(line: &str) -> Option<String> {
    let path = line.strip_prefix("+++ ")?;
    let path = path.strip_prefix("b/").unwrap_or(path);
    Some(path.split('\t').next().unwrap_or(path).to_string())
}
