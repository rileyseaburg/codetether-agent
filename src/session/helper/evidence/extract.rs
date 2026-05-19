use super::record::EvidenceRecord;

pub(crate) fn from_text(text: &str) -> Vec<EvidenceRecord> {
    let mut out = Vec::new();
    super::extract_platform::collect(text, &mut out);
    super::extract_runtime::collect(text, &mut out);
    super::extract_artifact::collect(text, &mut out);
    super::extract_db::collect(text, &mut out);
    out
}
