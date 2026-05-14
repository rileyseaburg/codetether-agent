use super::record::EvidenceRecord;

pub(crate) fn collect(text: &str, out: &mut Vec<EvidenceRecord>) {
    for token in text.split_whitespace() {
        let clean = token.trim_matches(|c: char| ",.;:()[]".contains(c));
        if clean.ends_with(".webm") || clean.ends_with(".mp4") || clean.ends_with(".zip") {
            out.push(EvidenceRecord::new("artifact", "focused CI-like", clean));
        }
        if clean.contains("playwright-report") || clean.contains("test-results") {
            out.push(EvidenceRecord::new("artifact", "focused CI-like", clean));
        }
    }
}
