//! extract identifier after keyword for codebase indexing.

use super::*;

pub(in crate::indexer) fn extract_identifier_after_keyword(
    line: &str,
    keyword: &str,
) -> Option<String> {
    let prefix = format!("{keyword} ");
    let rest = line.strip_prefix(&prefix)?;
    sanitize_identifier(rest)
}
