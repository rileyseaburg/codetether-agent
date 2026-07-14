//! Provenance-preserving recall evidence formatting.

use super::hit::RecallHit;

pub(crate) fn evidence(hits: &[RecallHit], heading: &str) -> String {
    let body = hits
        .iter()
        .enumerate()
        .map(|(index, hit)| render_hit(index + 1, hit))
        .collect::<Vec<_>>()
        .join("\n\n");
    format!("{heading}\n\n{body}")
}

pub(crate) fn sources(hits: &[RecallHit]) -> Vec<String> {
    let mut sources = Vec::new();
    for hit in hits {
        let label = format!(
            "{} ({})",
            hit.title.as_deref().unwrap_or("<untitled>"),
            hit.session_id
        );
        if !sources.contains(&label) {
            sources.push(label);
        }
    }
    sources
}

fn render_hit(index: usize, hit: &RecallHit) -> String {
    let title = hit.title.as_deref().unwrap_or("<untitled>");
    format!(
        "[{index}] {title} | session={} | turns={}..{} | score={:.3}\n{}",
        hit.session_id,
        hit.start,
        hit.end.saturating_sub(1),
        hit.score,
        hit.excerpt.trim()
    )
}
