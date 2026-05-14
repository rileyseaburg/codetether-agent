use super::record::EvidenceRecord;

pub(crate) fn collect(text: &str, out: &mut Vec<EvidenceRecord>) {
    for token in text.split_whitespace() {
        if token.contains("youtu.be/") || token.contains("youtube.com/watch") {
            out.push(EvidenceRecord::new(
                "youtube",
                "real platform upload",
                token,
            ));
        }
        if token.contains("tiktok.com/") {
            out.push(EvidenceRecord::new("tiktok", "real platform upload", token));
        }
        if token.contains("drive.google.com") {
            out.push(EvidenceRecord::new("google", "real platform upload", token));
        }
    }
}
