use anyhow::Result;
use std::collections::HashMap;

use super::AstMatch;

pub(crate) fn build(
    query: &tree_sitter::Query,
    match_: &tree_sitter::QueryMatch<'_, '_>,
    source: &[u8],
) -> Result<AstMatch> {
    let mut captures = HashMap::new();
    let mut first = None;
    for capture in match_.captures {
        let node = capture.node;
        let name = query.capture_names()[capture.index as usize].to_string();
        let text = node.utf8_text(source)?.to_string();
        first.get_or_insert((text.clone(), node.start_position()));
        captures.insert(name, text);
    }
    let (text, point) = first.unwrap_or_default();
    Ok(AstMatch {
        line: point.row + 1,
        column: point.column + 1,
        captures,
        text,
    })
}
