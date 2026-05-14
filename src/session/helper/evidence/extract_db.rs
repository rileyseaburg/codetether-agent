use super::record::EvidenceRecord;

pub(crate) fn collect(text: &str, out: &mut Vec<EvidenceRecord>) {
    let lower = text.to_ascii_lowercase();
    if lower.contains("migration") && (lower.contains("applied") || lower.contains("succeeded")) {
        out.push(EvidenceRecord::new("db-migration", "live DB", "applied"));
    }
    if lower.contains("rows") || lower.contains("distinct") || lower.contains("row count") {
        out.push(EvidenceRecord::new("db-query", "live DB", "row-evidence"));
    }
}
