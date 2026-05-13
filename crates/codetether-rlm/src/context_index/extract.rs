//! Build typed context indexes from raw content.

use super::{
    ContextIndex, EvidenceKind, EvidenceRecord, cache, fallback, symbol, text, text_records,
};

/// Build an index for one content source.
pub(super) fn build(source: &str, content: &str) -> ContextIndex {
    let lines: Vec<&str> = content.lines().collect();
    let mut records = Vec::new();
    let mut symbols = Vec::new();
    collect_line_records(source, &lines, &mut records, &mut symbols);
    text_records::collect(source, &lines, &mut records);
    if records.is_empty() {
        records.push(fallback::record(source, content));
    }
    ContextIndex {
        source: source.into(),
        hash: cache::hash(source, content),
        symbols,
        records,
    }
}

fn collect_line_records(
    source: &str,
    lines: &[&str],
    records: &mut Vec<EvidenceRecord>,
    symbols: &mut Vec<String>,
) {
    for (idx, line) in lines.iter().enumerate() {
        let kind = symbol::kind(line);
        if kind == EvidenceKind::Text {
            continue;
        }
        let symbol = symbol::name(line);
        if let Some(ref name) = symbol {
            symbols.push(name.clone());
        }
        let (start, end, body) = text::window(lines, idx, 6);
        records.push(EvidenceRecord::new(
            records.len(),
            source,
            kind,
            (start, end),
            body,
            symbol.into_iter().collect(),
        ));
    }
}
