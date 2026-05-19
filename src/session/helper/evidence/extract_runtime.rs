use super::record::EvidenceRecord;

pub(crate) fn collect(text: &str, out: &mut Vec<EvidenceRecord>) {
    let lower = text.to_ascii_lowercase();
    if lower.contains("synced") && lower.contains("healthy") {
        out.push(EvidenceRecord::new(
            "argo",
            "live deployment/Argo",
            "healthy-synced",
        ));
    }
    if lower.contains("succeeded") || lower.contains("completed") {
        out.push(EvidenceRecord::new(
            "k8s",
            "live deployment/Argo",
            "pod-succeeded",
        ));
    }
    if lower.contains("cloudflare 502") || lower.contains("http 502") {
        out.push(EvidenceRecord::new("http", "blocked", "502"));
    }
    if lower.contains("failed") || lower.contains("outofsync") || lower.contains("degraded") {
        out.push(EvidenceRecord::new(
            "runtime",
            "blocked",
            "failed-or-degraded",
        ));
    }
}
