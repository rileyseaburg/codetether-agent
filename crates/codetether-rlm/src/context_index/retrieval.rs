//! Query-aware evidence retrieval.

use super::{ContextIndex, EvidenceRecord, plan, score};

/// Retrieve scored evidence under a token budget.
pub(super) fn retrieve(
    index: &ContextIndex,
    query: &str,
    budget_tokens: usize,
) -> Vec<EvidenceRecord> {
    let plan = plan::build(query, budget_tokens);
    let mut scored: Vec<EvidenceRecord> = index
        .records
        .iter()
        .cloned()
        .map(|mut r| {
            let (score, reason) = score::record(&r, &plan);
            r.score = score;
            r.reason = reason;
            r
        })
        .collect();
    scored.sort_by(|a, b| b.score.total_cmp(&a.score).then(a.id.cmp(&b.id)));
    select(scored, plan.budget_tokens)
}

fn select(records: Vec<EvidenceRecord>, budget_tokens: usize) -> Vec<EvidenceRecord> {
    let mut used = 0;
    let mut out = Vec::new();
    for record in records {
        let remaining = budget_tokens.saturating_sub(used);
        if let Some(fit) = fit(record, remaining) {
            used += tokens(&fit.text);
            out.push(fit);
        }
        if used >= budget_tokens || out.len() >= 24 {
            break;
        }
    }
    out
}

fn fit(mut record: EvidenceRecord, remaining: usize) -> Option<EvidenceRecord> {
    if remaining < 32 {
        return None;
    }
    if tokens(&record.text) <= remaining {
        return Some(record);
    }
    record.text = record.text.chars().take(remaining * 4).collect();
    record.reason.push_str("; degraded to fit budget");
    Some(record)
}

fn tokens(text: &str) -> usize {
    (text.chars().count() / 4).max(1)
}
