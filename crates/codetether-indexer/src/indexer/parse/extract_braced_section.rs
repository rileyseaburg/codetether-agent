//! extract braced section for codebase indexing.

pub(in crate::indexer) fn extract_braced_section(line: &str) -> Option<String> {
    let start = line.find('{')?;
    let end = line[start + 1..].find('}')?;
    Some(line[start + 1..start + 1 + end].to_string())
}
